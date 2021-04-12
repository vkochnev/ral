use proc_macro2::Span;
use syn::{Result, TypePath};

pub(crate) enum _FieldType {
    Bool(TypePath),
    Primitive(TypePath),
    Custom(TypePath),
}

impl _FieldType {
    pub(crate) fn from(ty: TypePath, width: u32, width_span: Span) -> Result<_FieldType> {
        if let Some(ty_name) = ty.path.get_ident() {
            match ty_name.to_string().as_str() {
                "bool" => {
                    if width != 1 {
                        Err(syn::Error::new(
                            width_span,
                            format!("Field size must be 1 for bool"),
                        ))
                    } else {
                        Ok(_FieldType::Bool(ty))
                    }
                }
                "u8" | "u16" | "u32" | "u64" => Ok(_FieldType::Primitive(ty)),
                _ => Ok(_FieldType::Custom(ty)),
            }
        } else {
            Ok(_FieldType::Custom(ty))
        }
    }
}
