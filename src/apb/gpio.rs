use crate::common;
use volatile_register::{RW};

pub struct GPIO {
    p: &'static mut GPIORegisterBlock
}

#[repr(C)]
struct GPIORegisterBlock {
    pub data:     RW<u32>,
    pub data_dir: RW<u32>,
    pub intr_en:  RW<u32>,
    pub pos_edge: RW<u32>,
    pub neg_edge: RW<u32>,
    pub intr_clr: RW<u32>,
    pub intr_sts: RW<u32>,
}

impl GPIO {
    pub fn new() -> GPIO {
        unsafe {
            if common::GPIO_CONSTRUCTED == false {
                common::GPIO_CONSTRUCTED = true;
                GPIO {
                    p: &mut *(0x8000_0004 as *mut GPIORegisterBlock) 
                }
            }
            else { 
                panic!("You may construct only one instance of GPIO.")
            }
        }
    }

    pub fn enable_input(&mut self, pins: u32) {
        unsafe {
            let mut curr: u32 = self.p.data_dir.read();
            curr &= !pins;
            self.p.data_dir.write(curr);
        }
    }

    pub fn read_input(&self, pins: u32) -> u32 {
        self.p.data.read() & pins
    }

    pub fn enable_output(&mut self, pins: u32) {
        unsafe {
            let mut curr: u32 = self.p.data_dir.read();
            curr |= pins;
            self.p.data_dir.write(curr);
        }
    }

    pub fn set_output(&mut self, pins: u32, pin_outputs: u32) {
        unsafe {
            let mut curr: u32 = self.p.data.read();
            curr &= !pins;
            curr |= pins & pin_outputs;
            self.p.data.write(curr);
        }
    }

    pub fn enable_interrupt_posedge(&mut self, pins: u32) {
        unsafe {
            let mut curr: u32 = self.p.neg_edge.read();
            curr &= !pins;
            self.p.neg_edge.write(curr);
            curr = self.p.pos_edge.read();
            curr |= pins;
            self.p.pos_edge.write(curr);
            curr = self.p.intr_en.read();
            curr |= pins;
            self.p.intr_en.write(curr);
        }
    }

    pub fn disable_interrupt_posedge(&mut self, pins: u32) {
        unsafe {
            let mut curr: u32 = self.p.intr_en.read();
            curr &= !pins;
            self.p.intr_en.write(curr);
            curr = self.p.pos_edge.read();
            curr &= !pins;
            self.p.pos_edge.write(curr);
        }
    }

    pub fn clear_interrupt(&mut self, pins: u32) {
        unsafe{
            let mut curr: u32 = self.p.intr_clr.read();
            curr |= pins;
            self.p.intr_clr.write(curr);
        }
    }

    pub fn interrupt_status(&self, pins: u32) -> u32{
        self.p.intr_sts.read() & pins
    }
}