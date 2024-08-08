mod process;
pub mod player;
pub mod spec;

pub enum AppToPlayerMessage {
    Play,
    Pause,
    SeekTo(usize),
    UseBuffer(Vec<f32>)
}

pub enum PlayerToAppMessage {
    // First usize is the last read idx of the buffer, second usize is the size of the buffer
    PlaybackPosition(usize, usize),
    DropBuffer(Vec<f32>)
}

