// Chip Frequency
pub const CHIP_FREQ: u32 = 100000000;

// Assorted Helper Functions
pub fn gpion(pin: u32) -> u32 {
    1 << pin
}

pub fn pwmn(channel: u32) -> u32 {
    channel * 0x0C
}

pub fn rounding_division(dividend: u32, divisor: u32) -> u32 {
    if divisor == 0 { return 0; }
    (dividend + (divisor / 2)) / divisor
}

pub fn tim_tcn(channel: u32) -> u32 {
    0x80020000 + 0x28 + (0x4 * channel)
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