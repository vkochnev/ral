use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned};
use syn::parse::Result;
use syn::spanned::Spanned;
use syn::{Ident, LitInt};

use crate::field::_Field;
use crate::field_type::_FieldType;
use crate::register::{_Register, _RegisterWithUses};
use crate::spanned::_Spanned;
use crate::uses::_Uses;

pub(super) fn render_register_with_uses(
    register_with_uses: _RegisterWithUses,
) -> Result<TokenStream> {
    let _RegisterWithUses(uses, register) = register_with_uses;
    let uses = render_uses(uses);
    let definition = render_register(register)?;
    Ok(quote! {
        #uses

        #definition
    })
}

pub(super) fn render_uses(uses: _Uses) -> TokenStream {
    let _Uses(uses) = uses;
    quote! {
        use core::sync::atomic::AtomicPtr;

        use core::convert::TryFrom;

        use ral::{borrow_register, init_register, return_register, value_read, value_write, R, Register, VolatileCell};

        #(#uses)*
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
    let offset = register.offset;
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
            init_register!(super::BASE_ADDRESS + #offset, #name);

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

            const RESET_MASK: Self::ValueType = #reset_mask;

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
                    value_read!(self, #mask, #offset) == 1
                }
            })
        }
        _FieldType::Primitive(ty) => {
            let method_name = format_ident!("get_{}", field.name);
            Ok(quote! {
                #[doc = #description]
                #[inline]
                pub fn #method_name(&self) -> #ty {
                    value_read!(self, #mask, #offset) as #ty
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
                    <#ty as TryFrom<<Self as Register>::ValueType>>::try_from(value_read!(self, #mask, #offset))
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
                    value_write!(self, #mask, #offset, value as <Self as Register>::ValueType);
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
                    value_write!(self, #mask, #offset, value as <Self as Register>::ValueType);
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
                    value_write!(self, #mask, #offset, <<Self as Register>::ValueType as TryFrom<#ty>>::try_from(value)?);
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
