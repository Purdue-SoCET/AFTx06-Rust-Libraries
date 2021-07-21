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

pub enum Channel {
    CH0,
    CH1,
    CH2,
    CH3,
    CH4,
    CH5,
    CH6,
    CH7,
}

pub enum Pre {
    DIV1 =   0,
    DIV2 =   1,
    DIV4 =   2,
    DIV8 =   3,
    DIV16 =  4,
    DIV32 =  5,
    DIV64 =  6,
    DIV128 = 7,
}

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

    pub fn set_output_action(&mut self, channel: Channel, output_action: u32) {
        unsafe {
            let mut curr: u32 = self.p.tcr.read();
            match channel {
                Channel::CH0 => curr = (curr & !(common::tim_tcr_output_mask(0))) | (common::tim_tcr_output_mask(0) & (output_action << 0)),
                Channel::CH1 => curr = (curr & !(common::tim_tcr_output_mask(1))) | (common::tim_tcr_output_mask(1) & (output_action << 1)),
                Channel::CH2 => curr = (curr & !(common::tim_tcr_output_mask(2))) | (common::tim_tcr_output_mask(2) & (output_action << 2)),
                Channel::CH3 => curr = (curr & !(common::tim_tcr_output_mask(3))) | (common::tim_tcr_output_mask(3) & (output_action << 3)),
                Channel::CH4 => curr = (curr & !(common::tim_tcr_output_mask(4))) | (common::tim_tcr_output_mask(4) & (output_action << 4)),
                Channel::CH5 => curr = (curr & !(common::tim_tcr_output_mask(5))) | (common::tim_tcr_output_mask(5) & (output_action << 5)),
                Channel::CH6 => curr = (curr & !(common::tim_tcr_output_mask(6))) | (common::tim_tcr_output_mask(6) & (output_action << 6)),
                Channel::CH7 => curr = (curr & !(common::tim_tcr_output_mask(7))) | (common::tim_tcr_output_mask(7) & (output_action << 7)),
            }
            self.p.tcr.write(curr);
        }
    }

    pub fn set_input_capture_edge(&mut self, channel: Channel, capture_edge: u32) {
        unsafe {
            let mut curr: u32 = self.p.tcr.read();
            match channel {
                Channel::CH0 => curr = (curr & !(common::tim_tcr_edge_mask(0))) | (common::tim_tcr_edge_mask(0) & (capture_edge << 0)),
                Channel::CH1 => curr = (curr & !(common::tim_tcr_edge_mask(1))) | (common::tim_tcr_edge_mask(1) & (capture_edge << 1)),
                Channel::CH2 => curr = (curr & !(common::tim_tcr_edge_mask(2))) | (common::tim_tcr_edge_mask(2) & (capture_edge << 2)),
                Channel::CH3 => curr = (curr & !(common::tim_tcr_edge_mask(3))) | (common::tim_tcr_edge_mask(3) & (capture_edge << 3)),
                Channel::CH4 => curr = (curr & !(common::tim_tcr_edge_mask(4))) | (common::tim_tcr_edge_mask(4) & (capture_edge << 4)),
                Channel::CH5 => curr = (curr & !(common::tim_tcr_edge_mask(5))) | (common::tim_tcr_edge_mask(5) & (capture_edge << 5)),
                Channel::CH6 => curr = (curr & !(common::tim_tcr_edge_mask(6))) | (common::tim_tcr_edge_mask(6) & (capture_edge << 6)),
                Channel::CH7 => curr = (curr & !(common::tim_tcr_edge_mask(7))) | (common::tim_tcr_edge_mask(7) & (capture_edge << 7)),
            }
            self.p.tcr.write(curr);
        }
    }

    pub fn set_prescaler(&mut self, pre_div: Pre) {
        unsafe {
            let mut curr: u32 = self.p.tscr2.read();
            match pre_div {
                Pre::DIV1 =>   curr = (curr & !(TIM_TSCR2_PRE_MASK)) | (TIM_TSCR2_PRE_MASK & (Pre::DIV1 as u32)),
                Pre::DIV2 =>   curr = (curr & !(TIM_TSCR2_PRE_MASK)) | (TIM_TSCR2_PRE_MASK & (Pre::DIV2 as u32)),
                Pre::DIV4 =>   curr = (curr & !(TIM_TSCR2_PRE_MASK)) | (TIM_TSCR2_PRE_MASK & (Pre::DIV4 as u32)),
                Pre::DIV8 =>   curr = (curr & !(TIM_TSCR2_PRE_MASK)) | (TIM_TSCR2_PRE_MASK & (Pre::DIV8 as u32)),
                Pre::DIV16 =>  curr = (curr & !(TIM_TSCR2_PRE_MASK)) | (TIM_TSCR2_PRE_MASK & (Pre::DIV16 as u32)),
                Pre::DIV32 =>  curr = (curr & !(TIM_TSCR2_PRE_MASK)) | (TIM_TSCR2_PRE_MASK & (Pre::DIV32 as u32)),
                Pre::DIV64 =>  curr = (curr & !(TIM_TSCR2_PRE_MASK)) | (TIM_TSCR2_PRE_MASK & (Pre::DIV64 as u32)),
                Pre::DIV128 => curr = (curr & !(TIM_TSCR2_PRE_MASK)) | (TIM_TSCR2_PRE_MASK & (Pre::DIV128 as u32)),
            }
            self.p.tscr2.write(curr);
        }
    }

    pub fn set_output_compare(&mut self, channel: Channel, output_action: u32, interrupt_enable: u32, value: u32) {
        unsafe {
            let mut curr: u32 = self.p.tcr.read();
            match channel {
                Channel::CH0 => {
                    curr = (curr & !(common::tim_tcr_output_mask(0))) | (common::tim_tcr_output_mask(0) & (output_action << 0));
                    self.p.tcr.write(curr);
                    self.p.tc0.write(value);
                    curr = self.p.tie.read();
                    curr = (curr & !(common::tim_tie_mask(0))) | (common::tim_tie_mask(0) & (interrupt_enable << 0));
                }
                Channel::CH1 => {
                    curr = (curr & !(common::tim_tcr_output_mask(1))) | (common::tim_tcr_output_mask(1) & (output_action << 1));
                    self.p.tcr.write(curr);
                    self.p.tc1.write(value);
                    curr = self.p.tie.read();
                    curr = (curr & !(common::tim_tie_mask(1))) | (common::tim_tie_mask(1) & (interrupt_enable << 1));
                }
                Channel::CH2 => {
                    curr = (curr & !(common::tim_tcr_output_mask(2))) | (common::tim_tcr_output_mask(2) & (output_action << 2));
                    self.p.tcr.write(curr);
                    self.p.tc2.write(value);
                    curr = self.p.tie.read();
                    curr = (curr & !(common::tim_tie_mask(2))) | (common::tim_tie_mask(2) & (interrupt_enable << 2));
                }
                Channel::CH3 => {
                    curr = (curr & !(common::tim_tcr_output_mask(3))) | (common::tim_tcr_output_mask(3) & (output_action << 3));
                    self.p.tcr.write(curr);
                    self.p.tc3.write(value);
                    curr = self.p.tie.read();
                    curr = (curr & !(common::tim_tie_mask(3))) | (common::tim_tie_mask(3) & (interrupt_enable << 3));
                }
                Channel::CH4 => {
                    curr = (curr & !(common::tim_tcr_output_mask(4))) | (common::tim_tcr_output_mask(4) & (output_action << 4));
                    self.p.tcr.write(curr);
                    self.p.tc4.write(value);
                    curr = self.p.tie.read();
                    curr = (curr & !(common::tim_tie_mask(4))) | (common::tim_tie_mask(4) & (interrupt_enable << 4));
                }
                Channel::CH5 => {
                    curr = (curr & !(common::tim_tcr_output_mask(5))) | (common::tim_tcr_output_mask(5) & (output_action << 5));
                    self.p.tcr.write(curr);
                    self.p.tc5.write(value);
                    curr = self.p.tie.read();
                    curr = (curr & !(common::tim_tie_mask(5))) | (common::tim_tie_mask(5) & (interrupt_enable << 5));
                }
                Channel::CH6 => {
                    curr = (curr & !(common::tim_tcr_output_mask(6))) | (common::tim_tcr_output_mask(6) & (output_action << 6));
                    self.p.tcr.write(curr);
                    self.p.tc6.write(value);
                    curr = self.p.tie.read();
                    curr = (curr & !(common::tim_tie_mask(6))) | (common::tim_tie_mask(6) & (interrupt_enable << 6));
                }
                Channel::CH7 => {
                    curr = (curr & !(common::tim_tcr_output_mask(7))) | (common::tim_tcr_output_mask(7) & (output_action << 7));
                    self.p.tcr.write(curr);
                    self.p.tc7.write(value);
                    curr = self.p.tie.read();
                    curr = (curr & !(common::tim_tie_mask(7))) | (common::tim_tie_mask(7) & (interrupt_enable << 7));
                }
            }
            self.p.tie.write(curr);
        }
    }

    pub fn set_input_capture(&mut self, channel: Channel, capture_edge: u32, interrupt_enable: u32) {
        unsafe {
            let mut curr = self.p.tcr.read();
            match channel {
                Channel::CH0 => {
                    curr = (curr & !(common::tim_tcr_edge_mask(0))) | (common::tim_tcr_edge_mask(0) & (capture_edge << 0));
                    self.p.tcr.write(curr);
                    curr = self.p.tie.read();
                    curr = (curr & !(common::tim_tie_mask(0))) | (common::tim_tie_mask(0) & (interrupt_enable << 0));
                }
                Channel::CH1 => {
                    curr = (curr & !(common::tim_tcr_edge_mask(1))) | (common::tim_tcr_edge_mask(1) & (capture_edge << 1));
                    self.p.tcr.write(curr);
                    curr = self.p.tie.read();
                    curr = (curr & !(common::tim_tie_mask(1))) | (common::tim_tie_mask(1) & (interrupt_enable << 1));
                }
                Channel::CH2 => {
                    curr = (curr & !(common::tim_tcr_edge_mask(2))) | (common::tim_tcr_edge_mask(2) & (capture_edge << 2));
                    self.p.tcr.write(curr);
                    curr = self.p.tie.read();
                    curr = (curr & !(common::tim_tie_mask(2))) | (common::tim_tie_mask(2) & (interrupt_enable << 2));
                }
                Channel::CH3 => {
                    curr = (curr & !(common::tim_tcr_edge_mask(3))) | (common::tim_tcr_edge_mask(3) & (capture_edge << 3));
                    self.p.tcr.write(curr);
                    curr = self.p.tie.read();
                    curr = (curr & !(common::tim_tie_mask(3))) | (common::tim_tie_mask(3) & (interrupt_enable << 3));
                }
                Channel::CH4 => {
                    curr = (curr & !(common::tim_tcr_edge_mask(4))) | (common::tim_tcr_edge_mask(4) & (capture_edge << 4));
                    self.p.tcr.write(curr);
                    curr = self.p.tie.read();
                    curr = (curr & !(common::tim_tie_mask(4))) | (common::tim_tie_mask(4) & (interrupt_enable << 4));
                }
                Channel::CH5 => {
                    curr = (curr & !(common::tim_tcr_edge_mask(5))) | (common::tim_tcr_edge_mask(5) & (capture_edge << 5));
                    self.p.tcr.write(curr);
                    curr = self.p.tie.read();
                    curr = (curr & !(common::tim_tie_mask(5))) | (common::tim_tie_mask(5) & (interrupt_enable << 5));
                }
                Channel::CH6 => {
                    curr = (curr & !(common::tim_tcr_edge_mask(6))) | (common::tim_tcr_edge_mask(6) & (capture_edge << 6));
                    self.p.tcr.write(curr);
                    curr = self.p.tie.read();
                    curr = (curr & !(common::tim_tie_mask(6))) | (common::tim_tie_mask(6) & (interrupt_enable << 6));
                }
                Channel::CH7 => {
                    curr = (curr & !(common::tim_tcr_edge_mask(7))) | (common::tim_tcr_edge_mask(7) & (capture_edge << 7));
                    self.p.tcr.write(curr);
                    curr = self.p.tie.read();
                    curr = (curr & !(common::tim_tie_mask(7))) | (common::tim_tie_mask(7) & (interrupt_enable << 7));
                }
            }
            self.p.tie.write(curr);
        }
    }

    pub fn read_input_capture(&self, channel: Channel) -> u32 {
        match channel {
            Channel::CH0 => self.p.tc0.read(),
            Channel::CH1 => self.p.tc1.read(),
            Channel::CH2 => self.p.tc2.read(),
            Channel::CH3 => self.p.tc3.read(),
            Channel::CH4 => self.p.tc4.read(),
            Channel::CH5 => self.p.tc5.read(),
            Channel::CH6 => self.p.tc6.read(),
            Channel::CH7 => self.p.tc7.read(),
        }
    }

    pub fn clear_interrupt(&mut self, channel: Channel) {
        unsafe {
            let mut curr = self.p.tflg1.read();
            match channel {
                Channel::CH0 => curr |= TIM_FLG1 & 0,
                Channel::CH1 => curr |= TIM_FLG1 & 1,
                Channel::CH2 => curr |= TIM_FLG1 & 2,
                Channel::CH3 => curr |= TIM_FLG1 & 3,
                Channel::CH4 => curr |= TIM_FLG1 & 4,
                Channel::CH5 => curr |= TIM_FLG1 & 5,
                Channel::CH6 => curr |= TIM_FLG1 & 6,
                Channel::CH7 => curr |= TIM_FLG1 & 7,
            }
            self.p.tflg1.write(curr);
        }
    }
    
    pub fn clear_interrupts(&mut self, channels: u32) {
        if channels > (common::U8_MAX as u32)
        {
            panic!("Channels must be of type u8.")
        }
        unsafe {
            let mut curr = self.p.tflg1.read();
            curr |= TIM_FLG1 & channels;
            self.p.tflg1.write(curr);
        }
    }

    pub fn enable_cf(&mut self, channel: Channel) {
        unsafe {
            let mut curr = self.p.tcf.read();
            match channel {
                Channel::CH0 => curr |= TIM_TCF_MASK & 0,
                Channel::CH1 => curr |= TIM_TCF_MASK & 1,
                Channel::CH2 => curr |= TIM_TCF_MASK & 2,
                Channel::CH3 => curr |= TIM_TCF_MASK & 3,
                Channel::CH4 => curr |= TIM_TCF_MASK & 4,
                Channel::CH5 => curr |= TIM_TCF_MASK & 5,
                Channel::CH6 => curr |= TIM_TCF_MASK & 6,
                Channel::CH7 => curr |= TIM_TCF_MASK & 7,
            }
            self.p.tcf.write(curr);
        }
    }

    pub fn enable_cfs(&mut self, channels: u32) {
        if channels > (common::U8_MAX as u32)
        {
            panic!("Channels must be of type u8.")
        }
        unsafe {
            let mut curr = self.p.tcf.read();
            curr |= TIM_TCF_MASK & channels;
            self.p.tcf.write(curr);
        }
    }

    pub fn enable_tov(&mut self, channel: Channel) {
        unsafe {
            let mut curr = self.p.tov.read();
            match channel {
                Channel::CH0 => curr |= TIM_TOV_MASK & 0,
                Channel::CH1 => curr |= TIM_TOV_MASK & 1,
                Channel::CH2 => curr |= TIM_TOV_MASK & 2,
                Channel::CH3 => curr |= TIM_TOV_MASK & 3,
                Channel::CH4 => curr |= TIM_TOV_MASK & 4,
                Channel::CH5 => curr |= TIM_TOV_MASK & 5,
                Channel::CH6 => curr |= TIM_TOV_MASK & 6,
                Channel::CH7 => curr |= TIM_TOV_MASK & 7,
            }
            self.p.tov.write(curr);
        }
    }

    pub fn enable_tovs(&mut self, channels: u32) {
        unsafe {
            let mut curr = self.p.tov.read();
            curr |= TIM_TOV_MASK & channels;
            self.p.tov.write(curr);
        }
    }

    pub fn disable_tov(&mut self, channel: Channel) {
        unsafe {
            let mut curr = self.p.tov.read();
            match channel {
                Channel::CH0 => curr &= !(TIM_TOV_MASK) | !(0),
                Channel::CH1 => curr &= !(TIM_TOV_MASK) | !(1),
                Channel::CH2 => curr &= !(TIM_TOV_MASK) | !(2),
                Channel::CH3 => curr &= !(TIM_TOV_MASK) | !(3),
                Channel::CH4 => curr &= !(TIM_TOV_MASK) | !(4),
                Channel::CH5 => curr &= !(TIM_TOV_MASK) | !(5),
                Channel::CH6 => curr &= !(TIM_TOV_MASK) | !(6),
                Channel::CH7 => curr &= !(TIM_TOV_MASK) | !(7),
            }
            self.p.tov.write(curr);
        }
    }

    pub fn disable_tovs(&mut self, channels: u32) {
        if channels > (common::U8_MAX as u32)
        {
            panic!("Channels must be of type u8.")
        }
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