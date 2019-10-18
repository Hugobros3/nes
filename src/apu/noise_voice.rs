use crate::apu::apu_device::LENGTH_COUNTER_LOOKUP_TABLE;
// Noise voice registers
bf!(NoiseVoiceReg1[u8] {
    volume: 0:3,
    envelope_period: 0:3,
    envelope_disable: 4:4,
    loop_envelope: 5:5,
    halt: 5:5,
    unused: 6:7,
});
bf!(NoiseVoiceReg3[u8] {
    period_index: 0:3,
    unused: 4:6,
    mode: 7:7,
});
bf!(NoiseVoiceReg4[u8] {
    unused: 0:2,
    length_index: 3:7,
});

pub struct NoiseVoice {
    pub control_enabled: bool,
    pub length_counter: u16,

    register1: NoiseVoiceReg1,
    register3: NoiseVoiceReg3,
    register4: NoiseVoiceReg4,

    envelope_counter: u8,
    envelope_divider: u8,
    envelope_freshly_reset_flag: bool,
    volume_out_of_envelope: u8,

    shift_register: u16,

    timer: u16,
}

impl NoiseVoice {
    pub fn new() -> Self {
        Self {
            control_enabled: false,
            length_counter: 0,

            register1: NoiseVoiceReg1::new(0),
            register3: NoiseVoiceReg3::new(0),
            register4: NoiseVoiceReg4::new(0),

            envelope_counter: 0,
            envelope_divider: 0,
            envelope_freshly_reset_flag: false,
            volume_out_of_envelope: 0,

            shift_register: 1,

            timer: 0,
        }
    }

    pub fn write_register(&mut self, address: u8, data: u8) {
        match address {
            /* 0x400C */ 0x00 => { self.register1.val = data; }
            /* 0x400D */ // nothing here !
            /* 0x400E */ 0x02 => { self.register3.val = data; }
            /* 0x400F */ 0x03 => {
                self.register4.val = data;

                // Write to 4th register triggers length counter reset
                if self.control_enabled {
                    let length_key = self.register4.length_index();
                    self.length_counter = LENGTH_COUNTER_LOOKUP_TABLE[(length_key >> 1) as usize][(length_key & 0x01) as usize] as u16;
                }
            }
            _ => {}
        }
    }

    pub fn clock_length_counter(&mut self) {
        if self.control_enabled {
            if self.register1.halt() == 0 && self.length_counter > 0 {
                self.length_counter -= 1;
            }
        } else {
            self.length_counter = 0;
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
        let timer_period = TIMER_PERIODS[self.register3.period_index() as usize];
        if self.timer == 0 {
            let bit14 = if self.register3.mode() == 1 {
                let bit0 = self.shift_register & 0x01;
                let bit6 = (self.shift_register >> 6) & 0x01;
                bit0 ^ bit6
            } else {
                let bit0 = self.shift_register & 0x01;
                let bit1 = (self.shift_register >> 1) & 0x01;
                bit0 ^ bit1
            };
            self.shift_register >>= 1;
            self.shift_register |= bit14 << 14;

            self.timer = timer_period;
        }
        self.timer -= 1;
    }

    pub fn output(&self) -> u8 {
        let bit0 = (self.shift_register & 0x01) as u8;
        self.volume_out_of_envelope * bit0 * ((self.length_counter > 0) as u8)
    }
}

const TIMER_PERIODS: [u16; 16] = [
    0x04,
    0x08,
    0x10,
    0x20,
    0x40,
    0x60,
    0x80,
    0xA0,
    0xCA,
    0xFE,
    0x17C,
    0x1FC,
    0x2FA,
    0x3F8,
    0x7F2,
    0xFE4,
];