//! This `hub` crate is the
//! entry point of the Rust logic.

mod messages;

mod conversion_handler;
mod encoder_decoder;
pub mod progress_report_buffer;
use conversion_handler::{handle_conversion, ConversionInstructions};

use messages::rust_signal::TotalNumberOfFilesFound;
use rinf::debug_print;
use std::sync::Arc;
use tokio::{self, sync::Mutex};
// use tokio_with_wasm::tokio; // Uncomment this line to target the web

rinf::write_interface!();

// Use `tokio::spawn` to run concurrent tasks.
// Always use non-blocking async functions
// such as `tokio::fs::File::open`.
// If you really need to use blocking code,
// use `tokio::task::spawn_blocking`.

enum AppState {
    Convert,
    DoNothing,
}

async fn main() {
    let app_state = Arc::new(Mutex::new(AppState::DoNothing));
    tokio::spawn(dart_listen_start(Arc::clone(&app_state)));
    tokio::spawn(dart_listen_cancel(Arc::clone(&app_state)));
    tokio::spawn(dart_listen_check_directory());
}

async fn dart_listen_cancel(app_state: Arc<Mutex<AppState>>) {
    use messages::dart_signal::*;
    let mut reciever = Cancel::get_dart_signal_receiver();
    while let Some(_dart_signal) = reciever.recv().await {
        let mut state = app_state.lock().await;
        *state = AppState::DoNothing;
        debug_print!("Requested Cancel");
    }
}

async fn dart_listen_start(app_state: Arc<Mutex<AppState>>) {
    use messages::dart_signal::*;
    let mut receiver = Convert::get_dart_signal_receiver();
    while let Some(dart_signal) = receiver.recv().await {
        let mut state = app_state.lock().await;
        if let AppState::DoNothing = *state {
            *state = AppState::Convert;
        }
        drop(state);
        let message = dart_signal.message.clone();
        debug_print!("{}", message.src_path);
        debug_print!("{}", message.dest_path);
        debug_print!("{}", message.copy_unrecognised_files);
        let instruction = ConversionInstructions {
            copy_unrecognised_files: message.copy_unrecognised_files,
            src_path: message.src_path.clone(),
            dest_path: message.dest_path.clone(),
            no_of_threads: message.no_of_threads.clone(),
            target_format: dart_signal.message.target_format(),
            mp3_config: Mp3Config {
                quality: message.mp3_config.clone().unwrap().quality,
                bitrate: message.mp3_config.clone().unwrap().bitrate,
            },
        };
        let transfered_app_state = Arc::clone(&app_state);

        tokio::spawn(async {
            debug_print!("Starting handle_conversion");
            handle_conversion(instruction, transfered_app_state).await;
            debug_print!("Finished handle_conversion");
        });
    }
}

async fn dart_listen_check_directory() {
    use crate::conversion_handler::traverse_directory;
    use messages::dart_signal::*;
    let mut reciever = CheckDirectory::get_dart_signal_receiver();
    while let Some(check_dir) = reciever.recv().await {
        let check_dir = check_dir.message.src;
        debug_print!("{}", check_dir);
        let mut list_of_files: Vec<String> = Vec::new();
        match traverse_directory(&check_dir, &mut list_of_files, check_dir.len()) {
            Ok(list_of_files) => {
                debug_print!("{}", list_of_files.len());
                TotalNumberOfFilesFound {
                    files_found: true,
                    number: list_of_files.len() as i32,
                }
                .send_signal_to_dart();
            }
            Err(_) => {
                TotalNumberOfFilesFound {
                    files_found: false,
                    number: 0,
                }
                .send_signal_to_dart();
            }
        };
    }
}
