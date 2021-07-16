use volatile_register::{RW};
use crate::common::{U8_MAX};

// GPIO Construction Check
pub static mut GPIO_CONSTRUCTED: bool =  false;

// GPIO Constants
pub const GPIO: u32 =                  	0x80000000;
pub const GPIO_DATA: u32 =             	GPIO + 0x04;
pub const GPIO_DATA_DIRECTION: u32 =   	GPIO + 0x08;
pub const GPIO_INTERRUPT_ENABLE: u32 =	GPIO + 0x0C;
pub const GPIO_POSITIVE_EDGE: u32 =     GPIO + 0x10;
pub const GPIO_NEGATIVE_EDGE: u32 =     GPIO + 0x14;
pub const GPIO_INTERRUPT_CLEAR: u32 =	GPIO + 0x18;
pub const GPIO_INTERRUPT_STATUS: u32 =	GPIO + 0x1C;
pub const GPIOALL_AFTX06: u32 =         0xFF;
pub const GPIOALL: u32 =                0xFFFFFFFF;

// GPIO Pins
pub enum Pin {
    PIN0 = 1 << 0,
    PIN1 = 1 << 1,
    PIN2 = 1 << 2,
    PIN3 = 1 << 3,
    PIN4 = 1 << 4,
    PIN5 = 1 << 5,
    PIN6 = 1 << 6,
    PIN7 = 1 << 7,
}

pub enum Out {
    Lo,
    Hi,
}

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
            if GPIO_CONSTRUCTED == false {
                GPIO_CONSTRUCTED = true;
                GPIO {
                    p: &mut *(0x8000_0004 as *mut GPIORegisterBlock) 
                }
            }
            else { 
                panic!("You may construct only one instance of GPIO.")
            }
        }
    }

    pub fn enable_input(&mut self, pin: Pin)
    {
        unsafe {
            let mut curr: u32 = self.p.data_dir.read();
            match pin {
                Pin::PIN0 => curr &= !(Pin::PIN0 as u32),
                Pin::PIN1 => curr &= !(Pin::PIN1 as u32),
                Pin::PIN2 => curr &= !(Pin::PIN2 as u32),
                Pin::PIN3 => curr &= !(Pin::PIN3 as u32),
                Pin::PIN4 => curr &= !(Pin::PIN4 as u32),
                Pin::PIN5 => curr &= !(Pin::PIN5 as u32),
                Pin::PIN6 => curr &= !(Pin::PIN6 as u32),
                Pin::PIN7 => curr &= !(Pin::PIN7 as u32),
            }
            self.p.data_dir.write(curr);
        }
    }

    pub fn enable_inputs(&mut self, pins: u32) {
        if pins > U8_MAX
        {
            panic!("Pins must be of type u8.")
        }
        unsafe {
            let mut curr: u32 = self.p.data_dir.read();
            curr &= !pins;
            self.p.data_dir.write(curr);
        }
    }

    pub fn read_input(&self, pin: Pin) -> u32 {
        match pin {
            Pin::PIN0 => self.p.data.read() & (Pin::PIN0 as u32),
            Pin::PIN1 => self.p.data.read() & (Pin::PIN1 as u32),
            Pin::PIN2 => self.p.data.read() & (Pin::PIN2 as u32),
            Pin::PIN3 => self.p.data.read() & (Pin::PIN3 as u32),
            Pin::PIN4 => self.p.data.read() & (Pin::PIN4 as u32),
            Pin::PIN5 => self.p.data.read() & (Pin::PIN5 as u32),
            Pin::PIN6 => self.p.data.read() & (Pin::PIN6 as u32),
            Pin::PIN7 => self.p.data.read() & (Pin::PIN7 as u32),
        }
    }

    pub fn read_inputs(&self, pins: u32) -> u32 {
        if pins > U8_MAX
        {
            panic!("Pins must be of type u8.")
        }
        self.p.data.read() & pins
    }

    pub fn enable_output(&mut self, pin: Pin) {
        unsafe {
            let mut curr: u32 = self.p.data_dir.read();
            match pin {
                Pin::PIN0 => curr |= Pin::PIN0 as u32,
                Pin::PIN1 => curr |= Pin::PIN1 as u32,
                Pin::PIN2 => curr |= Pin::PIN2 as u32,
                Pin::PIN3 => curr |= Pin::PIN3 as u32,
                Pin::PIN4 => curr |= Pin::PIN4 as u32,
                Pin::PIN5 => curr |= Pin::PIN5 as u32,
                Pin::PIN6 => curr |= Pin::PIN6 as u32,
                Pin::PIN7 => curr |= Pin::PIN7 as u32,
            }
            self.p.data_dir.write(curr);
        }
    }

    pub fn enable_outputs(&mut self, pins: u32) {
        if pins > U8_MAX
        {
            panic!("Pins must be of type u8.")
        }
        unsafe {
            let mut curr: u32 = self.p.data_dir.read();
            curr |= pins;
            self.p.data_dir.write(curr);
        }
    }

    pub fn set_output(&mut self, pin: Pin, pin_output: Out) {
        unsafe {
            let mut curr: u32 = self.p.data.read();
            match pin {
                Pin::PIN0 => {
                    curr &= !(Pin::PIN0 as u32);
                    match pin_output {
                        Out::Lo => curr |= (Pin::PIN0 as u32) & (0 << 0),
                        Out::Hi => curr |= (Pin::PIN0 as u32) & (1 << 0),
                    }
                }
                Pin::PIN1 => {
                    curr &= !(Pin::PIN1 as u32);
                    match pin_output {
                        Out::Lo => curr |= (Pin::PIN1 as u32) & (0 << 1),
                        Out::Hi => curr |= (Pin::PIN1 as u32) & (1 << 1),
                    }
                }
                Pin::PIN2 => {
                    curr &= !(Pin::PIN2 as u32);
                    match pin_output {
                        Out::Lo => curr |= (Pin::PIN2 as u32) & (0 << 2),
                        Out::Hi => curr |= (Pin::PIN2 as u32) & (1 << 2),
                    }
                }
                Pin::PIN3 => {
                    curr &= !(Pin::PIN3 as u32);
                    match pin_output {
                        Out::Lo => curr |= (Pin::PIN3 as u32) & (0 << 3),
                        Out::Hi => curr |= (Pin::PIN3 as u32) & (1 << 3),
                    }
                }
                Pin::PIN4 => {
                    curr &= !(Pin::PIN4 as u32);
                    match pin_output {
                        Out::Lo => curr |= (Pin::PIN4 as u32) & (0 << 4),
                        Out::Hi => curr |= (Pin::PIN4 as u32) & (1 << 4),
                    }
                }
                Pin::PIN5 => {
                    curr &= !(Pin::PIN5 as u32);
                    match pin_output {
                        Out::Lo => curr |= (Pin::PIN5 as u32) & (0 << 5),
                        Out::Hi => curr |= (Pin::PIN5 as u32) & (1 << 5),
                    }
                }
                Pin::PIN6 => {
                    curr &= !(Pin::PIN6 as u32);
                    match pin_output {
                        Out::Lo => curr |= (Pin::PIN6 as u32) & (0 << 6),
                        Out::Hi => curr |= (Pin::PIN6 as u32) & (1 << 6),
                    }
                }
                Pin::PIN7 => {
                    curr &= !(Pin::PIN7 as u32);
                    match pin_output {
                        Out::Lo => curr |= (Pin::PIN7 as u32) & (0 << 7),
                        Out::Hi => curr |= (Pin::PIN7 as u32) & (1 << 7),
                    }
                }
            }
            self.p.data.write(curr);
        }
    }

    pub fn set_outputs(&mut self, pins: u32, pin_outputs: u32) {
        if pins > U8_MAX || pin_outputs > U8_MAX
        {
            panic!("Pins must be of type u8.")
        }
        unsafe {
            let mut curr: u32 = self.p.data.read();
            curr &= !pins;
            curr |= pins & pin_outputs;
            self.p.data.write(curr);
        }
    }

    pub fn enable_interrupt_posedge(&mut self, pin: Pin) {
        unsafe {
            let mut curr: u32 = self.p.neg_edge.read();
            match pin {
                Pin::PIN0 => {
                    curr &= !(Pin::PIN0 as u32);
                    self.p.neg_edge.write(curr);
                    curr = self.p.pos_edge.read();
                    curr |= Pin::PIN0 as u32;
                    self.p.pos_edge.write(curr);
                    curr = self.p.intr_en.read();
                    curr |= Pin::PIN0 as u32;
                }
                Pin::PIN1 => {
                    curr &= !(Pin::PIN1 as u32);
                    self.p.neg_edge.write(curr);
                    curr = self.p.pos_edge.read();
                    curr |= Pin::PIN1 as u32;
                    self.p.pos_edge.write(curr);
                    curr = self.p.intr_en.read();
                    curr |= Pin::PIN1 as u32;
                }
                Pin::PIN2 => {
                    curr &= !(Pin::PIN2 as u32);
                    self.p.neg_edge.write(curr);
                    curr = self.p.pos_edge.read();
                    curr |= Pin::PIN2 as u32;
                    self.p.pos_edge.write(curr);
                    curr = self.p.intr_en.read();
                    curr |= Pin::PIN2 as u32;
                }
                Pin::PIN3 => {
                    curr &= !(Pin::PIN3 as u32);
                    self.p.neg_edge.write(curr);
                    curr = self.p.pos_edge.read();
                    curr |= Pin::PIN3 as u32;
                    self.p.pos_edge.write(curr);
                    curr = self.p.intr_en.read();
                    curr |= Pin::PIN3 as u32;
                }
                Pin::PIN4 => {
                    curr &= !(Pin::PIN4 as u32);
                    self.p.neg_edge.write(curr);
                    curr = self.p.pos_edge.read();
                    curr |= Pin::PIN4 as u32;
                    self.p.pos_edge.write(curr);
                    curr = self.p.intr_en.read();
                    curr |= Pin::PIN4 as u32;
                }
                Pin::PIN5 => {
                    curr &= !(Pin::PIN5 as u32);
                    self.p.neg_edge.write(curr);
                    curr = self.p.pos_edge.read();
                    curr |= Pin::PIN5 as u32;
                    self.p.pos_edge.write(curr);
                    curr = self.p.intr_en.read();
                    curr |= Pin::PIN5 as u32;
                }
                Pin::PIN6 => {
                    curr &= !(Pin::PIN6 as u32);
                    self.p.neg_edge.write(curr);
                    curr = self.p.pos_edge.read();
                    curr |= Pin::PIN6 as u32;
                    self.p.pos_edge.write(curr);
                    curr = self.p.intr_en.read();
                    curr |= Pin::PIN6 as u32;
                }
                Pin::PIN7 => {
                    curr &= !(Pin::PIN7 as u32);
                    self.p.neg_edge.write(curr);
                    curr = self.p.pos_edge.read();
                    curr |= Pin::PIN7 as u32;
                    self.p.pos_edge.write(curr);
                    curr = self.p.intr_en.read();
                    curr |= Pin::PIN7 as u32;
                }
            }
            self.p.intr_en.write(curr);
        }
    }

    pub fn enable_interrupts_posedge(&mut self, pins: u32) {
        if pins > U8_MAX
        {
            panic!("Pins must be of type u8.")
        }
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

    pub fn disable_interrupt_posedge(&mut self, pin: Pin) {
        unsafe {
            let mut curr: u32 = self.p.intr_en.read();
            match pin {
                Pin::PIN0 => {
                    curr &= !(Pin::PIN0 as u32);
                    self.p.intr_en.write(curr);
                    curr = self.p.pos_edge.read();
                    curr &= !(Pin::PIN0 as u32);
                }
                Pin::PIN1 => {
                    curr &= !(Pin::PIN1 as u32);
                    self.p.intr_en.write(curr);
                    curr = self.p.pos_edge.read();
                    curr &= !(Pin::PIN1 as u32);
                }
                Pin::PIN2 => {
                    curr &= !(Pin::PIN2 as u32);
                    self.p.intr_en.write(curr);
                    curr = self.p.pos_edge.read();
                    curr &= !(Pin::PIN2 as u32);
                }
                Pin::PIN3 => {
                    curr &= !(Pin::PIN3 as u32);
                    self.p.intr_en.write(curr);
                    curr = self.p.pos_edge.read();
                    curr &= !(Pin::PIN3 as u32);
                }
                Pin::PIN4 => {
                    curr &= !(Pin::PIN4 as u32);
                    self.p.intr_en.write(curr);
                    curr = self.p.pos_edge.read();
                    curr &= !(Pin::PIN4 as u32);
                }
                Pin::PIN5 => {
                    curr &= !(Pin::PIN5 as u32);
                    self.p.intr_en.write(curr);
                    curr = self.p.pos_edge.read();
                    curr &= !(Pin::PIN5 as u32);
                }
                Pin::PIN6 => {
                    curr &= !(Pin::PIN6 as u32);
                    self.p.intr_en.write(curr);
                    curr = self.p.pos_edge.read();
                    curr &= !(Pin::PIN6 as u32);
                }
                Pin::PIN7 => {
                    curr &= !(Pin::PIN7 as u32);
                    self.p.intr_en.write(curr);
                    curr = self.p.pos_edge.read();
                    curr &= !(Pin::PIN7 as u32);
                }
            }
            self.p.pos_edge.write(curr);
        }
    }

    pub fn disable_interrupts_posedge(&mut self, pins: u32) {
        if pins > U8_MAX
        {
            panic!("Pins must be of type u8.")
        }
        unsafe {
            let mut curr: u32 = self.p.intr_en.read();
            curr &= !pins;
            self.p.intr_en.write(curr);
            curr = self.p.pos_edge.read();
            curr &= !pins;
            self.p.pos_edge.write(curr);
        }
    }

    pub fn clear_interrupt(&mut self, pin: Pin) {
        unsafe {
            let mut curr: u32 = self.p.intr_clr.read();
            match pin {
                Pin::PIN0 => curr |= Pin::PIN0 as u32,
                Pin::PIN1 => curr |= Pin::PIN1 as u32,
                Pin::PIN2 => curr |= Pin::PIN2 as u32,
                Pin::PIN3 => curr |= Pin::PIN3 as u32,
                Pin::PIN4 => curr |= Pin::PIN4 as u32,
                Pin::PIN5 => curr |= Pin::PIN5 as u32,
                Pin::PIN6 => curr |= Pin::PIN6 as u32,
                Pin::PIN7 => curr |= Pin::PIN7 as u32,
            }
            self.p.intr_clr.write(curr);
        }
    }

    pub fn clear_interrupts(&mut self, pins: u32) {
        if pins > U8_MAX
        {
            panic!("Pins must be of type u8.")
        }
        unsafe {
            let mut curr: u32 = self.p.intr_clr.read();
            curr |= pins;
            self.p.intr_clr.write(curr);
        }
    }

    pub fn interrupt_status(&self, pin: Pin) -> u32 {
        match pin {
            Pin::PIN0 => self.p.intr_sts.read() & (Pin::PIN0 as u32),
            Pin::PIN1 => self.p.intr_sts.read() & (Pin::PIN1 as u32),
            Pin::PIN2 => self.p.intr_sts.read() & (Pin::PIN2 as u32),
            Pin::PIN3 => self.p.intr_sts.read() & (Pin::PIN3 as u32),
            Pin::PIN4 => self.p.intr_sts.read() & (Pin::PIN4 as u32),
            Pin::PIN5 => self.p.intr_sts.read() & (Pin::PIN5 as u32),
            Pin::PIN6 => self.p.intr_sts.read() & (Pin::PIN6 as u32),
            Pin::PIN7 => self.p.intr_sts.read() & (Pin::PIN7 as u32),
        }
    }

    pub fn interrupts_status(&self, pins: u32) -> u32 {
        if pins > U8_MAX
        {
            panic!("Pins must be of type u8.")
        }
        self.p.intr_sts.read() & pins
    }
}