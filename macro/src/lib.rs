//! Provides macro with DSL describing register
//!
//! # DSL and produced result
//!
//! Internally generates register description backed by [ral](https://docs.rs/ral)
//! ```
//! register! {
//!     use crate_name::types::CustomType; // Use expressions for custom types used in field declarations
//!
//!     #[access = "read-write"] // Optional register wide access specifier
//!     #[doc = "Register description"] // Optional register description
//!     reg0 { // Register name
//!         offset: 0x8, // Register offset in enclosing peripheral
//!         value_size: 32, // Register size, expected values are 8, 16, 32 or 64
//!         reset_mask: 0xFFFFFFFF, // Reset mask
//!         reset_value: 0x0, // Reset value
//!         fields: { // Optional fields
//!             #[doc = "Bits 16:31 - Read-only u16 field"] // Each field can have a description
//!             #[access = "read-only"] // Access specifier
//!             field5[16:16] as u16, // Field specification `<name>[<offset>:<width>] as <type>`
//!
//!             #[doc = "Bits 14:15 - Write-only field"]
//!             #[access = "write-only"] // Only setters will be generated
//!             field4[14:2] as u8, // Default supported types are `u8`, `u16`, `u32` and `u64`
//!
//!             #[doc = "Bits 11:13 - Read-only field"]
//!             #[access = "read-only"] // Only getters will be generated
//!             field3[11:3] as u8,
//!
//!             #[doc = "Bit 10 - Boolean field"]
//!             #[access = "read-write"] // `read-write` is a default access specifier
//!             field2[10:1] as bool, // `bool` fields are also supported by default
//!
//!             #[doc = "Bits 8:9 - Enum field"]
//!             #[access = "read-write"] // Both getters and setters will be generated
//!             field1[8:2] as CustomType, // Custom types are supported with limitations
//!
//!             #[doc = "Bits 0:7 - Read-write by default long field"]
//!             field0[0:8] as u8
//!         }
//!     }
//! }
//! ```
//!
//! Above register definition will be transformed into following code
//! ```
//! // Required uses section
//! use core::sync::atomic::AtomicPtr;
//! use core::convert::TryFrom;
//! use ral::{
//!     borrow_register, init_register, return_register, value_read, value_write, R, Register,
//!     VolatileCell,
//! };
//! use crate_name::types::CustomType;
//! const REGISTER: AtomicPtr<VolatileCell<<Reg0 as Register>::ValueType>> = AtomicPtr::new(
//!     (super::BASE_ADDRESS /* Enclosing peripheral/cluster base address */ + 0x00 /* offset */) as *mut VolatileCell<<Reg0 as Register>::ValueType>,
//! );
//! ///Register description
//! pub fn reg0() -> Option<Reg0> {
//!     borrow_register(&REGISTER).map(Reg0)
//! }
//! pub struct Reg0(R<u32, Reg0>);
//! impl Drop for Reg0 {
//!     fn drop(&mut self) {
//!         let Reg0(register) = self;
//!         return_register(&REGISTER, register);
//!     }
//! }
//! impl Register for Reg0 {
//!     type RegisterType = Self;
//!     type ValueType = u32;
//!     const RESET_MASK: Self::ValueType = 0xFFFF_FFFF;
//!     const RESET_VALUE: Self::ValueType = 0x1234_0000;
//! }
//! impl ReadableRegister for Reg0 { // Will be added if register wide access allows read
//!     fn get_bits(&self) -> Self::ValueType {
//!         self.0.get_bits()
//!     }
//!
//!     fn read(&mut self) -> &mut Self::RegisterType {
//!         self.0.read();
//!         self
//!     }
//! }
//! impl WritableRegister for Reg0 { // Will be added if register wide access allows write
//!     fn set_bits(&mut self, bits: Self::ValueType) -> &mut Self::RegisterType {
//!         self.0.set_bits(bits);
//!         self
//!     }
//!     fn reset(&mut self) -> &mut Self::RegisterType {
//!         self.set_bits(Self::RESET_VALUE)
//!     }
//!     fn write(&mut self) -> &mut Self::RegisterType {
//!         self.0.write();
//!         self
//!     }
//! }
//! impl Reg0 { // Will be added if any fields specified
//!     ///Bits 16:31 - Read-only u16 field
//!     #[inline]
//!     pub fn get_field5(&self) -> u16 {
//!         value_read!(self, 0x0000FFFFu32, 16) as u16
//!         // Will further expand into
//!         // ((self.0.get_bits() >> 16) & 0x0000FFFFu32) as u16
//!     }
//!     ///Bits 14:15 - Write-only field
//!     #[inline]
//!     pub fn set_field4(&mut self, value: u8) -> &mut Self {
//!         value_write!(self, 0x00000003u32, 14, value as <Self as Register>::ValueType);
//!         // Will further expand into
//!         // self.0.set_bits(
//!         //     (self.0.get_bits() & !(0x00000003u32 << 14))
//!         //         | ((value as <Self as Register>::ValueType & 0x00000003u32) << 14),
//!         // );
//!         self
//!     }
//!     ///Bits 11:13 - Read-only field
//!     #[inline]
//!     pub fn get_field3(&self) -> u8 {
//!         value_read!(self, 0x00000007u32, 11) as u8
//!         // Will further expand into
//!         // ((self.0.get_bits() >> 11) & 0x00000007u32) as u8
//!     }
//!     ///Bit 10 - Boolean field
//!     #[inline]
//!     pub fn is_field2_set(&self) -> bool {
//!         value_read!(self, 0x00000001u32, 10) == 1
//!         // Will further expand into
//!         // ((self.0.get_bits() >> 10) & 0x00000001u32) == 1
//!     }
//!     ///Bit 10 - Boolean field
//!     #[inline]
//!     pub fn set_field2_value(&mut self, value: bool) -> &mut Self {
//!         value_write!(self, 0x00000001u32, 10, value as <Self as Register>::ValueType);
//!         // Will further expand into
//!         // self.0.set_bits(
//!         //     (self.0.get_bits() & !(0x00000001u32 << 10))
//!         //         | ((value as <Self as Register>::ValueType & 0x00000001u32) << 10),
//!         // );
//!         self
//!     }
//!     ///Bit 10 - Boolean field
//!     #[inline]
//!     pub fn set_field2(&mut self) -> &mut Self {
//!         self.set_field2_value(true)
//!     }
//!     ///Bit 10 - Boolean field
//!     #[inline]
//!     pub fn unset_field2(&mut self) -> &mut Self {
//!         self.set_field2_value(false)
//!     }
//!     ///Bits 8:9 - Enum field
//!     #[inline]
//!     pub fn get_field1(
//!         &self,
//!     ) -> Result<CustomType, <CustomType as TryFrom<<Self as Register>::ValueType>>::Error> {
//!         <CustomType as TryFrom<<Self as Register>::ValueType>>::try_from(value_read!(self, 0x00000003u32, 8))
//!         // Will further expand into
//!         // <CustomType as TryFrom<<Self as Register>::ValueType>>::try_from(
//!         //     (self.0.get_bits() >> 8) & 0x00000003u32,
//!         // )
//!     }
//!     ///Bits 8:9 - Enum field
//!     #[inline]
//!     pub fn set_field1(
//!         &mut self,
//!         value: CustomType,
//!     ) -> Result<&mut Self, <<Self as Register>::ValueType as TryFrom<CustomType>>::Error>
//!     {
//!         value_write!(self, 0x00000003u32, 8, <<Self as Register>::ValueType as TryFrom<#ty>>::try_from(value)?);
//!         // Will further expand into
//!         // self.0.set_bits(
//!         //     (self.0.get_bits() & !(0x00000003u32 << 8))
//!         //         | ((<<Self as Register>::ValueType as TryFrom<CustomType>>::try_from(value)?
//!         //             & 0x00000003u32) << 8),
//!         // );
//!         Ok(self)
//!     }
//!     ///Bits 0:7 - Read-write by default long field
//!     #[inline]
//!     pub fn get_field0(&self) -> u8 {
//!         value_read!(self, 0x000000FFu32, 0) as u8
//!         // Will further expand into
//!         // ((self.0.get_bits() >> 0) & 0x000000FFu32) as u8
//!     }
//!     ///Bits 0:7 - Read-write by default long field
//!     #[inline]
//!     pub fn set_field0(&mut self, value: u8) -> &mut Self {
//!         value_write!(self, 0x000000FFu32, 0, value as <Self as Register>::ValueType);
//!         // Will further expand into
//!         // self.0.set_bits(
//!         //     (self.0.get_bits() & !(0x000000FFu32 << 0))
//!         //         | ((value as <Self as Register>::ValueType & 0x000000FFu32) << 0),
//!         // );
//!         self
//!     }
//! }
//! ```
//!
//! # Proposed modules structure
//!
//! Enclosing peripheral module can look like this
//! ```ignore
//! #![doc = "Peripheral description"]
//!
//! mod reg0;
//! pub use reg0::*;
//!
//! const BASE_ADDRESS: usize = 0x0000_0000; // Should be actual peripheral base address
//! ```
//!
//! Peripherals module should look like this
//! ```ignore
//! pub mod peripheral;
//! ```
//!
//! And finally `lib.rs` should look like this
//! ```ignore
//! #![doc = "Device description"]
//!
//! pub mod peripherals;
//! ```
//!
//! So the recommended module hierarchy is
//! ```ignore
//! src/
//! ├── lib.rs
//! └── peripherals/
//!     ├── mod.rs
//!     └── peripheral/
//!         ├── mod.rs
//!         ├── reg0.rs
//!         ├── reg1.rs
//!         └── cluster/
//!             ├── mod.rs
//!             ├── reg2.rs
//!             ├── reg3.rs
//!             └── reg4.rs
//! ```
//!
//! The layout also includes optional cluster module, which can look like this
//! ```ignore
//! #![doc = "Cluster description"]
//!
//! mod reg2;
//! pub use reg2::*;
//! mod reg3;
//! pub use reg3::*;
//! mod reg4;
//! pub use reg4::*;
//!
//! const BASE_ADDRESS: usize = super::BASE_ADDRESS + 0x40; // Enclosing peripheral/cluster base address plus offset
//! ```
//!
//! # Requirenments to custom types
//!
//! For read access you must implement `TryFrom<u32> for CustomType` or `From<u32> for CustomType` if your register is 32-bit
//! For write access you have to implement `TryFrom<CustomType> for u32` or `From<CustomType> for u32` respectively
//!
//! # How to use resulting library
//!
//! ```
//! use device_crate::peripherals::peripheral;
//!
//! let mut reg0 = peripheral::reg0()
//!         .unwrap() // Borrow the register
//!         .read(); // Load current data stored in register
//! let field3 = reg0.get_field3(); // Read `u8` field
//!
//! let field2 = reg0.is_field2_set(); // Read `bool` field
//! let field1: CustomType = reg0.get_field1().unwrap(); // Read field represented by `CustomType`
//! reg0
//!     .set_field0(if field2 {
//!         reg0.get_field5() as u8
//!     } else {
//!         0
//!     }) // Set `u8` field
//!     .unset_field2() // Unset `bool` field
//!     .set_field4(field3 + 10) // Set 'u8' field
//!     .set_field1(if field1 == Two { One } else { Three }).unwrap() // Set `CustomType` field
//!     .write(); // Finally write result to hardware register
//! ```
use proc_macro::TokenStream;

use syn::parse_macro_input;

use crate::register::{_Register, _RegisterWithUses};
use crate::uses::_Uses;

mod field;
mod field_type;
mod parse;
mod register;
mod render;
mod spanned;
mod uses;

/// Macro expanding into register definition and required uses
#[proc_macro]
pub fn register(item: TokenStream) -> TokenStream {
    render::render_register_with_uses(parse_macro_input!(item as _RegisterWithUses))
        .map(TokenStream::from)
        .unwrap_or_else(|err| TokenStream::from(err.to_compile_error()))
}

/// Macro expanding into required uses, might be useful when multiple registers to be defined in the same module
#[proc_macro]
pub fn register_uses(item: TokenStream) -> TokenStream {
    TokenStream::from(render::render_uses(parse_macro_input!(item as _Uses)))
}

/// Macro expanding into register definition, might be useful when multiple registers to be defined in the same module
#[proc_macro]
pub fn register_definition(item: TokenStream) -> TokenStream {
    render::render_register(parse_macro_input!(item as _Register))
        .map(TokenStream::from)
        .unwrap_or_else(|err| TokenStream::from(err.to_compile_error()))
}
