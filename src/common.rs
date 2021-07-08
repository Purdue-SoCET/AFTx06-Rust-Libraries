// Chip Frequency
pub const CHIP_FREQ: u32 = 100000000;

// APB Construction Checker
pub static mut CLINT_CONSTRUCTED: bool = false;
pub static mut GPIO_CONSTRUCTED: bool =  false;
pub static mut PLIC_CONSTRUCTED: bool =  false;
pub static mut PWM_CONSTRUCTED: bool =   false;
pub static mut TIM_CONSTRUCTED: bool =   false;

// CLINT Constants
pub const CLINT: u32 =                  0xE0000000;
pub const CLINT_MSIP: u32 =             CLINT + 0x00;
pub const CLINT_MTIME: u32 =            CLINT + 0x04;
pub const CLINT_MTIMECMP: u32 =         CLINT + 0x0C;
pub const CLINT_MSIP_DISABLE: u32 =     !(1 << 0);
pub const CLINT_MSIP_ENABLE: u32 =      1 << 0;
pub const CLINT_MSIP_MASK: u32 =        1 << 0;

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

// PWM Constants
pub const PWM: u32 =                      0x80010000;
pub const PWM_PERIOD: u32 =               PWM + 0x00;
pub const PWM_DUTY: u32 =                 PWM + 0x04;
pub const PWM_CONTROL: u32 =              PWM + 0x08;
pub const PWM_CONTROL_DISABLE: u32 =      !(1 << 0);
pub const PWM_CONTROL_ENABLE: u32 =       1 << 0;
pub const PWM_CONTROL_ACTIVE_HIGH: u32 =  !(1 << 1);
pub const PWM_CONTROL_ACTIVE_LOW: u32 =   1 << 1;
pub const PWM_CONTROL_ALIGN_LEFT: u32 =   !(1 << 2);
pub const PWM_CONTROL_ALIGN_CENTER: u32 = 1 << 2;
pub const PWM_CHANNEL_SIZE: u32 =         0x0C;
pub const PWM_MAX_FREQ: u32 =             CHIP_FREQ / 2;
pub const AFTX06_DUTY_OFFSET: u32 =       1;

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

pub fn gpion(pin: u32) -> u32 {
    1 << pin
}

pub fn pwmn(channel: u32) -> u32 {
    channel * PWM_CHANNEL_SIZE
}

pub fn rounding_division(dividend: u32, divisor: u32) -> u32 {
    if divisor == 0 { return 0; }
    (dividend + (divisor / 2)) / divisor
}

pub fn tim_tcn(channel: u32) -> u32 {
    TIM + 0x28 + (0x4 * channel)
}

pub fn tim_ios_input(channel: u32) -> u32 {
    !(1 << channel)
}

pub fn tim_ios_output(channel: u32) -> u32 {
    1 << channel
}

pub fn tim_tcr_edge_mask(channel: u32) -> u32 {
    0x101 << channel
}

pub fn tim_tcr_output_mask(channel: u32) -> u32 {
    (0x101 << 16) << channel
}

pub fn tim_tie_mask(channel: u32) -> u32 {
    1 << channel
}