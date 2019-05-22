use ral::{Register, value_read, value_write};
use vcell::VolatileCell;

const MASK: u32 = 0xFFFFFFFF;

const RESET_VALUE: u32 = 671088640;

pub (crate) fn moder(cell: &VolatileCell<u32>) -> impl Moder<Value=u32> + '_ {
    ral::register(cell, MASK, RESET_VALUE)
}

pub trait Moder: Register<Value=u32> {
    #[doc = "Bits 30:31 - Port x configuration bits (y = 0..15)"]
    fn get_moder15(&self) -> u8;

    #[doc = "Bits 30:31 - Port x configuration bits (y = 0..15)"]
    fn set_moder15(&mut self, value: u8) -> &mut Self;

    #[doc = "Bits 28:29 - Port x configuration bits (y = 0..15)"]
    fn get_moder14(&self) -> u8;

    #[doc = "Bits 28:29 - Port x configuration bits (y = 0..15)"]
    fn set_moder14(&mut self, value: u8) -> &mut Self;

    #[doc = "Bits 26:27 - Port x configuration bits (y = 0..15)"]
    fn get_moder13(&self) -> u8;

    #[doc = "Bits 26:27 - Port x configuration bits (y = 0..15)"]
    fn set_moder13(&mut self, value: u8) -> &mut Self;

    #[doc = "Bits 24:25 - Port x configuration bits (y = 0..15)"]
    fn get_moder12(&self) -> u8;

    #[doc = "Bits 24:25 - Port x configuration bits (y = 0..15)"]
    fn set_moder12(&mut self, value: u8) -> &mut Self;

    #[doc = "Bits 22:23 - Port x configuration bits (y = 0..15)"]
    fn get_moder11(&self) -> u8;

    #[doc = "Bits 22:23 - Port x configuration bits (y = 0..15)"]
    fn set_moder11(&mut self, value: u8) -> &mut Self;

    #[doc = "Bits 20:21 - Port x configuration bits (y = 0..15)"]
    fn get_moder10(&self) -> u8;

    #[doc = "Bits 20:21 - Port x configuration bits (y = 0..15)"]
    fn set_moder10(&mut self, value: u8) -> &mut Self;

    #[doc = "Bits 18:19 - Port x configuration bits (y = 0..15)"]
    fn get_moder9(&self) -> u8;

    #[doc = "Bits 18:19 - Port x configuration bits (y = 0..15)"]
    fn set_moder9(&mut self, value: u8) -> &mut Self;

    #[doc = "Bits 16:17 - Port x configuration bits (y = 0..15)"]
    fn get_moder8(&self) -> u8;

    #[doc = "Bits 16:17 - Port x configuration bits (y = 0..15)"]
    fn set_moder8(&mut self, value: u8) -> &mut Self;

    #[doc = "Bits 14:15 - Port x configuration bits (y = 0..15)"]
    fn get_moder7(&self) -> u8;

    #[doc = "Bits 14:15 - Port x configuration bits (y = 0..15)"]
    fn set_moder7(&mut self, value: u8) -> &mut Self;

    #[doc = "Bits 12:13 - Port x configuration bits (y = 0..15)"]
    fn get_moder6(&self) -> u8;

    #[doc = "Bits 12:13 - Port x configuration bits (y = 0..15)"]
    fn set_moder6(&mut self, value: u8) -> &mut Self;

    #[doc = "Bits 10:11 - Port x configuration bits (y = 0..15)"]
    fn get_moder5(&self) -> u8;

    #[doc = "Bits 10:11 - Port x configuration bits (y = 0..15)"]
    fn set_moder5(&mut self, value: u8) -> &mut Self;

    #[doc = "Bits 8:9 - Port x configuration bits (y = 0..15)"]
    fn get_moder4(&self) -> u8;

    #[doc = "Bits 8:9 - Port x configuration bits (y = 0..15)"]
    fn set_moder4(&mut self, value: u8) -> &mut Self;

    #[doc = "Bits 6:7 - Port x configuration bits (y = 0..15)"]
    fn get_moder3(&self) -> u8;

    #[doc = "Bits 6:7 - Port x configuration bits (y = 0..15)"]
    fn set_moder3(&mut self, value: u8) -> &mut Self;

    #[doc = "Bits 4:5 - Port x configuration bits (y = 0..15)"]
    fn get_moder2(&self) -> u8;

    #[doc = "Bits 4:5 - Port x configuration bits (y = 0..15)"]
    fn set_moder2(&mut self, value: u8) -> &mut Self;

    #[doc = "Bits 2:3 - Port x configuration bits (y = 0..15)"]
    fn get_moder1(&self) -> u8;

    #[doc = "Bits 2:3 - Port x configuration bits (y = 0..15)"]
    fn set_moder1(&mut self, value: u8) -> &mut Self;

    #[doc = "Bits 0:1 - Port x configuration bits (y = 0..15)"]
    fn get_moder0(&self) -> u8;

    #[doc = "Bits 0:1 - Port x configuration bits (y = 0..15)"]
    fn set_moder0(&mut self, value: u8) -> &mut Self;
}

impl<T: Register<Value=u32>> Moder for T {
    #[doc = "Bits 30:31 - Port x configuration bits (y = 0..15)"]
    #[inline]
    fn get_moder15(&self) -> u8 {
        value_read!(self, 3u8, 30, u8)
    }

    #[doc = "Bits 30:31 - Port x configuration bits (y = 0..15)"]
    #[inline]
    fn set_moder15(&mut self, value: u8) -> &mut Self {
        value_write!(self, 3u8, 30, u32, value)
    }

    #[doc = "Bits 28:29 - Port x configuration bits (y = 0..15)"]
    #[inline]
    fn get_moder14(&self) -> u8 {
        value_read!(self, 3u8, 28, u8)
    }

    #[doc = "Bits 28:29 - Port x configuration bits (y = 0..15)"]
    #[inline]
    fn set_moder14(&mut self, value: u8) -> &mut Self {
        value_write!(self, 3u8, 28, u32, value)
    }

    #[doc = "Bits 26:27 - Port x configuration bits (y = 0..15)"]
    #[inline]
    fn get_moder13(&self) -> u8 {
        value_read!(self, 3u8, 26, u8)
    }

    #[doc = "Bits 26:27 - Port x configuration bits (y = 0..15)"]
    #[inline]
    fn set_moder13(&mut self, value: u8) -> &mut Self {
        value_write!(self, 3u8, 26, u32, value)
    }

    #[doc = "Bits 24:25 - Port x configuration bits (y = 0..15)"]
    #[inline]
    fn get_moder12(&self) -> u8 {
        value_read!(self, 3u8, 24, u8)
    }

    #[doc = "Bits 24:25 - Port x configuration bits (y = 0..15)"]
    #[inline]
    fn set_moder12(&mut self, value: u8) -> &mut Self {
        value_write!(self, 3u8, 24, u32, value)
    }

    #[doc = "Bits 22:23 - Port x configuration bits (y = 0..15)"]
    #[inline]
    fn get_moder11(&self) -> u8 {
        value_read!(self, 3u8, 22, u8)
    }

    #[doc = "Bits 22:23 - Port x configuration bits (y = 0..15)"]
    #[inline]
    fn set_moder11(&mut self, value: u8) -> &mut Self {
        value_write!(self, 3u8, 22, u32, value)
    }

    #[doc = "Bits 20:21 - Port x configuration bits (y = 0..15)"]
    #[inline]
    fn get_moder10(&self) -> u8 {
        value_read!(self, 3u8, 20, u8)
    }

    #[doc = "Bits 20:21 - Port x configuration bits (y = 0..15)"]
    #[inline]
    fn set_moder10(&mut self, value: u8) -> &mut Self {
        value_write!(self, 3u8, 20, u32, value)
    }

    #[doc = "Bits 18:19 - Port x configuration bits (y = 0..15)"]
    #[inline]
    fn get_moder9(&self) -> u8 {
        value_read!(self, 3u8, 18, u8)
    }

    #[doc = "Bits 18:19 - Port x configuration bits (y = 0..15)"]
    #[inline]
    fn set_moder9(&mut self, value: u8) -> &mut Self {
        value_write!(self, 3u8, 18, u32, value)
    }

    #[doc = "Bits 16:17 - Port x configuration bits (y = 0..15)"]
    #[inline]
    fn get_moder8(&self) -> u8 {
        value_read!(self, 3u8, 16, u8)
    }

    #[doc = "Bits 16:17 - Port x configuration bits (y = 0..15)"]
    #[inline]
    fn set_moder8(&mut self, value: u8) -> &mut Self {
        value_write!(self, 3u8, 16, u32, value)
    }

    #[doc = "Bits 14:15 - Port x configuration bits (y = 0..15)"]
    #[inline]
    fn get_moder7(&self) -> u8 {
        value_read!(self, 3u8, 14, u8)
    }

    #[doc = "Bits 14:15 - Port x configuration bits (y = 0..15)"]
    #[inline]
    fn set_moder7(&mut self, value: u8) -> &mut Self {
        value_write!(self, 3u8, 14, u32, value)
    }

    #[doc = "Bits 12:13 - Port x configuration bits (y = 0..15)"]
    #[inline]
    fn get_moder6(&self) -> u8 {
        value_read!(self, 3u8, 12, u8)
    }

    #[doc = "Bits 12:13 - Port x configuration bits (y = 0..15)"]
    #[inline]
    fn set_moder6(&mut self, value: u8) -> &mut Self {
        value_write!(self, 3u8, 12, u32, value)
    }

    #[doc = "Bits 10:11 - Port x configuration bits (y = 0..15)"]
    #[inline]
    fn get_moder5(&self) -> u8 {
        value_read!(self, 3u8, 10, u8)
    }

    #[doc = "Bits 10:11 - Port x configuration bits (y = 0..15)"]
    #[inline]
    fn set_moder5(&mut self, value: u8) -> &mut Self {
        value_write!(self, 3u8, 10, u32, value)
    }

    #[doc = "Bits 8:9 - Port x configuration bits (y = 0..15)"]
    #[inline]
    fn get_moder4(&self) -> u8 {
        value_read!(self, 3u8, 8, u8)
    }

    #[doc = "Bits 8:9 - Port x configuration bits (y = 0..15)"]
    #[inline]
    fn set_moder4(&mut self, value: u8) -> &mut Self {
        value_write!(self, 3u8, 8, u32, value)
    }

    #[doc = "Bits 6:7 - Port x configuration bits (y = 0..15)"]
    #[inline]
    fn get_moder3(&self) -> u8 {
        value_read!(self, 3u8, 6, u8)
    }

    #[doc = "Bits 6:7 - Port x configuration bits (y = 0..15)"]
    #[inline]
    fn set_moder3(&mut self, value: u8) -> &mut Self {
        value_write!(self, 3u8, 6, u32, value)
    }

    #[doc = "Bits 4:5 - Port x configuration bits (y = 0..15)"]
    #[inline]
    fn get_moder2(&self) -> u8 {
        value_read!(self, 3u8, 4, u8)
    }

    #[doc = "Bits 4:5 - Port x configuration bits (y = 0..15)"]
    #[inline]
    fn set_moder2(&mut self, value: u8) -> &mut Self {
        value_write!(self, 3u8, 4, u32, value)
    }

    #[doc = "Bits 2:3 - Port x configuration bits (y = 0..15)"]
    #[inline]
    fn get_moder1(&self) -> u8 {
        value_read!(self, 3u8, 2, u8)
    }

    #[doc = "Bits 2:3 - Port x configuration bits (y = 0..15)"]
    #[inline]
    fn set_moder1(&mut self, value: u8) -> &mut Self {
        value_write!(self, 3u8, 2, u32, value)
    }

    #[doc = "Bits 0:1 - Port x configuration bits (y = 0..15)"]
    #[inline]
    fn get_moder0(&self) -> u8 {
        value_read!(self, 3u8, 0, u8)
    }

    #[doc = "Bits 0:1 - Port x configuration bits (y = 0..15)"]
    #[inline]
    fn set_moder0(&mut self, value: u8) -> &mut Self {
        value_write!(self, 3u8, 0, u32, value)
    }
}