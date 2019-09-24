#![no_std]
extern crate alloc;

use ral::Register;

use crate::stm32::peripherals::gpioa::moder;

mod stm32;

fn main() {
    let mut moder = moder::moder().unwrap();
    moder.read();
    let moder0 = moder.get_moder0();

    moder
        .set_moder1(moder.get_moder2())
        .set_moder4(moder0 + 10)
        .set_moder10(0xFF)
        .write();
}
