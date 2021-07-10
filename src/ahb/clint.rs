use volatile_register::{RW};

// CLINT Construction Check
pub static mut CLINT_CONSTRUCTED: bool = false;

// CLINT Constants
pub const CLINT: u32 =                  0xE0000000;
pub const CLINT_MSIP: u32 =             CLINT + 0x00;
pub const CLINT_MTIME: u32 =            CLINT + 0x04;
pub const CLINT_MTIMECMP: u32 =         CLINT + 0x0C;
pub const CLINT_MSIP_DISABLE: u32 =     !(1 << 0);
pub const CLINT_MSIP_ENABLE: u32 =      1 << 0;
pub const CLINT_MSIP_MASK: u32 =        1 << 0;

pub struct CLINT {
    p: &'static mut CLINTRegisterBlock
}

#[repr(C)]
struct CLINTRegisterBlock {
    pub msip:      RW<u32>,
    pub mtimeh:    RW<u32>,
    pub mtimel:    RW<u32>,
    pub mtimecmph: RW<u32>,
    pub mtimecmpl: RW<u32>,
}

impl CLINT {
    pub fn new() -> CLINT {
        unsafe {
            if CLINT_CONSTRUCTED == false {
                CLINT_CONSTRUCTED = true;
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
        self.p.msip.read() & CLINT_MSIP_MASK
    }

    pub fn set_interrupt(&mut self) {
        unsafe {
            let mut curr = self.p.msip.read();
            curr |= CLINT_MSIP_ENABLE;
            self.p.msip.write(curr);
        }
    }

    pub fn clear_interrupt(&mut self) {
        unsafe {
            let mut curr = self.p.msip.read();
            curr &= CLINT_MSIP_DISABLE;
            self.p.msip.write(curr);
        }
    }
}