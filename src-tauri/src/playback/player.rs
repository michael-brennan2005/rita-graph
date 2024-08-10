use cpal::{traits::{DeviceTrait, HostTrait, StreamTrait}, SampleFormat, Stream, StreamConfig, SupportedStreamConfigRange};
use rtrb::{Consumer, PopError, Producer, RingBuffer};

use crate::playback::process::Process;

use super::{spec::F32FormatSpec, AppToPlayerMessage, PlayerToAppMessage};

pub struct Player {
    // Stream needs to be restored else the output thread will get destroyed
    #[allow(dead_code)]
    stream: Stream, 
    format: F32FormatSpec,

    app_to_player_send: Producer<AppToPlayerMessage>,
    player_to_app_recv: Consumer<PlayerToAppMessage>,
}

impl Player {
    pub fn new() -> Player {
        let host = cpal::default_host();

        let device = host.default_output_device().unwrap();
        println!("Using default device: {}", device.name().unwrap());

        let mut config_range: Option<SupportedStreamConfigRange> = None;

        for available_config in device.supported_output_configs().unwrap() {
            println!(
                "Found config with sample format {} and sample rate [{:?}, {:?}]",
                available_config.sample_format(),
                available_config.min_sample_rate(),
                available_config.max_sample_rate()
            );
            if available_config.sample_format() == SampleFormat::F32 {
                println!("Will use this sample format as it is f32");
                config_range = Some(available_config);
                break;
            }
        }

        if config_range.is_none() {
            println!("Could not find output config with i16 format");
            panic!();
        }

        let config_range = config_range.unwrap();
        let config: StreamConfig = config_range.clone().with_max_sample_rate().into();

        let format: F32FormatSpec = F32FormatSpec {
            channels: config.channels as usize,
            sample_rate: config.sample_rate.0 as usize,
        };

        println!("Final config: {:?}", config);

        let (app_to_player_send, app_to_player_recv) = RingBuffer::<AppToPlayerMessage>::new(256);
        let (player_to_app_send, player_to_app_recv) = RingBuffer::<PlayerToAppMessage>::new(256);
        let mut process = Process::new(app_to_player_recv, player_to_app_send);

        let stream = device
            .build_output_stream(
                &config,
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    process.process(data);
                },
                move |err| {
                    panic!("Audio ouput stream failure: {}", err);
                },
                None,
            )
            .expect("Stream building failed");

        stream.play().expect("Stream playing failed");

        Self {
            stream,
            format,
            app_to_player_send,
            player_to_app_recv,
        }
    }

    pub fn play(&mut self) {
        let _ = self.app_to_player_send.push(AppToPlayerMessage::Play);
    }

    pub fn pause(&mut self) {
        let _ = self.app_to_player_send.push(AppToPlayerMessage::Pause);
    }

    pub fn format(&self) -> F32FormatSpec {
        self.format
    }

    pub fn set_new_buffer(&mut self, buffer: Vec<f32>) {
        let _ = self.app_to_player_send.push(AppToPlayerMessage::UseBuffer(buffer));
    }

    pub fn pop_message(&mut self) -> Result<PlayerToAppMessage, PopError> {
        self.player_to_app_recv.pop()
    }
}