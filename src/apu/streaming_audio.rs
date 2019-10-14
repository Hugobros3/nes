use cpal::traits::{DeviceTrait, EventLoopTrait, HostTrait};
use cpal::{StreamData, UnknownTypeOutputBuffer};
use std::sync::Mutex;
use std::thread;
use std::sync::mpsc::{sync_channel, SyncSender};
use std::cell::UnsafeCell;

pub type FrameSoundBuffer = Vec<u8>;

//static QUEUE: Mutex<Vec<Box<FrameSoundBuffer>>> = Mutex::new(Vec::<Box<FrameSoundBuffer>>::new());

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

        let mut rx = Mutex::new(rx);
        let sample_rate = format.sample_rate.0 as f32;
        let mut sample_clock = 0f32;
        //println!("{:?}", format.sample_rate);

        // Produce a sinusoid of maximum amplitude.
        /*let mut next_value = || {
            sample_clock = (sample_clock + 1.0) % sample_rate;
            //rand::random::<f32>()
            //if ((sample_clock * (440.0) / sample_rate) % 1.0) < 0.45 + 0.05 * rand::random::<f32>() { 1.0 } else { 0.0 }
            //(sample_clock * 440.0 * 2.0 * 3.141592 / sample_rate).sin()
            (sample_clock / sample_rate < 0.20) as i32 as f32 * if ((sample_clock * (440.0) / sample_rate) % 1.0) < 0.5 { 1.0 } else { 0.0 }
        };*/

        let mut next_value = move || {
            while remaining == 0 {
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
            sampled


            //sample_clock = (sample_clock + 1.0) % sample_rate;
            //(sample_clock / sample_rate < 0.20) as i32 as f32 * if ((sample_clock * (440.0) / sample_rate) % 1.0) < 0.5 { 1.0 } else { 0.0 }
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
                        /*if remaining == 0 {
                            let buf = rx.get_mut().unwrap().recv().unwrap();
                            remaining = buf.len();
                            current_buffer_pos = 0;
                            current_buffer = Some(buf);
                        }

                        let optref = current_buffer.as_mut();
                        let sampled = optref.unwrap()[current_buffer_pos];
                        current_buffer_pos += 1;
                        remaining -= 1;
                        let sampled = sampled as f32 / 255.0;*/
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

pub fn garbage_test(output: &mut SyncSender<FrameSoundBuffer>) {
    let mut garbage_data = vec![0u8; 100000];
    garbage_data.iter_mut().for_each(|s| { *s = rand::random::<u8>(); });
    output.send(garbage_data);

    //garbage_data.map_in_place(| s | {  *s = rand::random::<u8>()})
}