use std::mem::replace;

use rtrb::{Producer, Consumer};

use super::{AppToPlayerMessage, PlayerToAppMessage};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlaybackState {
    Paused,
    Playing,
}

pub struct Process {
    current_buffer: Option<Vec<f32>>,
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

    // NOTE: as of right now we never have errors, but in the future, if we do,
    // go back to a try_process (which returns void) calls a process (which returns result).
    // I like that pattern
    pub fn process(
        &mut self,
        data: &mut [f32],
    ) {
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
            return;
        }

        // assmue data is 2 channels and at correct sample rate.
        if let Some(current_buffer) = &self.current_buffer {
            if self.current_buffer_idx + data.len() >= current_buffer.len() {
                // TODO: PLEASE VET THIS (will be easy to do with synthetic waveform nodes we start making where we can make them be 1 sec)
                let count_at_end = current_buffer.len() - self.current_buffer_idx;
                let count_at_start = data.len() - count_at_end;
                unsafe {
                    std::ptr::copy(current_buffer.as_ptr().add(self.current_buffer_idx), data.as_mut_ptr(), count_at_end);
                    std::ptr::copy(current_buffer.as_ptr(), data.as_mut_ptr().add(count_at_end), count_at_start);                    
                }
                self.current_buffer_idx = count_at_start;
            } else {
                unsafe {
                    std::ptr::copy(current_buffer.as_ptr().add(self.current_buffer_idx), data.as_mut_ptr(), data.len());
                }
                self.current_buffer_idx += data.len();
            }

            let _ = self.player_to_app_send.push(PlayerToAppMessage::PlaybackPosition(self.current_buffer_idx, current_buffer.len()));
        } else {
            silence(data);
            let _ = self.player_to_app_send.push(PlayerToAppMessage::PlaybackPosition(0, 0));
        }

    }
}

pub fn silence(data: &mut [f32]) {
    for sample in data.iter_mut() {
        *sample = 0.0;
    }
}