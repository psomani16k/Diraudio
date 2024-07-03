use std::{collections::VecDeque, sync::Arc, time::Duration};

use tokio::{sync::Mutex, time::sleep};

use crate::messages::rust_signal::{MessageType, ProgressUpdate};

pub struct ProgressBuffer {
    buffer: VecDeque<ProgressUpdate>,
}

impl ProgressBuffer {
    pub fn new() -> ProgressBuffer {
        let buff: VecDeque<ProgressUpdate> = VecDeque::new();
        ProgressBuffer { buffer: buff }
    }

    pub fn add(&mut self, update: ProgressUpdate) {
        self.buffer.push_back(update);
    }

    pub fn send_one_update(&mut self) -> bool {
        match self.buffer.pop_front() {
            Some(update) => {
                // debug_print!("{:?}", update);
                let is_end = update.message_type == MessageType::ConversionFinish.into();
                update.send_signal_to_dart();
                return is_end;
            }
            None => {
                return false;
            }
        };
    }
}

pub async fn handle_buffer(buffer: Arc<Mutex<ProgressBuffer>>) {
    loop {
        {
            let mut progress_buffer = buffer.lock().await;
            match progress_buffer.send_one_update() {
                true => {
                    return;
                }
                false => {}
            }
        }
        sleep(Duration::from_millis(100)).await;
    }
}
