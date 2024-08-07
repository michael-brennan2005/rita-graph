mod process;
pub mod player;

pub enum AppToPlayerMessage {
    Play,
    Pause,
    SeekTo(usize),
    UseBuffer(Vec<i16>)
}

pub enum PlayerToAppMessage {
    PlaybackPosition(usize),
    DropBuffer(Vec<i16>)
}
