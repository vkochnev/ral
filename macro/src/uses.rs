use syn::parse::{Parse, ParseStream};
use syn::token::Use;
use syn::{ItemUse, Result};

pub(super) struct _Uses(pub(super) Vec<ItemUse>);

impl Parse for _Uses {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut uses = Vec::new();
        while input.peek(Use) {
            uses.push(input.parse::<ItemUse>()?);
        }
        Ok(_Uses(uses))
    }
}
