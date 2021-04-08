use crate::{TokenStream2, TypeGenerator};
use frame_metadata::{v13::RuntimeMetadataV13, RuntimeMetadata, RuntimeMetadataPrefixed};
use quote::{format_ident, quote};
use scale_info::prelude::string::ToString;
use heck::SnakeCase as _;

pub struct RuntimeGenerator {
    metadata: RuntimeMetadataV13,
}

impl RuntimeGenerator {
    pub fn new(metadata: RuntimeMetadataPrefixed) -> Self {
        match metadata.1 {
            RuntimeMetadata::V13(v13) => Self { metadata: v13 },
            _ => panic!("Unsupported metadata version {:?}", metadata.1),
        }
    }

    pub fn generate_runtime(&self, mod_name: &str) -> TokenStream2 {
        let type_gen = TypeGenerator::new(&self.metadata.types, "__runtime_types");
        let types_mod = type_gen.generate_types_mod();
        let types_mod_ident = types_mod.ident();
        let modules = self.metadata.modules.iter().map(|module| {
            let mod_name = format_ident!("{}", module.name.to_string().to_snake_case());
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
                        let ty = type_gen.resolve_type_path(arg.ty.id(), &[]);
                        // todo: add docs and #[compact] attr
                        quote! { #name: #ty }
                    });
                    quote! {
                        #[derive(Debug, ::codec::Encode, ::codec::Decode)]
                        pub struct #name {
                            #( #args ),*
                        }
                    }
                })
                .collect::<Vec<_>>();
            let event =
                if let Some(ref events) = module.event {
                    let event_variants = events
                        .iter()
                        .map(|event| {
                            let name = format_ident!("{}", event.name);
                            let args = event.arguments.iter().map(|arg| {
                                type_gen.resolve_type_path(arg.ty.id(), &[])
                                // todo: add docs and #[compact] attr
                            });
                            quote! {
                        #name (#( #args ),*),
                    }
                        })
                        .collect::<Vec<_>>();
                    quote! {
                        #[derive(Debug, ::codec::Encode, ::codec::Decode)]
                        pub enum Event {
                            #( #event_variants )*
                        }
                    }
                } else {
                    quote! {}
                };

            let calls = if !calls.is_empty() {
                quote! {
                    mod calls {
                        use super::#types_mod_ident;
                        #( #calls )*
                    }
                }
            } else {
                quote! {}
            };

            quote! {
                pub mod #mod_name {
                    use super::#types_mod_ident;
                    #calls
                    #event
                }
            }
        });

        let outer_event_variants = self.metadata.modules.iter().filter_map(|m| {
            let variant_name = format_ident!("{}", m.name);
            let mod_name = format_ident!("{}", m.name.to_string().to_snake_case());
            let index = proc_macro2::Literal::u8_unsuffixed(m.index);

            m.event.as_ref().map(|_| {
                quote! {
                    #[codec(index = #index)]
                    #variant_name(#mod_name::Event),
                }
            })
        });

        let outer_event = quote! {
            #[derive(Debug, ::codec::Encode, ::codec::Decode)]
            pub enum Event {
                #( #outer_event_variants )*
            }
        };

        let mod_name = format_ident!("{}", mod_name);
        quote! {
            #[allow(dead_code, unused_imports, non_camel_case_types)]
            pub mod #mod_name {
                #outer_event
                #( #modules )*
                #types_mod
            }
        }
    }
}
