#[derive(Clone, Copy)]
/// The spec that is used for playback and that all sound buffers should use. As of right now, we default to always using
/// F32 with whatever max sample rate the device supports.
pub struct F32FormatSpec {
    pub channels: usize,
    pub sample_rate: usize
}
 
pub trait F32Convert<T> {
    fn convert(val: T) -> f32;
}

impl F32Convert<i16> for F32FormatSpec {
    fn convert(val: i16) -> f32 {
        (val as f32 / 32767.0).clamp(-1.0, 1.0)
    }
}