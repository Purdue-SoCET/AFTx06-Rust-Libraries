use crate::common;
use volatile_register::{RW};

pub struct PWM {
    p: &'static mut PWMRegisterBlock
}

#[repr(C)]
struct PWMRegisterBlock {
    pub pwm0_period: RW<u32>,
    pub pwm0_duty:   RW<u32>,
    pub pwm0_ctrl:   RW<u32>,
}

impl PWM {
    pub fn new() -> PWM {
        unsafe {
            if common::PWM_CONSTRUCTED == false {
                common::PWM_CONSTRUCTED = true;
                PWM {
                    p: &mut *(0x8000_0004 as *mut PWMRegisterBlock) 
                }
            }
            else { 
                panic!("You may construct only one instance of PWM.")
            }
        }
    }

    pub fn set_frequency(&mut self, channel: u32, mut frequency: u32) {
        unsafe {
            if common::PWM_MAX_FREQ < frequency { frequency = common::PWM_MAX_FREQ; }
            let period: u32 = common::rounding_division(common::CHIP_FREQ, frequency);
            match channel {
                0 => {
                    self.p.pwm0_period.write(period);
                    self.p.pwm0_duty.write(common::rounding_division(period, 2) + common::AFTX06_DUTY_OFFSET);
                }
                _ => panic!("You may only write to pwm channel 0."),
            }
        }
    }

    pub fn set_period(&mut self, channel: u32, period: u32) {
        unsafe {
            match channel {
                0 => self.p.pwm0_period.write(period),
                _ => panic!("You may only write to pwm channel 0."),
            }
        }
    }

    pub fn set_duty(&mut self, channel: u32, duty: u32) {
        unsafe {
            match channel {
                0 => self.p.pwm0_duty.write(duty + common::AFTX06_DUTY_OFFSET),
                _ => panic!("You may only write to pwm channel 0."),
            }
        }
    }

    pub fn disable(&mut self, channel: u32) {
        unsafe {
            match channel {
                0 => {
                    let mut curr: u32 = self.p.pwm0_ctrl.read();
                    curr &= common::PWM_CONTROL_DISABLE;
                    self.p.pwm0_ctrl.write(curr);
                }
                _ => panic!("You may only write to pwm channel 0."),
            }
        }
    }

    pub fn enable(&mut self, channel: u32) {
        unsafe {
            match channel {
                0 => {
                    let mut curr: u32 = self.p.pwm0_ctrl.read();
                    curr |= common::PWM_CONTROL_ENABLE;
                    self.p.pwm0_ctrl.write(curr);
                }
                _ => panic!("You may only write to pwm channel 0."),
            }
        }
    }

    pub fn set_active_high(&mut self, channel: u32) {
        unsafe {
            match channel {
                0 => {
                    let mut curr: u32 = self.p.pwm0_ctrl.read();
                    curr &= common::PWM_CONTROL_ACTIVE_HIGH;
                    self.p.pwm0_ctrl.write(curr);
                }
                _ => panic!("You may only write to pwm channel 0."),
            }
        }
    }

    pub fn set_active_low(&mut self, channel: u32) {
        unsafe {
            match channel {
                0 => {
                    let mut curr: u32 = self.p.pwm0_ctrl.read();
                    curr |= common::PWM_CONTROL_ACTIVE_LOW;
                    self.p.pwm0_ctrl.write(curr);
                }
                _ => panic!("You may only write to pwm channel 0."),
            }
        }
    }

    pub fn set_align_left(&mut self, channel: u32) {
        unsafe {
            match channel {
                0 => {
                    let mut curr: u32 = self.p.pwm0_ctrl.read();
                    curr &= common::PWM_CONTROL_ALIGN_LEFT;
                    self.p.pwm0_ctrl.write(curr);
                }
                _ => panic!("You may only write to pwm channel 0."),
            }
        }
    }

    pub fn set_align_center(&mut self, channel: u32) {
        unsafe {
            match channel {
                0 => {
                    let mut curr: u32 = self.p.pwm0_ctrl.read();
                    curr |= common::PWM_CONTROL_ALIGN_CENTER;
                    self.p.pwm0_ctrl.write(curr);
                }
                _ => panic!("You may only write to pwm channel 0."),
            }
        }
    }
}