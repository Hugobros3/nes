use cpal::traits::{DeviceTrait, EventLoopTrait, HostTrait};
use cpal::{StreamData, UnknownTypeOutputBuffer};
use crate::bus::Bus;

pub fn test_sound() {
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

    let sample_rate = format.sample_rate.0 as f32;
    let mut sample_clock = 0f32;

    println!("{}", format.sample_rate.0);

    // Produce a sinusoid of maximum amplitude.
    let mut next_value = || {
        sample_clock = (sample_clock + 1.0) % sample_rate;
        //rand::random::<f32>()
        //if ((sample_clock * (440.0) / sample_rate) % 1.0) < 0.45 + 0.05 * rand::random::<f32>() { 1.0 } else { 0.0 }
        //(sample_clock * 440.0 * 2.0 * 3.141592 / sample_rate).sin()
        (sample_clock / sample_rate < 0.20) as i32 as f32 * if ((sample_clock * (440.0) / sample_rate) % 1.0) < 0.5 { 1.0 } else { 0.0 }
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
                    let value = ((next_value() * 0.5 + 0.5) * std::u16::MAX as f32) as u16;
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
            _ => (),
        }
    });
}

// Square voices registers
bf!(SquareVoiceReg1[u8] {
    volume_env_period: 0:3,
    env_disable: 4:4,
    env_disable_length: 5:5,
    duty: 6:7,
});
bf!(SquareVoiceReg2[u8] {
    shift: 0:2,
    negative: 3:3,
    period: 4:6,
    enable_sweep: 7:7,
});
bf!(SquareVoiceReg3[u8] {
    period_low: 0:7,
});
bf!(SquareVoiceReg4[u8] {
    period_high: 0:2,
    length_index: 3:7,
});

// Triangle voice registers
bf!(TriangleVoiceReg1[u8] {
    linear_counter_load: 0:6,
    control: 7:7,
});
bf!(TriangleVoiceReg2[u8]{
    period_low: 0:7,
});
bf!(TriangleVoiceReg3[u8]{
    period_high: 0:2,
    length_index: 3:7,
});

// Noise voice registers
bf!(NoiseVoiceReg1[u8] {
    vol_env_period: 0:3,
    env_disable: 4:4,
    loop_env_disable_length: 5:5,
    unused: 6:7,
});
bf!(NoiseVoiceReg2[u8] {
    period_index: 0:3,
    unused: 4:6,
    short_mode: 7:7,
});
bf!(NoiseVoiceReg3[u8] {
    unused: 0:2,
    length_index: 3:7,
});

// DMC registers
bf!(DMCReg1[u8] {
    frequency_index: 0:3,
    unused: 4:5,
    do_loop: 6:6,
    irq_enable: 7:7,
});
bf!(DMCReg2[u8] {
    dac: 0:6,
    unused: 7:7,
});
bf!(DMCReg3[u8] {
    sample_address: 0:7,
});
bf!(DMCReg4[u8] {
    sample_length: 0:7,
});

// Common registers
bf!(CommonReg1[u8] {
    length_ctr_enable_pulse_1: 0:0,
    length_ctr_enable_pulse_2: 1:1,
    length_ctr_enable_triangle: 2:2,
    length_ctr_enable_noise: 3:3,
    length_ctr_enable_dmc: 4:4,
    unused: 5:7,
});
bf!(CommonReg2[u8] {
    unused: 0:5,
    irq_disable: 6:6,
    frame_sequencer_mode: 7:7,
});

pub struct Apu {
    irq_disable: bool,
    frame_sequencer_mode: bool,
    sequencer_counter: u8,
    sequencer_divider: u32,
    sequencer_interrupt_flag: bool,
}

impl Apu {
    fn new() -> Self {
        Self {
            irq_disable: false,
            frame_sequencer_mode: false,
            sequencer_counter: 0,
            sequencer_divider: 0,
            sequencer_interrupt_flag: false,
        }
    }

    fn cpu_write(&mut self, bus: &Bus, address: u16, data: u8) {
        match address {
            0x4017 => {
                // Reset divider & sequencer
                self.frame_sequencer_mode = data & 0x80 != 0;
                self.irq_disable = data & 0x40 != 0;
                self.sequencer_counter = 0;
                self.sequencer_divider = 0;

                if (self.frame_sequencer_mode) {
                    self.clock_sequencer();
                }
            }
            _ => {}
        }
    }

    fn clock_main(&mut self, cycles: u32) {
        self.sequencer_divider += cycles;
        if self.sequencer_divider >= 89490 {
            self.sequencer_divider -= 89490;
            self.clock_sequencer();
        }
    }

    fn clock_sequencer(&mut self) {
        if !(self.frame_sequencer_mode) {
            // mode 0
            if self.sequencer_counter == 3 && !self.irq_disable {
                self.sequencer_interrupt_flag = true;
            }

            if self.sequencer_counter % 2 == 1 {
                self.clock_length_counters_and_sweep_units();
            }

            self.clock_envelopes_and_triangle_linear_counter();

            self.sequencer_counter = (self.sequencer_counter + 1) % 5;
        } else {
            // mode 1
            if self.sequencer_counter % 2 == 0 && self.sequencer_counter < 4 {
                self.clock_length_counters_and_sweep_units();
            }

            if self.sequencer_counter < 4 {
                self.clock_envelopes_and_triangle_linear_counter();
            }

            self.sequencer_counter = (self.sequencer_counter + 1) % 5;
        }
    }

    fn clock_length_counters_and_sweep_units(&mut self) {}

    fn clock_envelopes_and_triangle_linear_counter(&mut self) {}
}