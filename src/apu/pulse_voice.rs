use crate::apu::apu_device::LENGTH_COUNTER_LOOKUP_TABLE;
// Square voices registers
bf!(SquareVoiceReg1[u8] {
    volume: 0:3,
    envelope_period: 0:3,
    envelope_disable: 4:4,
    loop_envelope: 5:5,
    halt: 5:5,
    duty: 6:7,
});
bf!(SquareVoiceReg2[u8] {
    shift: 0:2,
    negative: 3:3,
    period: 4:6,
    enable_sweep: 7:7,
});
bf!(SquareVoiceReg3[u8] {
});
bf!(SquareVoiceReg4[u8] {
    period_high: 0:2,
    length_index: 3:7,
});

pub struct PulseVoice {
    is_second_pulse_voice: bool,

    pub control_enabled: bool,
    pub length_counter: u16,

    register1: SquareVoiceReg1,
    register2: SquareVoiceReg2,
    register3: u8,
    register4: SquareVoiceReg4,

    envelope_counter: u8,
    envelope_divider: u8,
    envelope_freshly_reset_flag: bool,
    volume_out_of_envelope: u8,

    sweep_divider: u8,
    sweep_freshly_reset_flag: bool,
    sweep_output: bool,

    wave_timer: u16,
    cpu_clock_divider: u8,

    output_sequencer: u8,
}

impl PulseVoice {
    pub fn new(is_second_pulse_voice: bool) -> Self {
        Self {
            is_second_pulse_voice,

            control_enabled: false,
            length_counter: 0,

            register1: SquareVoiceReg1::new(0),
            register2: SquareVoiceReg2::new(0),
            register3: 0,
            register4: SquareVoiceReg4::new(0),

            envelope_counter: 0,
            envelope_divider: 0,
            envelope_freshly_reset_flag: false,
            volume_out_of_envelope: 0,

            sweep_divider: 0,
            sweep_freshly_reset_flag: false,
            sweep_output: false,

            wave_timer: 0,
            cpu_clock_divider: 0,

            output_sequencer: 0,
        }
    }

    pub fn write_register(&mut self, address: u8, data: u8) {
        match address {
            0x00 => { self.register1.val = data; }
            0x01 => {
                self.register2.val = data;
                self.sweep_freshly_reset_flag = true;
            }
            0x02 => { self.register3 = data; }
            0x03 => {
                self.register4.val = data;
                // Write to 4th register triggers length counter reset
                if self.control_enabled {
                    let length_key = self.register4.length_index();
                    self.length_counter = LENGTH_COUNTER_LOOKUP_TABLE[(length_key >> 1) as usize][(length_key & 0x01) as usize] as u16;
                }
                self.envelope_freshly_reset_flag = true;
                self.output_sequencer = 0;
            }
            _ => {}
        }
    }

    pub fn clock_length_counter_and_sweep_unit(&mut self) {
        if self.control_enabled {
            if self.register1.halt() == 0 && self.length_counter > 0 {
                self.length_counter -= 1;
            }
        } else {
            self.length_counter = 0;
        }

        // Sweep unit
        let voice_period = (self.register3 as u16) | ((self.register4.period_high() as u16) << 8);
        let mut shifter_result = voice_period >> self.register2.shift() as u16;
        if self.register2.negative() == 1 {
            shifter_result = !shifter_result;
        }
        if self.is_second_pulse_voice {
            shifter_result += 1;
        }
        shifter_result += voice_period;

        let sweep_divider_period = self.register2.period() + 1;
        if self.sweep_divider > 0 {
        } else {
            let mut dac_output = true;
            if voice_period < 8 || shifter_result > 0x7FF {
                dac_output = false;
            } else if self.register2.enable_sweep() == 1 && self.register2.shift() != 0 {
                self.register3 = ((shifter_result & 0xFF) as u8);
                self.register4.set_period_high(((shifter_result >> 8) & 0xFF) as u8);
            }
            self.sweep_output = dac_output;

            self.sweep_divider = sweep_divider_period;
        }
        self.sweep_divider -= 1;

        if self.sweep_freshly_reset_flag {
            self.sweep_freshly_reset_flag = false;
            self.sweep_divider = sweep_divider_period;
        }
    }

    pub fn clock_envelope(&mut self) {
        let divider_period = self.register1.envelope_period() + 1;
        if self.envelope_freshly_reset_flag {
            self.envelope_counter = 15;
            self.envelope_divider = divider_period;
            self.envelope_freshly_reset_flag = false;
        } else {
            if self.envelope_divider == 0 {
                if self.register1.loop_envelope() == 1 && self.envelope_counter == 0 {
                    self.envelope_counter = 15;
                } else if self.envelope_counter > 0 {
                    self.envelope_counter -= 1;
                }
                self.envelope_divider = divider_period;
            }
        }
        self.envelope_divider -= 1;

        self.volume_out_of_envelope = if self.register1.envelope_disable() == 1 { self.register1.volume() } else { self.envelope_counter };
    }

    pub fn clock_cpu(&mut self) {
        let voice_period = ((self.register3 as u16) | ((self.register4.period_high() as u16) << 8)) + 1;
        if self.wave_timer == 0 {

            if self.cpu_clock_divider == 0 {
                self.cpu_clock_divider = 2;
                self.output_sequencer = (self.output_sequencer + 1) % 8;
            }
            self.cpu_clock_divider -= 1;

            self.wave_timer = voice_period;
        }
        self.wave_timer -= 1;
    }

    pub fn output(&self) -> u8 {
        let sequence = self.output_sequencer;
        let waveform = ((((SQUARE_WAVEFORM_SEQUENCES[self.register1.duty() as usize] >> sequence) & 0x01) != 0) as u8) * 1;
        self.volume_out_of_envelope * (self.sweep_output as u8) * waveform
    }
}

const SQUARE_WAVEFORM_SEQUENCES: [u8; 4] = [
    0b0100_0000,
    0b0110_0000,
    0b0111_1000,
    0b1001_1111,
];