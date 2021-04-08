#![doc = "Types for cpu32"]

use std::convert::TryFrom;
use std::fmt::{Debug, Display, Formatter};

use crate::cpu32::types::EnumType::{One, Three, Two};

#[derive(Debug, PartialEq, Eq)]
pub enum EnumType {
    One,
    Two,
    Three,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct UnsupportedError<T: Display> {
    value: T,
}

impl<T: Display> Display for UnsupportedError<T> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&format!("Unsupported value [{}]", self.value), fmt)
    }
}

impl TryFrom<u32> for EnumType {
    type Error = UnsupportedError<u32>;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(One),
            2 => Ok(Two),
            3 => Ok(Three),
            unsupported => Err(UnsupportedError { value: unsupported }),
        }
    }
}

impl TryFrom<EnumType> for u32 {
    type Error = ();

    fn try_from(value: EnumType) -> Result<Self, Self::Error> {
        match value {
            One => Ok(1),
            Two => Ok(2),
            Three => Ok(3),
        }
    }
}
