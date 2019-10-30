#![no_std]

use core::marker::{Copy, PhantomData};
use core::ops::{BitAnd, BitOr, Not};
use core::ptr::{null_mut, NonNull};
use core::sync::atomic::AtomicPtr;
use core::sync::atomic::Ordering::SeqCst;

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

#[macro_export]
macro_rules! init_register {
    ($a: expr, $t: tt) => {
        AtomicPtr::new($a as *mut VolatileCell<<$t as Register>::ValueType>)
    };
}

pub fn borrow_register<ValueType, RegisterType>(
    holder: &AtomicPtr<VolatileCell<ValueType>>,
) -> Option<R<ValueType, RegisterType>>
where
    ValueType:
        Copy + Not<Output = ValueType> + BitAnd<Output = ValueType> + BitOr<Output = ValueType>,
    RegisterType: Register<RegisterType = RegisterType, ValueType = ValueType>,
{
    R::new(holder.swap(null_mut(), SeqCst))
}

pub fn return_register<ValueType, RegisterType>(
    holder: &AtomicPtr<VolatileCell<ValueType>>,
    register: &mut R<ValueType, RegisterType>,
) where
    ValueType:
        Copy + Not<Output = ValueType> + BitAnd<Output = ValueType> + BitOr<Output = ValueType>,
    RegisterType: Register<RegisterType = RegisterType, ValueType = ValueType>,
{
    holder.swap(register.into(), SeqCst);
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Testr(R<u32, Testr>);

    impl Register for Testr {
        type RegisterType = Self;

        type ValueType = u32;

        const MASK: Self::ValueType = 0xF3FF_FFFF;

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

    impl Testr {
        #[inline]
        pub fn get_testr_30_31(&self) -> u8 {
            value_read!(self, 3u8, 30, u8)
        }

        pub fn set_testr_30_31(&mut self, value: u8) -> &mut Self {
            value_write!(self, 3u8, 30, value)
        }

        #[inline]
        pub fn get_testr_0_3(&self) -> u8 {
            value_read!(self, 0xFu8, 0, u8)
        }

        pub fn set_testr_0_3(&mut self, value: u8) -> &mut Self {
            value_write!(self, 0xFu8, 0, value)
        }
    }

    #[test]
    fn test_register_access() {
        let mut registry_data: u32 = 0x89AB_CDEF;

        let register_holder: AtomicPtr<VolatileCell<<Testr as Register>::ValueType>> =
            init_register!(&mut registry_data as *mut u32, Testr);
        let mut register_some: Option<R<u32, Testr>> = borrow_register(&register_holder);
        let register_none: Option<R<u32, Testr>> = borrow_register(&register_holder);
        assert!(register_some.is_some());
        assert!(register_none.is_none());
        return_register(&register_holder, register_some.as_mut().unwrap());
        let mut register: Option<R<u32, Testr>> = borrow_register(&register_holder);
        assert!(register.is_some());
        return_register(&register_holder, register.as_mut().unwrap());
    }

    #[test]
    fn test_get() {
        let mut registry_data: u32 = 0x8765_4321;

        let register_holder: AtomicPtr<VolatileCell<<Testr as Register>::ValueType>> =
            init_register!(&mut registry_data as *mut u32, Testr);
        let mut register = borrow_register(&register_holder)
            .map(Testr)
            .unwrap();
        register.read();
        assert_eq!(register.get_testr_0_3(), 0x01);
        assert_eq!(register.get_testr_30_31(), 0x02);
    }

    #[test]
    fn test_set() {
        let mut registry_data: u32 = 0x8765_4321;

        let register_holder: AtomicPtr<VolatileCell<<Testr as Register>::ValueType>> =
            init_register!(&mut registry_data as *mut u32, Testr);
        let mut register = borrow_register(&register_holder)
            .map(Testr)
            .unwrap();
        register.set_testr_0_3(0xDAu8);
        register.set_testr_30_31(0x0Au8);
        register.write();
        let Testr(r) = &mut register;
        return_register(&register_holder, r);
        assert_eq!(registry_data, 0x8000_000A | Testr::RESET_VALUE);
    }

    #[test]
    fn test_update() {
        let mut registry_data: u32 = 0x8765_4321;

        let register_holder: AtomicPtr<VolatileCell<<Testr as Register>::ValueType>> =
            init_register!(&mut registry_data as *mut u32, Testr);
        let mut register = borrow_register(&register_holder)
            .map(Testr)
            .unwrap();
        register.read();
        register.set_testr_0_3(0xDAu8);
        register.set_testr_30_31(0x01u8);
        register.write();
        let Testr(r) = &mut register;
        return_register(&register_holder, r);
        assert_eq!(registry_data, 0x4B65_432A);
    }

    #[test]
    fn test_reset() {
        let mut registry_data: u32 = 0x8765_4321;

        let register_holder: AtomicPtr<VolatileCell<<Testr as Register>::ValueType>> =
            init_register!(&mut registry_data as *mut u32, Testr);
        let mut register = borrow_register(&register_holder)
            .map(Testr)
            .unwrap();
        register.read();
        register.reset();
        register.write();
        let Testr(r) = &mut register;
        return_register(&register_holder, r);
        assert_eq!(registry_data, Testr::RESET_VALUE);
    }
}
