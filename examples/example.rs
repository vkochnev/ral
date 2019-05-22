#![no_std]
use ral::Register;
use crate::stm32::peripherals::gpioa::moder::Moder;

mod stm32;

fn main() {
    let gpioa = stm32::peripherals::gpioa();
    let mut moder = gpioa.moder();
    moder.read();
    let moder0 = moder.get_moder0();

    moder.set_moder1(moder.get_moder2())
        .set_moder4(moder0 + 10)
        .set_moder10(0xFF)
        .write();

    let mut moder2 = gpioa.moder();
    moder2.set_moder2(0x0002)
        .write();

    let gpioa2 = stm32::peripherals::gpioa();
    gpioa2.moder()
        .set_moder4(42)
        .write();
}