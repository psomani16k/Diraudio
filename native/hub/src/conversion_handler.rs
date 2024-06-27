use crate::{
    encoder_decoder::{
        encoders::mp3::mp3::Mp3Encoder, raw_audio_data::raw_audio_data::RawAudioData,
    },
    messages::{
        dart_signal::{Mp3Config, TargetFormat},
        rust_signal::{MessageType, ProgressUpdate, TotalNumberOfFilesFound},
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
    // Traverse the source directory and make a list of all the files
    // Get all the files in the source directory
    let mut files: Vec<String> = Vec::new();
    let src_path = conversion_details.src_path.clone();
    let files = traverse_directory(&src_path, &mut files, src_path.len()).unwrap();
    TotalNumberOfFilesFound {
        number: files.len() as i32,
        files_found: true,
    }
    .send_signal_to_dart();
    let files = Arc::new(Mutex::new(files));

    let mut handles = Vec::new();
    for i in 0..conversion_details.no_of_threads {
        let conversion_details_clone = conversion_details.clone();
        let files_clone = Arc::clone(&files);
        let app_state_clone = Arc::clone(&app_state);

        let handle = tokio::task::spawn_blocking(move || {
            debug_print!("Spawning thread {}", i);
            tokio::runtime::Handle::current().block_on(async {
                process_files_till_empty(conversion_details_clone, i, files_clone, app_state_clone)
                    .await
            });
        });

        handles.push(handle);
    }

    for handle in handles {
        if let Err(e) = handle.await {
            debug_print!("Error in thread: {:?}", e);
        }
    }

    ProgressUpdate {
        msg: "Conversion Finished".to_string(),
        handling_thread: 0,
        message_type: MessageType::ConversionFinish.into(),
    }
    .send_signal_to_dart();
}

pub fn traverse_directory(
    src: &String,
    list_of_files: &mut Vec<String>,
    src_path_length: usize,
) -> Result<Vec<String>, String> {
    let paths = match fs::read_dir(src) {
        Ok(paths) => paths,
        Err(err) => {
            return Err(err.to_string());
        }
    };
    for path in paths {
        let entry = path.unwrap();
        let metadata = entry.metadata().unwrap();
        if metadata.is_dir() {
            match traverse_directory(
                &entry.path().to_str().unwrap().to_owned(),
                list_of_files,
                src_path_length,
            ) {
                Ok(_) => {}
                Err(err) => {
                    return Err(err);
                }
            };
        } else if metadata.is_file() {
            let path = entry.path();
            let path = path.to_str().unwrap().chars();
            let path: String = path.skip(src_path_length).collect();
            list_of_files.push(path);
        }
    }
    // for i in list_of_files.clone() {
    //     println!("{}", i)
    // }
    Ok(list_of_files.clone())
}

async fn process_files_till_empty(
    instruction: ConversionInstructions,
    thread_no: i32,
    files: Arc<Mutex<Vec<String>>>,
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
            list_of_files.pop()
        };
        match path_option {
            Some(path) => {
                debug_print!("thread {} handling {}", thread_no, path);
                handle_file(&instruction, path, thread_no);
            }
            None => {
                ProgressUpdate {
                    msg: "No more files to convert".to_string(),
                    handling_thread: thread_no,
                    message_type: MessageType::FileFinish.into(),
                }
                .send_signal_to_dart();
                return;
            }
        }
    }
}

fn handle_file(instruction: &ConversionInstructions, file_path: String, thread: i32) {
    match decide_file_action(&file_path) {
        // If the file isn't of a supported audio format then it will be copied
        FileAction::Copy => {
            if !instruction.copy_unrecognised_files {
                // debug_print!("thread {} did not copy file {}", thread, file_path);
                return;
            }
            debug_print!(
                "Copying {} to {}{}",
                file_path,
                instruction.dest_path,
                file_path
            );
            let target_path = instruction.dest_path.clone() + &file_path;

            let directory_path = get_target_directory(instruction.dest_path.clone(), &file_path);
            fs::create_dir_all(directory_path).unwrap();
            match std::fs::copy(instruction.src_path.clone() + &file_path, target_path) {
                Ok(_) => {
                    debug_print!(
                        "Copied {} to {}{}",
                        file_path,
                        instruction.dest_path,
                        file_path
                    );
                    ProgressUpdate {
                        handling_thread: thread,
                        msg: format!(
                            "Copied {} to {}{}",
                            file_path, instruction.dest_path, file_path
                        ),
                        message_type: MessageType::Success.into(),
                    }
                    .send_signal_to_dart();
                }
                Err(_) => {
                    debug_print!(
                        "Failed to copy {} to {}{}",
                        file_path,
                        instruction.dest_path,
                        file_path
                    );
                    ProgressUpdate {
                        handling_thread: thread,
                        msg: format!(
                            "Failed to copy {} to {}{}",
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
            let raw_audio = match RawAudioData::new_from_path(Path::new(&src_file_path)) {
                Ok(data) => data,
                Err(_) => {
                    debug_print!(
                        "Failed to decode file at {}. Skipping this file.",
                        file_path
                    );
                    ProgressUpdate {
                        handling_thread: thread,
                        message_type: MessageType::Fail.into(),
                        msg: format!(
                            "Failed to decode file at {}. Skipping this file.",
                            file_path
                        ),
                    };
                    return;
                }
            };
            let encoded_audio = match instruction.target_format {
                TargetFormat::Mp3 => raw_audio.encode_to_mp3(
                    instruction.mp3_config.quality(),
                    instruction.mp3_config.bitrate(),
                ),
                TargetFormat::Opus => todo!(),
            };
            match encoded_audio {
                Ok(output_audio) => {
                    let write_path = instruction.dest_path.clone() + &file_path;
                    let write_path = Path::new(&write_path);
                    let write_path = write_path.with_extension("mp3");
                    let directory_path =
                        get_target_directory(instruction.dest_path.clone(), &file_path);
                    fs::create_dir_all(directory_path).unwrap();
                    fs::write(&write_path, output_audio).unwrap();
                    debug_print!(
                        "Converted {} to {}",
                        file_path,
                        write_path.as_path().to_string_lossy()
                    );
                    ProgressUpdate {
                        handling_thread: thread,
                        message_type: MessageType::Success.into(),
                        msg: format!(
                            "Converted {} to {}",
                            file_path,
                            write_path.as_path().to_string_lossy()
                        ),
                    }
                    .send_signal_to_dart();
                }
                Err(_) => {
                    debug_print!(
                        "Failed to encode file at {}. Skipping this file.",
                        file_path
                    );
                    ProgressUpdate {
                        handling_thread: thread,
                        message_type: MessageType::Fail.into(),
                        msg: format!(
                            "Failed to encode file at {}. Skipping this file.",
                            file_path
                        ),
                    }
                    .send_signal_to_dart();
                }
            }
        }
    }
}

fn decide_file_action(file_path: &String) -> FileAction {
    if file_path.ends_with(".flac") {
        return FileAction::Convert;
    }
    FileAction::Copy
}

fn get_target_directory(dest_path: String, file_path: &String) -> String {
    let target_path = dest_path + &file_path;
    let target_path = Path::new(&target_path);
    let directory_path = target_path.parent().unwrap().to_str().unwrap().to_string();
    return directory_path;
}

enum FileAction {
    Copy,
    Convert,
}
