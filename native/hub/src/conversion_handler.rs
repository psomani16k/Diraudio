use crate::{
    encoder_decoder::{
        encoders::mp3_encoder::mp3::Mp3Encoder, raw_audio_data::raw_audio_data::RawAudioData,
    },
    messages::{
        dart_signal::{Mp3Config, TargetFormat},
        rust_signal::{MessageType, ProgressUpdate},
    },
    AppState,
};
use rinf::debug_print;
use std::{fs, path::Path, sync::Arc, usize};
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct ConversionInstructions {
    pub(crate) copy_unrecognised_files: bool,
    pub(crate) src_path: String,
    pub(crate) dest_path: String,
    pub(crate) no_of_threads: i32,
    pub(crate) target_format: TargetFormat,
    pub(crate) mp3_config: Mp3Config,
}

pub(crate) async fn handle_conversion(
    conversion_details: ConversionInstructions,
    app_state: Arc<Mutex<AppState>>,
) {
    // 1. traverse the source directory and make a list of all the files
    // 2. spawn threads with access to this list of files

    // get all the files in the source directory
    let mut files: Vec<String> = Vec::new();
    let src_path = conversion_details.src_path;
    let files = traverse_directory(&src_path, &mut files, src_path.len());
    // for file in files {
    //     println!("{}", file);
    // }
    let files = Arc::new(Mutex::new(files));
    for i in 0..conversion_details.no_of_threads {}
}

fn traverse_directory<'a>(
    src: &'a String,
    list_of_files: &'a mut Vec<String>,
    src_path_length: usize,
) -> &'a mut Vec<String> {
    let paths = fs::read_dir(src).unwrap();
    for path in paths {
        let entry = path.unwrap();
        let metadata = entry.metadata().unwrap();
        if metadata.is_dir() {
            traverse_directory(
                &entry.path().to_str().unwrap().to_owned(),
                list_of_files,
                src_path_length.clone(),
            );
        } else if metadata.is_file() {
            let path = entry.path();
            let path = path.to_str().unwrap().chars();
            let path: String = path.skip(src_path_length).collect();
            list_of_files.push(path);
        }
    }
    return list_of_files;
}

async fn process_till_empty(
    instruction: &ConversionInstructions,
    thread_no: i32,
    files: Arc<Mutex<&mut Vec<String>>>,
    app_state: Arc<Mutex<AppState>>,
) {
    loop {
        {
            let state = app_state.lock().await;
            if let AppState::DoNothing = *state {
                return;
            }
        }
        let path_option = {
            let mut list_of_files = files.lock().await;
            let path = (*list_of_files).pop();
            path
        };
        match path_option {
            Some(path) => handle_file(instruction, path, thread_no),
            None => {
                return;
            }
        }
    }
}

fn handle_file(instruction: &ConversionInstructions, file_path: String, thread: i32) {
    match decide_file_action(&file_path) {
        FileAction::Copy => {
            debug_print!(
                "Copying {} to {}{}",
                file_path,
                instruction.dest_path,
                file_path
            );
            match std::fs::copy(
                instruction.src_path.clone() + &file_path,
                instruction.dest_path.clone() + &file_path,
            ) {
                Ok(_) => {
                    ProgressUpdate {
                        handling_thread: thread,
                        msg: format!(
                            "Copying {} to {}{}",
                            file_path, instruction.dest_path, file_path
                        ),
                        message_type: MessageType::Success.into(),
                    }
                    .send_signal_to_dart();
                }
                Err(_) => {
                    ProgressUpdate {
                        handling_thread: thread,
                        msg: format!(
                            "Failed to copying {} to {}{}",
                            file_path, instruction.dest_path, file_path
                        ),
                        message_type: MessageType::Fail.into(),
                    }
                    .send_signal_to_dart();
                }
            };
        }
        FileAction::Convert => {
            let src_file_path = instruction.src_path.clone() + &file_path;
            let raw_audio = RawAudioData::new_from_path(Path::new(&src_file_path)).unwrap();
            let output_data = match instruction.target_format {
                TargetFormat::Mp3 => raw_audio.encode_to_mp3(
                    instruction.mp3_config.quality(),
                    instruction.mp3_config.bitrate(),
                ),
            };
        }
    }
}

fn decide_file_action(file_path: &String) -> FileAction {
    if file_path.ends_with(".flac") {
        return FileAction::Convert;
    }
    return FileAction::Copy;
}

enum FileAction {
    Copy,
    Convert,
}
