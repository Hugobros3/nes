use crate::apu::apu_device::LENGTH_COUNTER_LOOKUP_TABLE;
bf!(TriangleVoiceReg1[u8] {
    linear_counter_reload_value: 0:6,
    control: 7:7,
});
bf!(TriangleVoiceReg4[u8]{
    period_high: 0:2,
    length_index: 3:7,
});

pub struct TriangleVoice {
    pub control_enabled: bool,
    pub length_counter: u16,

    register1: TriangleVoiceReg1,
    register3: u8,
    register4: TriangleVoiceReg4,

    linear_counter: u8,
    linear_counter_halt_flag: bool,

    timer: u16,

    output_sequencer: u8,
}

impl TriangleVoice {
    pub fn new() -> Self {
        Self {
            control_enabled: false,

            register1: TriangleVoiceReg1::new(0),
            register3: 0,
            register4: TriangleVoiceReg4::new(0),

            length_counter: 0,

            linear_counter: 0,
            linear_counter_halt_flag: false,

            timer: 0,

            output_sequencer: 0,
        }
    }

    pub fn write_register(&mut self, address: u8, data: u8) {
        match address {
            /* 0x4008 */ 0x00 => { self.register1.val = data; }
            /* 0x4009 */ // nothing here !
            /* 0x400A */ 0x02 => { self.register3 = data; }
            /* 0x400B */ 0x03 => {
                self.register4.val = data;

                // Write to 4th register triggers length counter reset
                if self.control_enabled {
                    let length_key = self.register4.length_index();
                    self.length_counter = LENGTH_COUNTER_LOOKUP_TABLE[(length_key >> 1) as usize][(length_key & 0x01) as usize] as u16;
                }

                self.linear_counter_halt_flag = true;
            }
            _ => {}
        }
    }

    pub fn clock_length_counter(&mut self) {
        if self.control_enabled {
            if self.register1.control() == 0 && self.length_counter > 0 {
                self.length_counter -= 1;
            }
        } else {
            self.length_counter = 0;
        }
    }

    pub fn clock_linear_counter(&mut self) {
        if self.linear_counter_halt_flag {
            self.linear_counter = self.register1.linear_counter_reload_value();
        } else if self.linear_counter > 0 {
            self.linear_counter -= 1;
        }

        if self.register1.control() == 0 {
            self.linear_counter_halt_flag = false;
        }
    }

    pub fn clock_cpu(&mut self) {
        let voice_period = ((self.register3 as u16) | ((self.register4.period_high() as u16) << 8)) + 1;
        //println!("{} {} {}", self.register2, self.register3.val, voice_period);
        //let voice_period = 40;
        if self.timer == 0 {
            if self.linear_counter > 0 && self.length_counter > 0 {
                self.output_sequencer = (self.output_sequencer + 1) % 32;
            }

            self.timer = voice_period;
        }
        self.timer -= 1;
    }

    pub fn output(&self) -> u8 {
        TRIANGLE_VOICE_OUTPUT_SEQUENCE[self.output_sequencer as usize]
    }
}

const TRIANGLE_VOICE_OUTPUT_SEQUENCE: [u8;32] = [0xF, 0xE, 0xD, 0xC, 0xB, 0xA, 0x9, 0x8, 0x7, 0x6, 0x5, 0x4, 0x3, 0x2, 0x1, 0x0, 0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0xA, 0xB, 0xC, 0xD, 0xE, 0xF];