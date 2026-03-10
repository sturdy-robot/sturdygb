use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::mpsc::{sync_channel, SyncSender};
use std::sync::Mutex;

pub(super) static AUDIO_PRODUCER: Mutex<Option<SyncSender<[f32; 2]>>> = Mutex::new(None);
pub(super) static AUDIO_STREAM: Mutex<Option<cpal::Stream>> = Mutex::new(None);

pub(super) fn setup_audio(gb: &mut sturdygb_core::gb::Gb) {
    let host = cpal::default_host();
    let device = host.default_output_device();
    if let Some(device) = device {
        let config = device.default_output_config().unwrap().config();

        let sample_rate: u32 = config.sample_rate.into();
        gb.set_sample_rate(sample_rate);

        let (prod, cons) = sync_channel::<[f32; 2]>(4096);

        let channels = config.channels as usize;
        let mut last_sample = [0.0, 0.0];

        let stream = device.build_output_stream(
            &config,
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                for frame in data.chunks_mut(channels) {
                    let sample = match cons.try_recv() {
                        Ok(v) => v,
                        Err(_) => [last_sample[0] * 0.90, last_sample[1] * 0.90],
                    };
                    last_sample = sample;

                    if channels >= 1 && frame.len() >= 1 {
                        frame[0] = sample[0];
                    }
                    if channels >= 2 && frame.len() >= 2 {
                        frame[1] = sample[1];
                    }
                }
            },
            |err| eprintln!("an error occurred on stream: {}", err),
            None,
        );

        if let Ok(stream) = stream {
            stream.play().unwrap();
            if let Ok(mut guard) = AUDIO_PRODUCER.lock() {
                *guard = Some(prod);
            }
            if let Ok(mut guard) = AUDIO_STREAM.lock() {
                *guard = Some(stream);
            }
        }
    }
}
