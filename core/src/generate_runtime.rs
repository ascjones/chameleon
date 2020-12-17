use crate::{
    TokenStream2,
    TypeGenerator,
};
use frame_metadata::{
    RuntimeMetadata,
    RuntimeMetadataLastVersion,
    RuntimeMetadataPrefixed,
};
use quote::{
    format_ident,
    quote,
    IdentFragment,
};
use scale_info::{
    form::{
        CompactForm,
        FormString,
    },
    prelude::string::ToString,
};

pub struct RuntimeGenerator<S: FormString> {
    metadata: RuntimeMetadataLastVersion<CompactForm<S>>,
    type_generator: TypeGenerator<S>,
}

impl<S> RuntimeGenerator<S>
where
    S: FormString + From<&'static str> + ToString + IdentFragment,
{
    pub fn new(metadata: RuntimeMetadataPrefixed<S>) -> Self {
        match metadata.metadata {
            RuntimeMetadata::V12(v12) => {
                Self {
                    metadata: v12,
                    type_generator: TypeGenerator::new(metadata.types),
                }
            }
        }
    }

    pub fn generate_runtime(&self, mod_name: &str) -> TokenStream2 {
        let types_mod = "types";
        let types = self.type_generator.generate(types_mod);
        let modules = self.metadata.modules.iter().map(|module| {
            let mod_name = format_ident!("{}", module.name);
            quote! {
                mod #mod_name {

                }
            }
        });

        let mod_name = format_ident!("{}", mod_name);
        quote! {
            mod #mod_name {
                #types

                #( #modules )*
            }
        }
        // generate outer event/call
        // generate modules
        // generate calls/events
    }
}
