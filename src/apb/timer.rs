use crate::common;
use volatile_register::{RW};

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
            if common::TIM_CONSTRUCTED == false {
                common::TIM_CONSTRUCTED = true;
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
            curr |= common::TIM_TSCR_ENABLE;
            self.p.tscr.write(curr);
        }
    }

    pub fn disable(&mut self) {
        unsafe {
            let mut curr: u32 = self.p.tscr.read();
            curr &= common::TIM_TSCR_DISABLE;
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
            curr = (curr & !(common::TIM_TSCR2_PRE_MASK)) | (common::TIM_TSCR2_PRE_MASK & pre_div);
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
            curr |= common::TIM_FLG1 & channels;
            self.p.tflg1.write(curr);
        }
    }

    pub fn enable_cf(&mut self, channels: u32) {
        unsafe {
            let mut curr = self.p.tcf.read();
            curr |= common::TIM_TCF_MASK & channels;
            self.p.tcf.write(curr);
        }
    }

    pub fn enable_tov(&mut self, channels: u32) {
        unsafe {
            let mut curr = self.p.tov.read();
            curr |= common::TIM_TOV_MASK & channels;
            self.p.tov.write(curr);
        }
    }

    pub fn disable_tov(&mut self, channels: u32) {
        unsafe {
            let mut curr = self.p.tov.read();
            curr &= !(common::TIM_TOV_MASK) | !channels;
            self.p.tov.write(curr);
        }
    }

    pub fn read_count(&self) -> u32 {
        self.p.tcnt.read()
    }
}