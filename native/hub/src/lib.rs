//! This `hub` crate is the
//! entry point of the Rust logic.

mod messages;

mod conversion_handler;
mod encoder_decoder;
use conversion_handler::{handle_conversion, ConversionInstructions};

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
}

async fn dart_listen_cancel(app_state: Arc<Mutex<AppState>>) {
    use messages::dart_signal::*;
    let mut reciever = Cancel::get_dart_signal_receiver();
    while let Some(_dart_signal) = reciever.recv().await {
        let mut state = app_state.lock().await;
        *state = AppState::DoNothing;
    }
    debug_print!("returning start");
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
