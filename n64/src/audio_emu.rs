use cpal::{
    self,
    traits::{DeviceTrait, EventLoopTrait, HostTrait},
    StreamData, UnknownTypeOutputBuffer, Format,
};
use std::error::Error;
use std::{
    sync::mpsc::{channel, Receiver, Sender},
    thread,
};
use sample::{interpolate, ring_buffer, signal, Sample, Signal};

pub const BUFFER_NO_SAMPLES: usize = 2 * 512;
pub const BUFFER_COUNT: usize = 4;

fn get_sample<T>(
    format: &Format,
    //sinc: &mut interpolate::Sinc<[[f32; 1]; 100]>,
    current_index: &mut usize,
    current_buffer: &mut Option<Vec<i16>>,
    to_audio_receiver: &Receiver<Buffer>,
    from_audio_sender: &Sender<Buffer>,
    sample_convert: impl Fn(i16) -> T,
) -> T {
    if current_buffer.is_none() {
        if let Ok(buffer) = to_audio_receiver.try_recv() {

            let samples = buffer.samples.iter().map(|s| i16::to_sample::<f32>(*s));
            let signal = signal::from_interleaved_samples_iter(samples);


            let ring_buffer = ring_buffer::Fixed::from([[0.0]; 100]);
            let sinc = interpolate::Sinc::new(ring_buffer);
            
            let new_signal = signal.from_hz_to_hz(sinc, 22050 as f64, format.sample_rate.0 as f64);

            let converted_buffer = new_signal.until_exhausted().map().collect::<Vec<_>>();

            *current_buffer = Some(converted_buffer);
            *current_index = 0;

            from_audio_sender.send(buffer).ok();
        }
    }

    let sample = if let Some(buffer) = current_buffer {
        let sample = buffer[*current_index];

        *current_index += 1;

        if *current_index >= buffer.len() {
            current_buffer = None;
        }

        sample_convert(sample)
    } else {
        sample_convert(0)
    };

    sample
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

    let event_loop = host.event_loop();
    let stream_id = event_loop.build_output_stream(&device, &format)?;

    event_loop.play_stream(stream_id)?;

    //let sinc = interpolate::Sinc::new(ring_buffer::Fixed::from([[0.0]; 100]));

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
                        &format,
                        //&mut sinc,
                        &mut current_index,
                        &mut current_buffer,
                        &to_audio_receiver,
                        &from_audio_sender,
                        |sample| (sample as f32 + (u16::MAX / 2) as f32) as u16,
                    );
                }
            }
            StreamData::Output {
                buffer: UnknownTypeOutputBuffer::I16(mut buffer),
            } => {
                for elem in buffer.iter_mut() {
                    *elem = get_sample(
                        &format,
                        //&mut sinc,
                        &mut current_index,
                        &mut current_buffer,
                        &to_audio_receiver,
                        &from_audio_sender,
                        |sample| sample,
                    );
                }
            }
            StreamData::Output {
                buffer: UnknownTypeOutputBuffer::F32(mut buffer),
            } => {
                for elem in buffer.iter_mut() {
                    *elem = get_sample(
                        &format,
                        //&mut sinc,
                        &mut current_index,
                        &mut current_buffer,
                        &to_audio_receiver,
                        &from_audio_sender,
                        |sample| sample as f32 / (i16::MAX as f32),
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
