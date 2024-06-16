//! This `hub` crate is the
//! entry point of the Rust logic.

mod messages;

use messages::mp3::Mp3Bitrate;
use tokio;
// use tokio_with_wasm::tokio; // Uncomment this line to target the web

rinf::write_interface!();

// Use `tokio::spawn` to run concurrent tasks.
// Always use non-blocking async functions
// such as `tokio::fs::File::open`.
// If you really need to use blocking code,
// use `tokio::task::spawn_blocking`.
async fn main() {
    use messages::basic::*;
}
