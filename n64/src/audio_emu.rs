use cpal::{self, StreamData, UnknownTypeOutputBuffer, traits::{HostTrait, DeviceTrait, EventLoopTrait}};
use std::error::Error;
use std::thread;

pub const BUFFER_NO_SAMPLES: usize = 2 * 512;

fn audio_thread() -> Result<(), Box<dyn Error>> {
    let host = cpal::default_host();
    let device = host.default_output_device().ok_or("No output device available")?;
    
    let format = device.default_output_format()?;

    let event_loop = host.event_loop();
    let stream_id = event_loop.build_output_stream(&device, &format)?;

    event_loop.play_stream(stream_id)?;

    event_loop.run(move |stream_id, stream_result| {
        let stream_data = match stream_result {
            Ok(data) => data,
            Err(err) => {
                println!("Audio Stream Error {:?}: {}", stream_id, err);
                return;
            }
        };
    
        match stream_data {
            StreamData::Output { buffer: UnknownTypeOutputBuffer::U16(mut buffer) } => {

                println!("{}", buffer.len());

                for elem in buffer.iter_mut() {
                    *elem = u16::max_value() / 2;
                }
            },
            StreamData::Output { buffer: UnknownTypeOutputBuffer::I16(mut buffer) } => {

                println!("{}", buffer.len());

                for elem in buffer.iter_mut() {
                    *elem = 0;
                }
            },
            StreamData::Output { buffer: UnknownTypeOutputBuffer::F32(mut buffer) } => {

                println!("{}", buffer.len());

                for elem in buffer.iter_mut() {
                    *elem = 0.0;
                }
            },
            _ => (),
        }
    });
}

pub struct Audio {
}

impl Audio {
    #[inline]
    pub(crate) fn new() -> Self { 

        thread::spawn(move || {
            match audio_thread() {
                Ok(()) => (),
                Err(e) => {
                    println!("Audio Error: {}", e);
                }
            }
        });

        Self {
        }
    }

    #[inline]
    pub fn write_audio_blocking(&mut self, _buffer: &[i16]) {
    }

    #[inline]
    pub fn all_buffers_are_full(&self) -> bool {
        
        true
    }

    #[inline]
    pub fn update(&mut self) {
    }
}
