#![doc = "0x00 - GPIO port mode register"]

use core::ptr;
use core::sync::atomic::AtomicPtr;
use core::sync::atomic::Ordering::SeqCst;

use vcell::VolatileCell;

use ral::{value_read, value_write, Register, R};

use crate::stm32::peripherals::gpioa::BASE_ADDRESS;

const INDEX: usize = 0;

const REGISTER: AtomicPtr<VolatileCell<<Moder as Register>::ValueType>> = AtomicPtr::new(
    (BASE_ADDRESS + 32 * INDEX) as *mut VolatileCell<<Moder as Register>::ValueType>,
);

#[doc = "0x00 - GPIO port mode register"]
pub fn moder() -> Option<Moder> {
    Moder::get()
}

pub struct Moder(R<u32, Moder>);

impl Moder {
    fn get() -> Option<Moder> {
        R::new(REGISTER.swap(ptr::null_mut(), SeqCst)).map(Moder)
    }
}

impl Drop for Moder {
    fn drop(&mut self) {
        let Moder(register) = self;
        REGISTER.swap(register.into(), SeqCst);
    }
}

impl Register for Moder {
    type RegisterType = Self;

    type ValueType = u32;

    const MASK: Self::ValueType = 0xFFFF_FFFF;

    const RESET_VALUE: Self::ValueType = 0x2800_0000;

    fn get_bits(&self) -> Self::ValueType {
        self.0.get_bits()
    }

    fn set_bits(&mut self, bits: Self::ValueType) -> &mut Self::RegisterType {
        self.0.set_bits(bits);
        self
    }

    fn reset(&mut self) -> &mut Self::RegisterType {
        self.set_bits(Self::RESET_VALUE)
    }

    fn read(&mut self) -> &mut Self::RegisterType {
        self.0.read();
        self
    }

    fn write(&mut self) -> &mut Self::RegisterType {
        self.0.write();
        self
    }
}

impl Moder {
    #[doc = "Bits 30:31 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn get_moder15(&self) -> u8 {
        value_read!(self, 3u8, 30, u8)
    }

    #[doc = "Bits 30:31 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn set_moder15(&mut self, value: u8) -> &mut Self {
        value_write!(self, 3u8, 30, value)
    }

    #[doc = "Bits 28:29 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn get_moder14(&self) -> u8 {
        value_read!(self, 3u8, 28, u8)
    }

    #[doc = "Bits 28:29 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn set_moder14(&mut self, value: u8) -> &mut Self {
        value_write!(self, 3u8, 28, value)
    }

    #[doc = "Bits 26:27 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn get_moder13(&self) -> u8 {
        value_read!(self, 3u8, 26, u8)
    }

    #[doc = "Bits 26:27 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn set_moder13(&mut self, value: u8) -> &mut Self {
        value_write!(self, 3u8, 26, value)
    }

    #[doc = "Bits 24:25 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn get_moder12(&self) -> u8 {
        value_read!(self, 3u8, 24, u8)
    }

    #[doc = "Bits 24:25 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn set_moder12(&mut self, value: u8) -> &mut Self {
        value_write!(self, 3u8, 24, value)
    }

    #[doc = "Bits 22:23 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn get_moder11(&self) -> u8 {
        value_read!(self, 3u8, 22, u8)
    }

    #[doc = "Bits 22:23 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn set_moder11(&mut self, value: u8) -> &mut Self {
        value_write!(self, 3u8, 22, value)
    }

    #[doc = "Bits 20:21 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn get_moder10(&self) -> u8 {
        value_read!(self, 3u8, 20, u8)
    }

    #[doc = "Bits 20:21 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn set_moder10(&mut self, value: u8) -> &mut Self {
        value_write!(self, 3u8, 20, value)
    }

    #[doc = "Bits 18:19 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn get_moder9(&self) -> u8 {
        value_read!(self, 3u8, 18, u8)
    }

    #[doc = "Bits 18:19 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn set_moder9(&mut self, value: u8) -> &mut Self {
        value_write!(self, 3u8, 18, value)
    }

    #[doc = "Bits 16:17 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn get_moder8(&self) -> u8 {
        value_read!(self, 3u8, 16, u8)
    }

    #[doc = "Bits 16:17 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn set_moder8(&mut self, value: u8) -> &mut Self {
        value_write!(self, 3u8, 16, value)
    }

    #[doc = "Bits 14:15 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn get_moder7(&self) -> u8 {
        value_read!(self, 3u8, 14, u8)
    }

    #[doc = "Bits 14:15 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn set_moder7(&mut self, value: u8) -> &mut Self {
        value_write!(self, 3u8, 14, value)
    }

    #[doc = "Bits 12:13 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn get_moder6(&self) -> u8 {
        value_read!(self, 3u8, 12, u8)
    }

    #[doc = "Bits 12:13 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn set_moder6(&mut self, value: u8) -> &mut Self {
        value_write!(self, 3u8, 12, value)
    }

    #[doc = "Bits 10:11 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn get_moder5(&self) -> u8 {
        value_read!(self, 3u8, 10, u8)
    }

    #[doc = "Bits 10:11 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn set_moder5(&mut self, value: u8) -> &mut Self {
        value_write!(self, 3u8, 10, value)
    }

    #[doc = "Bits 8:9 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn get_moder4(&self) -> u8 {
        value_read!(self, 3u8, 8, u8)
    }

    #[doc = "Bits 8:9 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn set_moder4(&mut self, value: u8) -> &mut Self {
        value_write!(self, 3u8, 8, value)
    }

    #[doc = "Bits 6:7 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn get_moder3(&self) -> u8 {
        value_read!(self, 3u8, 6, u8)
    }

    #[doc = "Bits 6:7 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn set_moder3(&mut self, value: u8) -> &mut Self {
        value_write!(self, 3u8, 6, value)
    }

    #[doc = "Bits 4:5 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn get_moder2(&self) -> u8 {
        value_read!(self, 3u8, 4, u8)
    }

    #[doc = "Bits 4:5 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn set_moder2(&mut self, value: u8) -> &mut Self {
        value_write!(self, 3u8, 4, value)
    }

    #[doc = "Bits 2:3 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn get_moder1(&self) -> u8 {
        value_read!(self, 3u8, 2, u8)
    }

    #[doc = "Bits 2:3 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn set_moder1(&mut self, value: u8) -> &mut Self {
        value_write!(self, 3u8, 2, value)
    }

    #[doc = "Bits 0:1 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn get_moder0(&self) -> u8 {
        value_read!(self, 3u8, 0, u8)
    }

    #[doc = "Bits 0:1 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn set_moder0(&mut self, value: u8) -> &mut Self {
        value_write!(self, 3u8, 0, value)
    }
}
