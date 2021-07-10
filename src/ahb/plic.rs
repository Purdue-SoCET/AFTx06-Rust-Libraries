use volatile_register::{RW};

// PLIC Construction Check
pub static mut PLIC_CONSTRUCTED: bool =  false;

// PLIC Constants
pub const PLIC: u32 =        0xE0000000;
pub const PLIC_RES1: u32 =   PLIC + 0x00;
pub const PLIC_IPR1: u32 =   PLIC + 0x04;
pub const PLIC_IPR2: u32 =   PLIC + 0x08;
pub const PLIC_IPR3: u32 =   PLIC + 0x0C;
pub const PLIC_IPR4: u32 =   PLIC + 0x10;
pub const PLIC_IPR5: u32 =   PLIC + 0x14;
pub const PLIC_IPR6: u32 =   PLIC + 0x18;
pub const PLIC_IPR7: u32 =   PLIC + 0x1C;
pub const PLIC_IPR8: u32 =   PLIC + 0x20;
pub const PLIC_IPR9: u32 =   PLIC + 0x24;
pub const PLIC_IPR10: u32 =  PLIC + 0x28;
pub const PLIC_IPR11: u32 =  PLIC + 0x2C;
pub const PLIC_IPR12: u32 =  PLIC + 0x30;
pub const PLIC_IPR13: u32 =  PLIC + 0x34;
pub const PLIC_IPR14: u32 =  PLIC + 0x38;
pub const PLIC_IPR15: u32 =  PLIC + 0x3C;
pub const PLIC_IPR16: u32 =  PLIC + 0x40;
pub const PLIC_IPR17: u32 =  PLIC + 0x44;
pub const PLIC_IPR18: u32 =  PLIC + 0x48;
pub const PLIC_IPR19: u32 =  PLIC + 0x4C;
pub const PLIC_IPR20: u32 =  PLIC + 0x50;
pub const PLIC_IPR21: u32 =  PLIC + 0x54;
pub const PLIC_IPR22: u32 =  PLIC + 0x58;
pub const PLIC_IPR23: u32 =  PLIC + 0x5C;
pub const PLIC_IPR24: u32 =  PLIC + 0x60;
pub const PLIC_IPR25: u32 =  PLIC + 0x64;
pub const PLIC_IPR26: u32 =  PLIC + 0x68;
pub const PLIC_IPR27: u32 =  PLIC + 0x6C;
pub const PLIC_IPR28: u32 =  PLIC + 0x70;
pub const PLIC_IPR29: u32 =  PLIC + 0x74;
pub const PLIC_IPR30: u32 =  PLIC + 0x78;
pub const PLIC_IPR31: u32 =  PLIC + 0x7C;
pub const PLIC_IPR32: u32 =  PLIC + 0x80;
pub const PLIC_IPNDGR: u32 = PLIC + 0x84;
pub const PLIC_IER: u32 =    PLIC + 0x88;
pub const PLIC_RES2: u32 =   PLIC + 0x8C;
pub const PLIC_PTR: u32 =    PLIC + 0x90;
pub const PLIC_CCRL: u32 =   PLIC + 0x94;

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
            if PLIC_CONSTRUCTED == false {
                PLIC_CONSTRUCTED = true;
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