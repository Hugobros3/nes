use crate::bus::Bus;
use std::sync::mpsc::SyncSender;
use crate::apu::streaming_audio::FrameSoundBuffer;
use crate::apu::pulse_voice::PulseVoice;
use crate::apu::triangle_voice::TriangleVoice;

// Noise voice registers
bf!(NoiseVoiceReg1[u8] {
    vol_env_period: 0:3,
    env_disable: 4:4,
    loop_env_disable_length: 5:5,
    halt: 5:5,
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
bf!(DmcReg1[u8] {
    frequency_index: 0:3,
    unused: 4:5,
    do_loop: 6:6,
    irq_enable: 7:7,
});
bf!(DmcReg2[u8] {
    dac: 0:6,
    unused: 7:7,
});
bf!(DmcReg3[u8] {
    sample_address: 0:7,
});
bf!(DmcReg4[u8] {
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
    square_voice1: PulseVoice,
    square_voice2: PulseVoice,
    triangle_voice: TriangleVoice,

    noise_1: NoiseVoiceReg1,
    noise_2: NoiseVoiceReg2,
    noise_3: NoiseVoiceReg3,
    noise_length_counter: u16,

    dmc_1: DmcReg1,
    dmc_2: DmcReg2,
    dmc_3: DmcReg3,
    dmc_4: DmcReg4,
    dmc_irq: bool,

    common1: CommonReg1,
    common2: CommonReg2,

    sequencer_counter: u8,
    sequencer_divider: u32,
    sequencer_interrupt_flag: bool,

    audio_output: SyncSender<FrameSoundBuffer>,
    audio_buffer: Vec<u8>,
}

impl Apu {
    pub(crate) fn new(audio_output: SyncSender<FrameSoundBuffer>) -> Self {
        Self {
            square_voice1: PulseVoice::new(false),
            square_voice2: PulseVoice::new(true),

            triangle_voice: TriangleVoice::new(),

            noise_1: NoiseVoiceReg1::new(0),
            noise_2: NoiseVoiceReg2::new(0),
            noise_3: NoiseVoiceReg3::new(0),
            noise_length_counter: 0,

            dmc_1: DmcReg1::new(0),
            dmc_2: DmcReg2::new(0),
            dmc_3: DmcReg3::new(0),
            dmc_4: DmcReg4::new(0),
            dmc_irq: false,

            common1: CommonReg1::new(0),
            common2: CommonReg2::new(0),
            sequencer_counter: 0,
            sequencer_divider: 0,
            sequencer_interrupt_flag: false,

            audio_output,
            audio_buffer: Vec::<u8>::with_capacity(14_900),
        }
    }

    pub fn cpu_write(&mut self, bus: &Bus, address: u16, data: u8) {
        if address >= 0x4000 && address <= 0x4003 {
            self.square_voice1.write_register(((address & 0x03) as u8), data);
        } else if address >= 0x4004 && address <= 0x4007 {
            self.square_voice2.write_register(((address & 0x03) as u8), data);
        } else if address >= 0x4008 && address <= 0x400B {
            self.triangle_voice.write_register(((address & 0x03) as u8), data);
        } else if address >= 0x400C && address <= 0x400F {
            //self.triangle_voice.write_register(((address & 0x03) as u8), data);
        }

        match address {
            // Noise channel
            0x400C => { self.noise_1.val = data; }
            //0x400D: nothing
            0x400E => { self.noise_2.val = data; }
            0x400F => { self.noise_3.val = data; }
            // DPCM
            0x4010 => { self.dmc_1.val = data; }
            0x4011 => { self.dmc_2.val = data; }
            0x4012 => { self.dmc_3.val = data; }
            0x4013 => { self.dmc_4.val = data; }
            // Control
            0x4015 => {
                self.common1.val = data;
                self.square_voice1.control_enabled = self.common1.length_ctr_enable_pulse_1() != 0;
                self.square_voice2.control_enabled = self.common1.length_ctr_enable_pulse_2() != 0;
                self.triangle_voice.control_enabled = self.common1.length_ctr_enable_triangle() != 0;

                self.dmc_irq = false;
                //TODO DMC behavior
                //If d is set and the DMC's DMA reader has no more sample bytes to fetch, the DMC
                //sample is restarted. If d is clear then the DMA reader's sample bytes remaining
                //is set to 0.
            }
            0x4017 => {
                self.common2.val = data;
                self.sequencer_counter = 0;
                self.sequencer_divider = 0;

                if self.common2.frame_sequencer_mode() != 0 {
                    self.clock_sequencer();
                }
            }
            _ => {}
        }
    }

    pub fn cpu_read(&mut self, address: u16, data: &mut u8) {
        if address == 0x4015 {
            *data = ((self.dmc_irq as u8) << 7) |
                ((self.sequencer_interrupt_flag as u8) << 6) |
                /*(((self.dmc_sample_bytes_remaining > 0) as u8) << 4) | */
                (((self.noise_length_counter > 0) as u8) << 3) |
                (((self.triangle_voice.length_counter > 0) as u8) << 2) |
                (((self.square_voice2.length_counter > 0) as u8) << 1);
                (((self.square_voice1.length_counter > 0) as u8) << 0);

            self.sequencer_interrupt_flag = false;
        }
    }

    pub fn clock_main(&mut self, cycles: u32) {
        self.sequencer_divider += cycles;
        if self.sequencer_divider >= 89490 {
            self.sequencer_divider -= 89490;
            self.clock_sequencer();
        }
    }

    fn clock_sequencer(&mut self) {
        if self.common2.frame_sequencer_mode() == 0 {
            // mode 0
            if self.sequencer_counter == 3 {
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

    pub fn is_raising_interrupt(&self) -> bool {
        self.common2.irq_disable() == 0 && self.sequencer_interrupt_flag
    }

    fn clock_length_counters_and_sweep_units(&mut self) {
        self.square_voice1.clock_length_counter_and_sweep_unit();
        self.square_voice2.clock_length_counter_and_sweep_unit();

        self.triangle_voice.clock_length_counter();

        if self.common1.length_ctr_enable_noise() == 0 {
            self.noise_length_counter = 0;
        } else {
            if self.noise_1.halt() == 0 && self.noise_length_counter > 0 {
                self.noise_length_counter -= 1;
            }
        }
    }

    fn clock_envelopes_and_triangle_linear_counter(&mut self) {
        self.square_voice1.clock_envelope();
        self.square_voice2.clock_envelope();
        self.triangle_voice.clock_linear_counter();
    }

    pub fn clock_cpu_clock(&mut self) {
        self.square_voice1.clock_cpu();
        self.square_voice2.clock_cpu();
        self.triangle_voice.clock_cpu();

        let output = self.square_voice1.output() + self.square_voice2.output() + self.triangle_voice.output();
        self.audio_buffer.push(output);

        //TODO other channels
    }

    pub fn frame_done(&mut self) {
        let mut swap = Vec::<u8>::with_capacity(14_900);
        let downsample_me = std::mem::replace(&mut self.audio_buffer, swap);

        let sample_length = downsample_me.len();

        let mut bytes_req = 800;
        let mut fvec = Vec::<u8>::new();
        for i in 0..bytes_req {
            let next = i + 1;
            let mut start = (((sample_length as f32 / bytes_req as f32) * (i as f32)) as usize);
            let mut end = (((sample_length as f32 / bytes_req as f32) * (next as f32)) as usize);
            if start > sample_length {
                start = sample_length - 1;
            }
            if end > sample_length {
                end = sample_length - 1;
            }
            let size = end - start;
            assert_ne!(size, 0);
            let mut acc = 0.0f32;
            for sample_index in start..end {
                acc += downsample_me[sample_index] as f32;
            }
            acc /= size as f32;

            fvec.push(acc as u8);
        }

        self.audio_output.send(fvec);
    }
}

pub const LENGTH_COUNTER_LOOKUP_TABLE: [[u8; 2]; 16] = [
    [0x0A, 0xFE],
    [0x14, 0x02],
    [0x28, 0x04],
    [0x50, 0x06],
    [0xA0, 0x08],
    [0x3C, 0x0A],
    [0x0E, 0x0C],
    [0x1A, 0x0E],
    [0x0C, 0x10],
    [0x18, 0x12],
    [0x30, 0x14],
    [0x60, 0x16],
    [0xC0, 0x18],
    [0x48, 0x1A],
    [0x10, 0x1C],
    [0x20, 0x1E],
];