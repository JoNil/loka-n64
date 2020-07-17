use cpal::{
    self,
    traits::{DeviceTrait, EventLoopTrait, HostTrait},
    StreamData, UnknownTypeOutputBuffer,
};
use rubato::{InterpolationParameters, InterpolationType, Resampler, SincFixedIn, WindowFunction};
use std::error::Error;
use std::{
    sync::mpsc::{channel, Receiver, Sender},
    thread,
};

const BUFFER_NO_SAMPLES: usize = 2 * 512;
const BUFFER_COUNT: usize = 4;

fn get_sample<T>(
    resampler: &mut SincFixedIn<f32>,
    current_index: &mut usize,
    current_buffer: &mut Option<Vec<f32>>,
    to_audio_receiver: &Receiver<Buffer>,
    from_audio_sender: &Sender<Buffer>,
    sample_convert: impl Fn(f32) -> T,
) -> T {
    if current_buffer.is_none() {
        if let Ok(buffer) = to_audio_receiver.try_recv() {
            let mut channels = Vec::new();
            channels.push(Vec::with_capacity(BUFFER_NO_SAMPLES / 2));
            channels.push(Vec::with_capacity(BUFFER_NO_SAMPLES / 2));

            for frame in buffer.samples.chunks_exact(2) {
                channels[0].push(frame[0] as f32 / (i16::MAX as f32));
                channels[1].push(frame[1] as f32 / (i16::MAX as f32));
            }

            let resampled_buffers = resampler.process(&channels).unwrap();

            let mut converted_buffer =
                Vec::with_capacity(resampled_buffers[0].len() + resampled_buffers[1].len());

            for samples in resampled_buffers[0].iter().zip(resampled_buffers[1].iter()) {
                converted_buffer.push(*samples.0);
                converted_buffer.push(*samples.1);
            }

            *current_buffer = Some(converted_buffer);
            *current_index = 0;

            from_audio_sender.send(buffer).ok();
        }
    }

    sample_convert(if let Some(buffer) = current_buffer {
        let sample = buffer[*current_index];

        *current_index += 1;

        if *current_index >= buffer.len() {
            *current_buffer = None;
        }

        sample
    } else {
        0.0
    })
}

fn audio_thread(
    to_audio_receiver: Receiver<Buffer>,
    from_audio_sender: Sender<Buffer>,
) -> Result<(), Box<dyn Error>> {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .ok_or("No output device available")?;

    let format = device.default_output_format()?;
    assert!(format.channels == 2);

    let event_loop = host.event_loop();
    let stream_id = event_loop.build_output_stream(&device, &format)?;

    event_loop.play_stream(stream_id)?;

    let params = InterpolationParameters {
        sinc_len: 64,
        f_cutoff: 0.95,
        interpolation: InterpolationType::Nearest,
        oversampling_factor: 160,
        window: WindowFunction::BlackmanHarris2,
    };

    let mut resampler = SincFixedIn::<f32>::new(
        format.sample_rate.0 as f64 / 22050.0,
        params,
        BUFFER_NO_SAMPLES / 2,
        2,
    );

    let mut current_buffer = None;
    let mut current_index = 0;

    event_loop.run(move |stream_id, stream_result| {
        let stream_data = match stream_result {
            Ok(data) => data,
            Err(err) => {
                println!("Audio Stream Error {:?}: {}", stream_id, err);
                return;
            }
        };

        match stream_data {
            StreamData::Output {
                buffer: UnknownTypeOutputBuffer::U16(mut buffer),
            } => {
                for elem in buffer.iter_mut() {
                    *elem = get_sample(
                        &mut resampler,
                        &mut current_index,
                        &mut current_buffer,
                        &to_audio_receiver,
                        &from_audio_sender,
                        |sample| (sample + (u16::MAX / 2) as f32) as u16,
                    );
                }
            }
            StreamData::Output {
                buffer: UnknownTypeOutputBuffer::I16(mut buffer),
            } => {
                for elem in buffer.iter_mut() {
                    *elem = get_sample(
                        &mut resampler,
                        &mut current_index,
                        &mut current_buffer,
                        &to_audio_receiver,
                        &from_audio_sender,
                        |sample| (sample / (i16::MAX as f32)) as i16,
                    );
                }
            }
            StreamData::Output {
                buffer: UnknownTypeOutputBuffer::F32(mut buffer),
            } => {
                for elem in buffer.iter_mut() {
                    *elem = get_sample(
                        &mut resampler,
                        &mut current_index,
                        &mut current_buffer,
                        &to_audio_receiver,
                        &from_audio_sender,
                        |sample| sample as f32,
                    );
                }
            }
            _ => (),
        }
    });
}

struct Buffer {
    samples: Box<[i16]>,
}

impl Buffer {
    fn new() -> Self {
        let mut samples = Vec::new();
        samples.resize_with(BUFFER_NO_SAMPLES, Default::default);
        Self {
            samples: samples.into_boxed_slice(),
        }
    }
}

pub struct Audio {
    to_audio_sender: Sender<Buffer>,
    from_audio_receiver: Receiver<Buffer>,
    buffers: Vec<Buffer>,
}

impl Audio {
    #[inline]
    pub(crate) fn new() -> Self {
        let mut buffers = Vec::new();

        for _ in 0..BUFFER_COUNT {
            buffers.push(Buffer::new());
        }

        let (to_audio_sender, to_audio_receiver) = channel();
        let (from_audio_sender, from_audio_receiver) = channel();

        thread::spawn(
            move || match audio_thread(to_audio_receiver, from_audio_sender) {
                Ok(()) => (),
                Err(e) => {
                    println!("Audio Error: {}", e);
                }
            },
        );

        Self {
            to_audio_sender,
            from_audio_receiver,
            buffers,
        }
    }

    #[inline]
    pub fn update(&mut self, mut f: impl FnMut(&mut [i16])) {
        while let Ok(buffer) = self.from_audio_receiver.try_recv() {
            self.buffers.push(buffer);
        }

        for mut buffer in self.buffers.drain(..) {
            f(&mut buffer.samples);
            self.to_audio_sender
                .send(buffer)
                .map_err(|_| println!("Failed to send buffer to audio system"))
                .ok();
        }
    }
}
