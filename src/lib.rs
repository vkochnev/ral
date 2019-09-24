use core::marker::{Copy, PhantomData};
use core::ops::{BitAnd, BitOr, Not};
use std::ptr::NonNull;
use vcell::VolatileCell;

pub struct R<ValueType, RegisterType>
where
    ValueType:
        Copy + Not<Output = ValueType> + BitAnd<Output = ValueType> + BitOr<Output = ValueType>,
    RegisterType: Register<RegisterType = RegisterType, ValueType = ValueType>,
{
    register_type: PhantomData<RegisterType>,
    register: NonNull<VolatileCell<ValueType>>,
    bits: ValueType,
}

impl<ValueType, RegisterType> R<ValueType, RegisterType>
where
    ValueType:
        Copy + Not<Output = ValueType> + BitAnd<Output = ValueType> + BitOr<Output = ValueType>,
    RegisterType: Register<RegisterType = RegisterType, ValueType = ValueType>,
{
    #[inline]
    pub fn new(register: *mut VolatileCell<ValueType>) -> Option<Self> {
        NonNull::new(register).map(|ptr| Self {
            register_type: PhantomData,
            register: ptr,
            bits: RegisterType::RESET_VALUE,
        })
    }

    #[inline]
    pub fn get_bits(&self) -> ValueType {
        self.bits
    }

    #[inline]
    pub fn set_bits(&mut self, bits: ValueType) {
        self.bits = bits;
    }

    #[inline]
    pub fn read(&mut self) {
        unsafe {
            self.bits = self.register.as_ref().get();
        }
    }

    #[inline]
    pub fn write(&mut self) {
        unsafe {
            self.register.as_ref().set(
                (!RegisterType::MASK & RegisterType::RESET_VALUE)
                    | (RegisterType::MASK & self.bits),
            );
        }
    }
}

impl<ValueType, RegisterType> Into<*mut VolatileCell<ValueType>> for &mut R<ValueType, RegisterType>
where
    ValueType:
        Copy + Not<Output = ValueType> + BitAnd<Output = ValueType> + BitOr<Output = ValueType>,
    RegisterType: Register<RegisterType = RegisterType, ValueType = ValueType>,
{
    fn into(self) -> *mut VolatileCell<ValueType> {
        self.register.as_ptr()
    }
}

pub trait Register {
    type RegisterType: Register;

    type ValueType: Copy
        + Not<Output = Self::ValueType>
        + BitAnd<Output = Self::ValueType>
        + BitOr<Output = Self::ValueType>;

    const MASK: Self::ValueType;

    const RESET_VALUE: Self::ValueType;

    #[doc = r" Get value of the register as raw bits"]
    #[inline]
    fn get_bits(&self) -> Self::ValueType;

    #[doc = r" Set value of the register as raw bits"]
    #[inline]
    fn set_bits(&mut self, bits: Self::ValueType) -> &mut Self::RegisterType;

    #[doc = r" Reset value of the register to default"]
    #[inline]
    fn reset(&mut self) -> &mut Self::RegisterType;

    #[doc = r" Loads value from the register"]
    #[inline]
    fn read(&mut self) -> &mut Self::RegisterType;

    #[doc = r" Writes value to the register"]
    #[inline]
    fn write(&mut self) -> &mut Self::RegisterType;
}

#[doc = r"Extract specific bits from register value"]
#[macro_export]
macro_rules! value_read {
    ($r: expr, $m: expr, $o: expr, $t: ty) => {
        (Register::get_bits($r) >> $o) as $t & $m
    };
}

#[doc = r"Set specific bits to register value"]
#[macro_export]
macro_rules! value_write {
    ($r: expr, $m: expr, $o: expr, $v: expr) => {
        Register::set_bits(
            $r,
            (Register::get_bits($r) & !(($m as <Self as Register>::ValueType) << $o))
                | ((($v & $m) as <Self as Register>::ValueType) << $o),
        )
    };
}
