use crate::TokenStream2;
use frame_metadata::{RuntimeMetadataPrefixed, RuntimeMetadataLastVersion, RuntimeMetadata};
use scale_info::prelude::string::ToString;
use scale_info::{form::{CompactForm, FormString}, RegistryReadOnly};
use quote::{
    quote,
    format_ident,
    IdentFragment,
};

pub fn generate_runtime<S>(mod_name: &str, metadata: RuntimeMetadataPrefixed<S>) -> TokenStream2
where
    S: FormString + From<&'static str> + ToString + IdentFragment,
{
    let types_mod = "types";
    let types = crate::generate_types::generate(types_mod, &metadata.types);
    let runtime = metadata.metadata.generate_code(&metadata.types);

    let mod_name = format_ident!("{}", mod_name);
    quote! {
        mod #mod_name {
            #types

            #runtime
        }
    }
    // generate outer event/call
    // generate modules
    // generate calls/events
}

pub trait GenerateCode<S: FormString> {
    fn generate_code(&self, types: &RegistryReadOnly<S>) -> TokenStream2;
}

impl<S> GenerateCode<S> for RuntimeMetadata<CompactForm<S>>
where
    S: FormString + From<&'static str> + IdentFragment,
{
    fn generate_code(&self, types: &RegistryReadOnly<S>) -> TokenStream2 {
        match self {
            Self::V12(metadata) => metadata.generate_code(types)
        }
    }
}

impl<S> GenerateCode<S> for RuntimeMetadataLastVersion<CompactForm<S>>
where
    S: FormString + From<&'static str> + IdentFragment,
{
    fn generate_code(&self, types: &RegistryReadOnly<S>) -> TokenStream2 {
        let modules = self.modules
            .iter()
            .map(|module| {
                let mod_name = format_ident!("{}", module.name);
                quote! {
                    mod #mod_name {

                    }
                }
            });
        quote! {
            #( #modules )*
        }
    }
}

