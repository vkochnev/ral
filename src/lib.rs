use vcell::VolatileCell;
use std::ops::{BitAnd, Not, BitOr};

pub trait Value: Copy + Not<Output=Self> + BitAnd<Output=Self> + BitOr<Output=Self> {
    //only need other traits
}

impl Value for u8 {}

impl Value for u16 {}

impl Value for u32 {}

impl Value for u64 {}

// register
pub fn register<'a, V: Value>(cell: &'a VolatileCell<V>, mask: V, default_bits: V) -> impl Register<Value=V> + 'a {
    R {
        register: cell,
        mask,
        default_bits,
        bits: default_bits
    }
}

pub struct R<'a, V: Value> {
    register: &'a VolatileCell<V>,
    mask: V,
    default_bits: V,
    bits: V
}

pub trait Register {
    type Value;

    #[doc = r" Get value of the register as raw bits"]
    #[inline]
    fn get_bits(&self) -> Self::Value;

    #[doc = r" Set value of the register as raw bits"]
    #[inline]
    fn set_bits(&mut self, bits: Self::Value) -> &mut Self;

    #[doc = r" Reset value of the register to default"]
    #[inline]
    fn reset(&mut self) -> &mut Self;

    #[doc = r" Loads value from the register"]
    #[inline]
    fn read(&mut self) -> &mut Self;

    #[doc = r" Writes value to the register"]
    #[inline]
    fn write(&mut self) -> &mut Self;
}

impl <'a, V: Value> Register for R<'a, V> {
    type Value = V;

    fn get_bits(&self) -> Self::Value {
        self.bits
    }

    fn set_bits(&mut self, bits: Self::Value) -> &mut Self {
        self.bits = bits;
        self
    }

    fn reset(&mut self) -> &mut Self {
        self.bits = self.default_bits;
        self
    }

    fn read(&mut self) -> &mut Self {
        self.bits = self.register.get();
        self
    }

    fn write(&mut self) -> &mut Self {
        self.register.set((!self.mask & self.default_bits) | (self.mask & self.bits));
        self
    }
}

#[doc = r"Extract specific bits from register value"]
#[macro_export]
macro_rules! value_read {
    ($r:expr, $m:expr, $o:expr, $t:ty) => ((Register::get_bits($r) >> $o) as $t & $m);
}

#[doc = r"Set specific bits to register value"]
#[macro_export]
macro_rules! value_write {
    ($r:expr, $m:expr, $o:expr, $t:ty, $v:expr) => (Register::set_bits($r, (Register::get_bits($r) & !($m as $t << $o)) | (($v & $m) as $t << $o)))
}