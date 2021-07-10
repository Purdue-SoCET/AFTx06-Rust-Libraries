use crate::common;
use volatile_register::{RW};

// Timer Construction Check
pub static mut TIM_CONSTRUCTED: bool =   false;

// Timer Constants
pub const TIM: u32 =                       0x80020000;
pub const TIM_IOS: u32 =                   TIM + 0x00;
pub const TIM_TCF: u32 =                   TIM + 0x04;
pub const TIM_TCNT: u32 =                  TIM + 0x08;
pub const TIM_TSCR: u32 =                  TIM + 0x0C;
pub const TIM_TOV: u32 =                   TIM + 0x10;
pub const TIM_TCR: u32 =                   TIM + 0x14;
pub const TIM_TIE: u32 =                   TIM + 0x18;
pub const TIM_TSCR2: u32 =                 TIM + 0x1C;
pub const TIM_FLG1: u32 =                  TIM + 0x20;
pub const TIM_FLG2: u32 =                  TIM + 0x24;
pub const TIM_TCF_MASK: u32 =              0xFF;
pub const TIM_TSCR_ENABLE: u32 =           1 << 7;
pub const TIM_TSCR_DISABLE: u32 =          !(1 << 7);
pub const TIM_TOV_MASK: u32 =              0xFF;
pub const TIM_TCR_EDGE_DISABLE: u32 =      0x000;
pub const TIM_TCR_EDGE_FALLING: u32 =      0x001;
pub const TIM_TCR_EDGE_RISING: u32 =       0x100;
pub const TIM_TCR_EDGE_EITHER: u32 =       0x101;
pub const TIM_TCR_OUTPUT_DISCONNECT: u32 = 0x000 << 16;
pub const TIM_TCR_OUTPUT_TOGGLE: u32 =     0x001 << 16;
pub const TIM_TCR_OUTPUT_CLEAR: u32 =      0x100 << 16;
pub const TIM_TCR_OUTPUT_SET: u32 =        0x101 << 16;
pub const TIM_TIE_ENABLE: u32 =            1;
pub const TIM_TIE_DISABLE: u32 =           !1;
pub const TIM_TSCR2_TOI_ENABLE: u32 =      1 << 7;
pub const TIM_TSCR2_TOI_DISABLE: u32 =     !(1 << 7);
pub const TIM_TSCR2_TCRE_ENABLE: u32 =     1 << 6;
pub const TIM_TSCR2_TCRE_DISABLE: u32 =    !(1 << 6);
pub const TIM_TSCR2_PRE_MASK: u32 =        0x7;
pub const TIM_TSCR2_PRE_DIV1: u32 =        0;
pub const TIM_TSCR2_PRE_DIV2: u32 =        1;
pub const TIM_TSCR2_PRE_DIV4: u32 =        2;
pub const TIM_TSCR2_PRE_DIV8: u32 =        3;
pub const TIM_TSCR2_PRE_DIV16: u32 =       4;
pub const TIM_TSCR2_PRE_DIV32: u32 =       5;
pub const TIM_TSCR2_PRE_DIV64: u32 =       6;
pub const TIM_TSCR2_PRE_DIV128: u32 =      7;
pub const TIM_FLG1_MASK: u32 =             0xFF;
pub const TIM_FLG2_CLEAR: u32 =            1 << 7;

pub struct TIM {
    p: &'static mut TIMRegisterBlock
}

#[repr(C)]
struct TIMRegisterBlock {
    pub ios:    RW<u32>,
    pub tcf:    RW<u32>,
    pub tcnt:   RW<u32>,
    pub tscr:   RW<u32>,
    pub tov:    RW<u32>,
    pub tcr:    RW<u32>,
    pub tie:    RW<u32>,
    pub tscr2:  RW<u32>,
    pub tflg1:  RW<u32>,
    pub tflg2:  RW<u32>,
    pub tc0:    RW<u32>,
    pub tc1:    RW<u32>,
    pub tc2:    RW<u32>,
    pub tc3:    RW<u32>,
    pub tc4:    RW<u32>,
    pub tc5:    RW<u32>,
    pub tc6:    RW<u32>,
    pub tc7:    RW<u32>,
    pub rlv:    RW<u32>,
}

impl TIM {
    pub fn new() -> TIM {
        unsafe {
            if TIM_CONSTRUCTED == false {
                TIM_CONSTRUCTED = true;
                TIM {
                    p: &mut *(0x8002_0000 as *mut TIMRegisterBlock)
                }
            }
            else {
                panic!("You may construct only one instance of TIM.")
            }
        }
    }

    pub fn enable(&mut self) {
        unsafe {
            let mut curr: u32 = self.p.tscr.read();
            curr |= TIM_TSCR_ENABLE;
            self.p.tscr.write(curr);
        }
    }

    pub fn disable(&mut self) {
        unsafe {
            let mut curr: u32 = self.p.tscr.read();
            curr &= TIM_TSCR_DISABLE;
            self.p.tscr.write(curr);
        }
    }

    pub fn set_output_action(&mut self, channel: u32, output_action: u32) {
        unsafe {
            let mut curr: u32 = self.p.tcr.read();
            curr = (curr & !(common::tim_tcr_output_mask(channel))) | (common::tim_tcr_output_mask(channel) & (output_action << channel));
            self.p.tcr.write(curr);
        }
    }

    pub fn set_input_capture_edge(&mut self, channel: u32, capture_edge: u32) {
        unsafe {
            let mut curr: u32 = self.p.tcr.read();
            curr = (curr & !(common::tim_tcr_edge_mask(channel))) | (common::tim_tcr_edge_mask(channel) & (capture_edge << channel));
            self.p.tcr.write(curr);
        }
    }

    pub fn set_prescaler(&mut self, pre_div: u32) {
        unsafe {
            let mut curr: u32 = self.p.tscr2.read();
            curr = (curr & !(TIM_TSCR2_PRE_MASK)) | (TIM_TSCR2_PRE_MASK & pre_div);
            self.p.tscr2.write(curr);
        }
    }

    pub fn set_output_compare(&mut self, channel: u32, output_action: u32, interrupt_enable: u32, value: u32) {
        unsafe {
            let mut curr: u32 = self.p.tcr.read();
            curr = (curr & !(common::tim_tcr_output_mask(channel))) | (common::tim_tcr_output_mask(channel) & (output_action << channel));
            self.p.tcr.write(curr);
            match channel {
                0 => self.p.tc0.write(value),
                1 => self.p.tc1.write(value),
                2 => self.p.tc2.write(value),
                3 => self.p.tc3.write(value),
                4 => self.p.tc4.write(value),
                5 => self.p.tc5.write(value),
                6 => self.p.tc6.write(value),
                7 => self.p.tc7.write(value),
                _ => panic!("You can't write to a channel outside the bounds of 0-7."),
            }
            curr = self.p.tie.read();
            curr = (curr & !(common::tim_tie_mask(channel))) | (common::tim_tie_mask(channel) & (interrupt_enable << channel));
            self.p.tie.write(curr);
        }
    }

    pub fn set_input_capture(&mut self, channel: u32, capture_edge: u32, interrupt_enable: u32) {
        unsafe {
            let mut curr = self.p.tcr.read();
            curr = (curr & !(common::tim_tcr_edge_mask(channel))) | (common::tim_tcr_edge_mask(channel) & (capture_edge << channel));
            self.p.tcr.write(curr);
            curr = self.p.tie.read();
            curr = (curr & !(common::tim_tie_mask(channel))) | (common::tim_tie_mask(channel) & (interrupt_enable << channel));
            self.p.tie.write(curr);
        }
    }

    pub fn read_input_capture(&self, channel: u32) -> u32 {
        match channel {
            0 => self.p.tc0.read(),
            1 => self.p.tc1.read(),
            2 => self.p.tc2.read(),
            3 => self.p.tc3.read(),
            4 => self.p.tc4.read(),
            5 => self.p.tc5.read(),
            6 => self.p.tc6.read(),
            7 => self.p.tc7.read(),
            _ => panic!("You can't read a channel outside the bounds of 0-7."),
        }
    }

    pub fn clear_interrupt(&mut self, channels: u32) {
        unsafe {
            let mut curr = self.p.tflg1.read();
            curr |= TIM_FLG1 & channels;
            self.p.tflg1.write(curr);
        }
    }

    pub fn enable_cf(&mut self, channels: u32) {
        unsafe {
            let mut curr = self.p.tcf.read();
            curr |= TIM_TCF_MASK & channels;
            self.p.tcf.write(curr);
        }
    }

    pub fn enable_tov(&mut self, channels: u32) {
        unsafe {
            let mut curr = self.p.tov.read();
            curr |= TIM_TOV_MASK & channels;
            self.p.tov.write(curr);
        }
    }

    pub fn disable_tov(&mut self, channels: u32) {
        unsafe {
            let mut curr = self.p.tov.read();
            curr &= !(TIM_TOV_MASK) | !channels;
            self.p.tov.write(curr);
        }
    }

    pub fn read_count(&self) -> u32 {
        self.p.tcnt.read()
    }
}