use crate::common;
use volatile_register::{RW};

pub struct CLINT {
    p: &'static mut CLINTRegisterBlock
}

#[repr(C)]
struct CLINTRegisterBlock {
    pub msip:     RW<u32>,
    pub mtime:    RW<u64>,
    pub mtimecmp: RW<u64>,
}

impl CLINT {
    pub fn new() -> CLINT {
        unsafe {
            if common::CLINT_CONSTRUCTED == false {
                common::CLINT_CONSTRUCTED = true;
                CLINT {
                    p: &mut *(0xE000_0000 as *mut CLINTRegisterBlock) 
                }
            }
            else { 
                panic!("You may construct only one instance of CLINT.")
            }
        }
    }

    pub fn interrupt_status(&self) -> u32 {
        self.p.msip.read() & common::CLINT_MSIP_MASK
    }

    pub fn set_interrupt(&mut self) {
        unsafe {
            let mut curr = self.p.msip.read();
            curr |= common::CLINT_MSIP_ENABLE;
            self.p.msip.write(curr);
        }
    }

    pub fn clear_interrupt(&mut self) {
        unsafe {
            let mut curr = self.p.msip.read();
            curr &= common::CLINT_MSIP_DISABLE;
            self.p.msip.write(curr);
        }
    }
}