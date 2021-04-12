use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned};
use syn::{Ident, LitInt};
use syn::parse::Result;
use syn::spanned::Spanned;

use crate::field::_Field;
use crate::field_type::_FieldType;
use crate::register::_Register;
use crate::spanned::_Spanned;

pub(super) fn render_register_with_uses(register: _Register) -> Result<TokenStream> {
    let uses = render_uses();
    let definition = render_register(register)?;
    Ok(quote! {
        #uses

        #definition
    })
}

pub(super) fn render_uses() -> TokenStream {
    quote! {
        use core::sync::atomic::AtomicPtr;

        use core::convert::TryFrom;

        use ral::{borrow_register, init_register, R, Register, return_register, VolatileCell};

        use super::BASE_ADDRESS;
    }
}

pub(super) fn render_register(register: _Register) -> Result<TokenStream> {
    let name = format_ident!(
        "{}",
        register.name.to_string().to_case(Case::UpperCamel),
        span = register.name.span()
    );
    let method_name = register.name;
    let description = register.description;
    let index = register.offset;
    let value_size = register.value_size;
    let value_type = format_ident!("u{}", value_size.value, span = value_size.span());
    let reset_mask = register.reset_mask;
    let value_reset = register.reset_value;
    let mut methods = Vec::<TokenStream>::new();
    let methods_iter = register
        .fields
        .into_iter()
        .map(|field| render_field(field, &value_type, value_size.value));
    for method in methods_iter {
        methods.push(method?);
    }
    Ok(quote! {
        const REGISTER: AtomicPtr<VolatileCell<<#name as Register>::ValueType>> =
            init_register!(BASE_ADDRESS + #value_size * #index, #name);

        #[doc = #description]
        pub fn #method_name() -> Option<#name> {
            borrow_register(&REGISTER).map(#name)
        }

        pub struct #name(R<#value_type, #name>);

        impl Drop for #name {
            fn drop(&mut self) {
                let #name(register) = self;
                return_register(&REGISTER, register);
            }
        }

        impl Register for #name {
            type RegisterType = Self;

            type ValueType = #value_type;

            const MASK: Self::ValueType = #reset_mask;

            const RESET_VALUE: Self::ValueType = #value_reset;

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

        impl #name {
            #(#methods)*
        }
    })
}

fn render_field(field: _Field, value_type: &Ident, value_size: u32) -> Result<TokenStream> {
    let access = field
        .access
        .as_ref()
        .map(|lit_str| lit_str.value())
        .unwrap_or(String::from("read-write"));
    match access.as_str() {
        "write-only" | "writeOnce" => render_write(&field, value_type, value_size),
        "read-only" => render_read(&field, value_type, value_size),
        _ => {
            let read = render_read(&field, value_type, value_size)?;
            let write = render_write(&field, value_type, value_size)?;
            Ok(quote! {
                #read

                #write
            })
        }
    }
}

fn render_read(field: &_Field, value_type: &Ident, value_size: u32) -> Result<TokenStream> {
    let description = &field.description;
    let offset = &field.offset;
    let mask = build_mask(&field.width, value_size);
    match &field.ty {
        _FieldType::Bool(ty) => {
            let method_name = format_ident!("is_{}_set", field.name);
            Ok(quote! {
                #[doc = #description]
                #[inline]
                pub fn #method_name(&self) -> #ty {
                    ((Register::get_bits(self) >> #offset) & 1) == 1
                }
            })
        }
        _FieldType::Primitive(ty) => {
            let method_name = format_ident!("get_{}", field.name);
            Ok(quote! {
                #[doc = #description]
                #[inline]
                pub fn #method_name(&self) -> #ty {
                    ((Register::get_bits(self) >> #offset) & #mask) as #ty
                }
            })
        }
        _FieldType::Custom(ty) => {
            let ty_span = ty.span();
            let _ = quote_spanned! {ty_span=>
                struct _AssertTryFrom where #ty: core::convert::TryFrom<#value_type>;
            };
            let method_name = format_ident!("get_{}", field.name);
            Ok(quote! {
                #[doc = #description]
                #[inline]
                pub fn #method_name(&self) -> Result<#ty, <#ty as TryFrom<<Self as Register>::ValueType>>::Error> {
                    <#ty as TryFrom<<Self as Register>::ValueType>>::try_from((Register::get_bits(self) >> #offset) & #mask)
                }
            })
        }
    }
}

fn render_write(field: &_Field, value_type: &Ident, value_size: u32) -> Result<TokenStream> {
    let description = &field.description;
    let offset = &field.offset;
    let mask = build_mask(&field.width, value_size);
    match &field.ty {
        _FieldType::Bool(ty) => {
            let method_name = format_ident!("set_{}_value", field.name);
            let method_name_set = format_ident!("set_{}", field.name);
            let method_name_unset = format_ident!("unset_{}", field.name);
            Ok(quote! {
                #[doc = #description]
                #[inline]
                pub fn #method_name(&mut self, value: #ty) -> &mut Self {
                    Register::set_bits(self, (Register::get_bits(self) & !(1 << #offset))
                        | ((if value {1} else {0}) << #offset)
                    );
                    self
                }

                #[doc = #description]
                #[inline]
                pub fn #method_name_set(&mut self) -> &mut Self {
                    self.#method_name(true)
                }

                #[doc = #description]
                #[inline]
                pub fn #method_name_unset(&mut self) -> &mut Self {
                    self.#method_name(false)
                }
            })
        }
        _FieldType::Primitive(ty) => {
            let method_name = format_ident!("set_{}", field.name);
            Ok(quote! {
                #[doc = #description]
                #[inline]
                pub fn #method_name(&mut self, value: #ty) -> &mut Self {
                    Register::set_bits(self, (Register::get_bits(self) & !(#mask << #offset))
                        | (((value as <Self as Register>::ValueType) & #mask) << #offset)
                    );
                    self
                }
            })
        }
        _FieldType::Custom(ty) => {
            let ty_span = ty.span();
            let _ = quote_spanned! {ty_span=>
                struct _AssertTryInto where #value_type: core::convert::TryFrom<#ty>;
            };
            let method_name = format_ident!("set_{}", field.name);
            Ok(quote! {
                #[doc = #description]
                #[inline]
                pub fn #method_name(&mut self, value: #ty) -> Result<&mut Self, <<Self as Register>::ValueType as TryFrom<#ty>>::Error> {
                    Register::set_bits(self, (Register::get_bits(self) & !(#mask << #offset))
                        | ((<<Self as Register>::ValueType as TryFrom<#ty>>::try_from(value)? & #mask) << #offset)
                    );
                    Ok(self)
                }
            })
        }
    }
}

fn build_mask(width: &_Spanned<u32>, value_size: u32) -> LitInt {
    let mask = format!(
        "{mask:#0width$X}u{bits}",
        mask = 2u128.pow(width.value) - 1,
        width = ((value_size / 4) + 2) as usize,
        bits = value_size
    );
    LitInt::new(mask.as_str(), width.span())
}
