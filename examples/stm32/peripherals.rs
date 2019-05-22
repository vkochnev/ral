use crate::stm32::peripherals::gpioa::RegisterBlock;

pub mod gpioa;

#[doc = "GPIOA"]
pub fn gpioa() -> &'static RegisterBlock {
    gpioa::ptr()
}