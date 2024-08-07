use std::mem::replace;

use rtrb::{Producer, Consumer};

use super::{AppToPlayerMessage, PlayerToAppMessage};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlaybackState {
    Paused,
    Playing,
}

pub struct Process {
    current_buffer: Option<Vec<i16>>,
    current_buffer_idx: usize,

    playback_state: PlaybackState,

    app_to_player_recv: Consumer<AppToPlayerMessage>,
    player_to_app_send: Producer<PlayerToAppMessage>,
}

impl Process {
    pub fn new( app_to_player_recv: Consumer<AppToPlayerMessage>, player_to_app_send: Producer<PlayerToAppMessage>) -> Self {        
        Self {
            current_buffer: None,
            current_buffer_idx: 0,            
            playback_state: PlaybackState::Paused,
            app_to_player_recv,
            player_to_app_send,
        }
    }

    // TODO: were not acc erroring out anymore, what do?
    pub fn process(&mut self, data: &mut [i16]) {
        println!("X");
        if let Err(e) = self.try_process(data) {
            silence(data);
            println!("Error: {:?}", e);
        }
    }

    pub fn try_process(
        &mut self,
        data: &mut [i16],
    ) -> Result<(), ()> {
        while let Ok(msg) = self.app_to_player_recv.pop() {
            match msg {
                AppToPlayerMessage::Play => {
                    self.playback_state = PlaybackState::Playing;
                },
                AppToPlayerMessage::Pause => {
                    self.playback_state = PlaybackState::Paused;
                },
                AppToPlayerMessage::SeekTo(_) => {
                    todo!()
                },
                AppToPlayerMessage::UseBuffer(new_buffer) => {
                    if let Some(current_buffer) = &mut self.current_buffer {
                        let _ = self.player_to_app_send.push(PlayerToAppMessage::DropBuffer(replace(current_buffer, Vec::new())));
                    }

                    self.playback_state = PlaybackState::Paused;
                    self.current_buffer = Some(new_buffer);
                    self.current_buffer_idx = 0;
                },
            }
        }

        if self.playback_state == PlaybackState::Paused {
            silence(data);
            return Ok(());
        }

        // assmue data is 2 channels and at correct sample rate.
        if let Some(current_buffer) = &self.current_buffer {
            // TODO: do some copying or sumn
            for sample in data {
                if self.current_buffer_idx >= current_buffer.len() {
                    println!("D");
                    self.current_buffer_idx = 0;
                }

                *sample = current_buffer[self.current_buffer_idx];
                self.current_buffer_idx += 1;
            }
            Ok(())
        } else {
            silence(data);
            Ok(())
        }
    }
}

pub fn silence(data: &mut [i16]) {
    for sample in data.iter_mut() {
        *sample = 0;
    }
}