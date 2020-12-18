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
            let calls = module
                .calls
                .as_ref()
                .unwrap_or(&Vec::new())
                .iter()
                .map(|call| {
                    use heck::CamelCase as _;
                    // todo: add free functions to Call mod and doc strings
                    let name = format_ident!("{}", call.name.to_string().to_camel_case());
                    let args = call.arguments.iter().map(|arg| {
                        let name = format_ident!("{}", arg.name);
                        let ty = self.type_generator.resolve_type(arg.ty.id(), &[]);
                        // todo: add docs and #[compact] attr
                        quote! { #name: #ty }
                    });
                    quote! {
                        pub struct #name {
                            #( #args ),*
                        }
                    }
                })
                .collect::<Vec<_>>();
            let events = module
                .event
                .as_ref()
                .unwrap_or(&Vec::new())
                .iter()
                .map(|event| {
                    let name = format_ident!("{}", event.name);
                    let args = event.arguments.iter().map(|arg| {
                        self.type_generator.resolve_type(arg.ty.id(), &[])
                        // todo: add docs and #[compact] attr
                    });
                    quote! {
                        pub struct #name (
                            #( #args ),*
                        );
                    }
                })
                .collect::<Vec<_>>();
            let calls = if !calls.is_empty() {
                quote! {
                    mod calls {
                        // todo: use types mod name defined earlier
                        use super::*;
                        #( #calls )*
                    }
                }
            } else {
                quote! {}
            };
            let events = if !events.is_empty() {
                quote! {
                    pub mod events {
                        use super::*;
                        #( #events )*
                    }
                }
            } else {
                quote! {}
            };

            quote! {
                pub mod #mod_name {
                    use super::types::*;
                    #calls
                    #events
                }
            }
        });

        let mod_name = format_ident!("{}", mod_name);
        quote! {
            pub mod #mod_name {
                #types

                #( #modules )*
            }
        }
        // todo: generate outer event? needs custom decode for potential changing indices
    }
}
