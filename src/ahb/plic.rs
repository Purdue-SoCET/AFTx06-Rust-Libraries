use crate::common;
use volatile_register::{RW};

pub struct PLIC {
    p: &'static mut PLICRegisterBlock
}

#[repr(C)]
struct PLICRegisterBlock {
    pub res1:   u32,
    pub ipr1:   RW<u32>,
    pub ipr2:   RW<u32>,
    pub ipr3:   RW<u32>,
    pub ipr4:   RW<u32>,
    pub ipr5:   RW<u32>,
    pub ipr6:   RW<u32>,
    pub ipr7:   RW<u32>,
    pub ipr8:   RW<u32>,
    pub ipr9:   RW<u32>,
    pub ipr10:  RW<u32>,
    pub ipr11:  RW<u32>,
    pub ipr12:  RW<u32>,
    pub ipr13:  RW<u32>,
    pub ipr14:  RW<u32>,
    pub ipr15:  RW<u32>,
    pub ipr16:  RW<u32>,
    pub ipr17:  RW<u32>,
    pub ipr18:  RW<u32>,
    pub ipr19:  RW<u32>,
    pub ipr20:  RW<u32>,
    pub ipr21:  RW<u32>,
    pub ipr22:  RW<u32>,
    pub ipr23:  RW<u32>,
    pub ipr24:  RW<u32>,
    pub ipr25:  RW<u32>,
    pub ipr26:  RW<u32>,
    pub ipr27:  RW<u32>,
    pub ipr28:  RW<u32>,
    pub ipr29:  RW<u32>,
    pub ipr30:  RW<u32>,
    pub ipr31:  RW<u32>,
    pub ipr32:  RW<u32>,
    pub ipndgr: RW<u32>,
    pub ier:    RW<u32>,
    pub res2:   u32,
    pub ptr:    RW<u32>,
    pub ccr:    RW<u32>,
}

impl PLIC {
    pub fn new() -> PLIC {
        unsafe {
            if common::PLIC_CONSTRUCTED == false {
                common::PLIC_CONSTRUCTED = true;
                PLIC {
                    p: &mut *(0xE001_0000 as *mut PLICRegisterBlock)
                }
            }
            else {
                panic!("You may construct only one instance of PLIC.")
            }
        }
    }
}