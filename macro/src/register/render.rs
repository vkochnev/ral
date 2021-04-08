use std::str::FromStr;

use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote, quote_spanned};
use syn::parse::Result;
use syn::spanned::Spanned;
use syn::{Ident, LitInt};

use super::{_Field, _Register};

pub(super) fn render_register(register: _Register) -> Result<TokenStream> {
    let uses = render_uses();
    let definition = render_definition(register)?;
    Ok(quote! {
        #uses

        #definition
    })
}

pub(super) fn render_uses() -> TokenStream {
    quote! {
        use core::sync::atomic::AtomicPtr;

        use core::convert::TryFrom;

        use vcell::VolatileCell;

        use ral::{borrow_register, return_register, init_register, Register, R};

        use super::BASE_ADDRESS;
    }
}

pub(super) fn render_definition(register: _Register) -> Result<TokenStream> {
    let name = register.name;
    let method_name = format_ident!("{}", name.to_string().to_lowercase(), span = name.span());
    let description = register.description;
    let index = register.offset;
    let value_size = register.value_size;
    let value_type = format_ident!("u{}", value_size.base10_digits(), span = value_size.span());
    let reset_mask = register.reset_mask;
    let value_reset = register.reset_value;
    let register_definition = quote! {
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
    };

    let value_size = u32::from_str(value_size.base10_digits()).unwrap();
    let mut methods = Vec::<proc_macro2::TokenStream>::new();
    let methods_iter = register
        .fields
        .into_iter()
        .map(|field| render_field(field, &value_type, value_size));
    for method in methods_iter {
        methods.push(method?);
    }

    let register_impl = quote! {
        impl #name {
            #(#methods)*
        }
    };

    Ok(TokenStream::from(quote! {
        #register_definition

        #register_impl
    }))
}

fn render_field(
    field: _Field,
    value_type: &Ident,
    value_size: u32,
) -> Result<proc_macro2::TokenStream> {
    let access = field
        .access
        .as_ref()
        .map(|lit_str| lit_str.value())
        .unwrap_or(String::from("read-write"));
    let mask_length = u32::from_str(field.size.base10_digits()).map_err(|err| {
        syn::Error::new(
            field.size.span(),
            format!("Failed to parse length of mask: {}", err),
        )
    })?;
    let offset = u32::from_str(field.offset.base10_digits()).map_err(|err| {
        syn::Error::new(
            field.size.span(),
            format!("Failed to parse field offset: {}", err),
        )
    })?;
    if mask_length > value_size {
        Err(syn::Error::new(
            field.size.span(),
            format!(
                "Mask length cannot be more than value size [{} > {}]",
                mask_length, value_size
            ),
        ))?
    }
    if offset >= value_size {
        Err(syn::Error::new(
            field.offset.span(),
            format!(
                "Field offset must be less than value size [{} > {}]",
                offset, value_size
            ),
        ))?
    }
    if offset + mask_length > value_size {
        Err(syn::Error::new(
            field.size.span(),
            format!(
                "Field offset and mask length in sum cannot be more than value size [{} + {} > {}]",
                offset, mask_length, value_size
            ),
        ))?
    }
    let as_bool = if let Some(ty_name) = field.ty.path.get_ident() {
        if ty_name == "bool" {
            if mask_length != 1 {
                Err(syn::Error::new(
                    field.size.span(),
                    format!("Field size must be 1 for bool"),
                ))?
            }
            true
        } else {
            false
        }
    } else {
        Err(syn::Error::new(
            field.ty.span(),
            format!("Field type must be identifier"),
        ))?
    };
    match access.as_str() {
        "write-only" | "writeOnce" => {
            render_write(&field, mask_length, value_type, value_size, as_bool)
        }
        "read-only" => render_read(&field, mask_length, value_type, value_size, as_bool),
        _ => {
            let read = render_read(&field, mask_length, value_type, value_size, as_bool)?;
            let write = render_write(&field, mask_length, value_type, value_size, as_bool)?;
            Ok(quote! {
                #read

                #write
            })
        }
    }
}

fn render_read(
    field: &_Field,
    mask_length: u32,
    value_type: &Ident,
    value_size: u32,
    as_bool: bool,
) -> Result<proc_macro2::TokenStream> {
    let description = &field.description;
    let ty = &field.ty;
    let ty_span = ty.span();
    let offset = &field.offset;
    let mask = build_mask(mask_length, value_size, field.size.span())?;
    if as_bool {
        let method_name = format_ident!("is_{}_set", field.name);
        Ok(quote! {
            #[doc = #description]
            #[inline]
            pub fn #method_name(&self) -> bool {
                ((Register::get_bits(self) >> #offset) & 1) == 1
            }
        })
    } else {
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

fn render_write(
    field: &_Field,
    mask_length: u32,
    value_type: &Ident,
    value_size: u32,
    as_bool: bool,
) -> Result<proc_macro2::TokenStream> {
    let description = &field.description;
    let ty = &field.ty;
    let ty_span = ty.span();
    let offset = &field.offset;
    let mask = build_mask(mask_length, value_size, field.size.span())?;
    if as_bool {
        let method_name = format_ident!("set_{}_value", field.name);
        let method_name_set = format_ident!("set_{}", field.name);
        let method_name_unset = format_ident!("unset_{}", field.name);
        Ok(quote! {
            #[doc = #description]
            #[inline]
            pub fn #method_name(&mut self, value: bool) -> &mut Self {
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
    } else {
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

fn build_mask(mask_length: u32, value_size: u32, span: Span) -> Result<LitInt> {
    let mask = match value_size {
        8 => Some(format!("0x{:X}u8", 2u8.pow(mask_length) - 1)),
        16 => Some(format!("0x{:X}u16", 2u16.pow(mask_length) - 1)),
        32 => Some(format!("0x{:X}u32", 2u32.pow(mask_length) - 1)),
        64 => Some(format!("0x{:X}u64", 2u64.pow(mask_length) - 1)),
        _ => None,
    };
    mask.map(|mask| LitInt::new(mask.as_str(), span))
        .ok_or(syn::Error::new(
            span,
            format!("Unexpected value size {}", value_size),
        ))
}
