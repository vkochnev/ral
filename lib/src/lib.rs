//! Library providing basis for low level registers access
//! Mostly for internal use by macros residing in `ral-macro`
//! For usage examples refer to [ral-macro](https://docs.rs/ral-macro) crate documentation
//! This crate re-exports macros refined in `ral-macro`, so this is the only library needed to be defined in dependencies
#![no_std]

use core::marker::{Copy, PhantomData};
use core::ops::{BitAnd, BitOr, Not};
use core::ptr::{null_mut, NonNull};
use core::sync::atomic::AtomicPtr;
use core::sync::atomic::Ordering::SeqCst;

pub use vcell::VolatileCell;

pub use ral_macro::*;

/// Register data holder, abstracts interaction with actual hardware
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
    /// Creates new instance bound to specific register
    #[inline]
    pub fn new(register: *mut VolatileCell<ValueType>) -> Option<Self> {
        NonNull::new(register).map(|ptr| Self {
            register_type: PhantomData,
            register: ptr,
            bits: RegisterType::RESET_VALUE,
        })
    }

    /// Gets cached bits
    #[inline]
    pub fn get_bits(&self) -> ValueType {
        self.bits
    }

    /// Stores bits into cache
    #[inline]
    pub fn set_bits(&mut self, bits: ValueType) {
        self.bits = bits;
    }

    /// Loads data from register to cache
    #[inline]
    pub fn read(&mut self) {
        unsafe {
            self.bits = self.register.as_ref().get();
        }
    }

    /// Stores data from cache to register
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

/// Trait representing register, actual registers are to implement this
pub trait Register {
    /// Actual register type for operations chaining
    type RegisterType: Register;

    /// Value type, expected to be one of `u8`, `u16`, `u32` or `u64`
    type ValueType: Copy
        + Not<Output = Self::ValueType>
        + BitAnd<Output = Self::ValueType>
        + BitOr<Output = Self::ValueType>;

    /// Reset mask
    const MASK: Self::ValueType;

    /// Reset value
    const RESET_VALUE: Self::ValueType;

    /// Get value of the register as raw bits
    fn get_bits(&self) -> Self::ValueType;

    /// Set value of the register as raw bits
    fn set_bits(&mut self, bits: Self::ValueType) -> &mut Self::RegisterType;

    /// Reset value of the register to default
    fn reset(&mut self) -> &mut Self::RegisterType;

    /// Loads value from the register
    fn read(&mut self) -> &mut Self::RegisterType;

    /// Writes value to the register
    fn write(&mut self) -> &mut Self::RegisterType;
}

/// Extract specific bits from register value
#[macro_export]
macro_rules! value_read {
    ($r: expr, $m: expr, $o: expr) => {
        ((Register::get_bits($r) >> $o) & $m)
    };
}

/// Set specific bits to register value
#[macro_export]
macro_rules! value_write {
    ($r: expr, $m: expr, $o: expr, $v: expr) => {
        Register::set_bits(
            $r,
            (Register::get_bits($r) & !($m << $o)) | ((($v) & $m) << $o),
        );
    };
}

/// Init register
#[macro_export]
macro_rules! init_register {
    ($a: expr, $t: tt) => {
        AtomicPtr::new($a as *mut VolatileCell<<$t as Register>::ValueType>)
    };
}

/// Exclusively borrows register
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

/// Releases register, so it can be borrowed again
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

    struct TestR(R<u32, TestR>);

    impl Register for TestR {
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

    impl TestR {
        #[inline]
        pub fn get_test_30_31(&self) -> u8 {
            value_read!(self, 0x3u32, 30) as u8
        }

        pub fn set_test_30_31(&mut self, value: u8) -> &mut Self {
            value_write!(self, 0x3u32, 30, value as <Self as Register>::ValueType);
            self
        }

        #[inline]
        pub fn get_test_0_3(&self) -> u8 {
            value_read!(self, 0xFu32, 0) as u8
        }

        pub fn set_test_0_3(&mut self, value: u8) -> &mut Self {
            value_write!(self, 0xFu32, 0, value as <Self as Register>::ValueType);
            self
        }
    }

    #[test]
    fn test_register_access() {
        let mut registry_data: u32 = 0x89AB_CDEF;

        let register_holder: AtomicPtr<VolatileCell<<TestR as Register>::ValueType>> =
            init_register!(&mut registry_data as *mut u32, TestR);
        let mut register_some: Option<R<u32, TestR>> = borrow_register(&register_holder);
        let register_none: Option<R<u32, TestR>> = borrow_register(&register_holder);
        assert!(register_some.is_some());
        assert!(register_none.is_none());
        return_register(&register_holder, register_some.as_mut().unwrap());
        let mut register: Option<R<u32, TestR>> = borrow_register(&register_holder);
        assert!(register.is_some());
        return_register(&register_holder, register.as_mut().unwrap());
    }

    #[test]
    fn test_get() {
        let mut registry_data: u32 = 0x8765_4321;

        let register_holder: AtomicPtr<VolatileCell<<TestR as Register>::ValueType>> =
            init_register!(&mut registry_data as *mut u32, TestR);
        let mut register = borrow_register(&register_holder).map(TestR).unwrap();
        register.read();
        assert_eq!(register.get_test_0_3(), 0x01);
        assert_eq!(register.get_test_30_31(), 0x02);
    }

    #[test]
    fn test_set() {
        let mut registry_data: u32 = 0x8765_4321;

        let register_holder: AtomicPtr<VolatileCell<<TestR as Register>::ValueType>> =
            init_register!(&mut registry_data as *mut u32, TestR);
        let mut register = borrow_register(&register_holder).map(TestR).unwrap();
        register.set_test_0_3(0xDAu8).set_test_30_31(0x0Au8).write();
        let TestR(r) = &mut register;
        return_register(&register_holder, r);
        assert_eq!(registry_data, 0x8000_000A | TestR::RESET_VALUE);
    }

    #[test]
    fn test_update() {
        let mut registry_data: u32 = 0x8765_4321;

        let register_holder: AtomicPtr<VolatileCell<<TestR as Register>::ValueType>> =
            init_register!(&mut registry_data as *mut u32, TestR);
        let mut register = borrow_register(&register_holder).map(TestR).unwrap();
        register
            .read()
            .set_test_0_3(0xDAu8)
            .set_test_30_31(0x01u8)
            .write();
        let TestR(r) = &mut register;
        return_register(&register_holder, r);
        assert_eq!(registry_data, 0x4B65_432A);
    }

    #[test]
    fn test_reset() {
        let mut registry_data: u32 = 0x8765_4321;

        let register_holder: AtomicPtr<VolatileCell<<TestR as Register>::ValueType>> =
            init_register!(&mut registry_data as *mut u32, TestR);
        let mut register = borrow_register(&register_holder).map(TestR).unwrap();
        register.read();
        register.reset();
        register.write();
        let TestR(r) = &mut register;
        return_register(&register_holder, r);
        assert_eq!(registry_data, TestR::RESET_VALUE);
    }
}
