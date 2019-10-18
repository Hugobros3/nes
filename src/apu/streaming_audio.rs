use cpal::traits::{DeviceTrait, EventLoopTrait, HostTrait};
use cpal::{StreamData, UnknownTypeOutputBuffer};
use std::sync::Mutex;
use std::thread;
use std::sync::mpsc::{sync_channel, SyncSender};
use std::cell::UnsafeCell;

pub type FrameSoundBuffer = Vec<u8>;

/// Creates an synchronous thread to read sound data
pub fn launch_sound() -> SyncSender<FrameSoundBuffer> {
    let (tx, rx) = sync_channel::<FrameSoundBuffer>(10);

    let host = cpal::default_host();
    let event_loop = host.event_loop();

    let device = host.default_output_device().expect("no output device available");

    let mut supported_formats_range = device.supported_output_formats()
        .expect("error while querying formats");
    let format = supported_formats_range.next()
        .expect("no supported format?!")
        .with_max_sample_rate();

    let stream_id = event_loop.build_output_stream(&device, &format).unwrap();
    event_loop.play_stream(stream_id).expect("failed to play_stream");

    thread::spawn(move || {
        let mut current_buffer: Option<FrameSoundBuffer> = Option::None;
        let mut current_buffer_pos = 0;
        let mut remaining = 0;
        let mut last_sampled = 0f32;

        let mut rx = Mutex::new(rx);
        let sample_rate = format.sample_rate.0 as f32;

        let mut next_value = move || {
            if remaining == 0 {
                let r = rx.get_mut().unwrap().try_recv();
                if let Result::Ok(buffer) = r {
                    remaining = buffer.len();
                    current_buffer_pos = 0;
                    current_buffer = Some(buffer);
                }
            }

            if remaining > 0 {
                let optref = current_buffer.as_mut();
                let sampled = optref.unwrap()[current_buffer_pos];
                current_buffer_pos += 1;
                remaining -= 1;
                let sampled = sampled as f32 / 255.0;
                last_sampled = sampled;
                sampled
            } else {
                last_sampled
            }
            /*while remaining == 0 {
                let buf = rx.get_mut().unwrap().recv().unwrap();
                remaining = buf.len();
                current_buffer_pos = 0;
                current_buffer = Some(buf);
            }

            let optref = current_buffer.as_mut();
            let sampled = optref.unwrap()[current_buffer_pos];
            current_buffer_pos += 1;
            remaining -= 1;
            let sampled = sampled as f32 / 255.0;
            sampled*/
        };

        event_loop.run(move |stream_id, stream_result| {
            let stream_data = match stream_result {
                Ok(data) => data,
                Err(err) => {
                    eprintln!("an error occurred on stream {:?}: {}", stream_id, err);
                    return;
                }
                _ => return,
            };

            match stream_data {
                cpal::StreamData::Output { buffer: cpal::UnknownTypeOutputBuffer::U16(mut buffer) } => {
                    for sample in buffer.chunks_mut(format.channels as usize) {
                        let sampled = next_value();

                        let value = ((sampled * 0.5 + 0.5) * std::u16::MAX as f32) as u16;
                        for out in sample.iter_mut() {
                            *out = value;
                        }
                    }
                }
                cpal::StreamData::Output { buffer: cpal::UnknownTypeOutputBuffer::I16(mut buffer) } => {
                    for sample in buffer.chunks_mut(format.channels as usize) {
                        let value = (next_value() * std::i16::MAX as f32) as i16;
                        for out in sample.iter_mut() {
                            *out = value;
                        }
                    }
                }
                cpal::StreamData::Output { buffer: cpal::UnknownTypeOutputBuffer::F32(mut buffer) } => {
                    for sample in buffer.chunks_mut(format.channels as usize) {
                        let value = next_value();
                        for out in sample.iter_mut() {
                            *out = value;
                        }
                    }
                }
                _ => {
                    panic!("unhandled :(")
                }
            }
        });
    });

    return tx;
}

/// Feed garbage to test the async sound routines
pub fn garbage_test(output: &mut SyncSender<FrameSoundBuffer>) {
    let mut garbage_data = vec![0u8; 100000];
    garbage_data.iter_mut().for_each(|s| { *s = rand::random::<u8>(); });
    output.send(garbage_data);
}