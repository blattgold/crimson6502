#[macro_export]
macro_rules! flag {
    (carry) => { 0b0000_0001};
    (zero) => {0b0000_0010};
    (interrupt) => {0b0000_0100};
    (decimal) => {0b0000_1000};
    (brk) => {0b0001_0000};
    (unused) => {0b0010_0000};
    (overflow) => {0b0100_0000};
    (negative) => {0b1000_0000};
}