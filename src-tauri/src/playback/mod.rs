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
    PlaybackPosition(usize),
    DropBuffer(Vec<f32>)
}

