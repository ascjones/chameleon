use crate::{TokenStream2, TypeGenerator};
use frame_metadata::{v14::RuntimeMetadataV14, RuntimeMetadata, RuntimeMetadataPrefixed, PalletCallMetadata};
use heck::SnakeCase as _;
use quote::{format_ident, quote};
use scale_info::prelude::string::ToString;
use scale_info::form::PortableForm;

pub struct RuntimeGenerator {
    metadata: RuntimeMetadataV14,
}

impl RuntimeGenerator {
    pub fn new(metadata: RuntimeMetadataPrefixed) -> Self {
        match metadata.1 {
            RuntimeMetadata::V14(v14) => Self { metadata: v14 },
            _ => panic!("Unsupported metadata version {:?}", metadata.1),
        }
    }

    pub fn generate_runtime(&self, mod_name: &str) -> TokenStream2 {
        let type_gen = TypeGenerator::new(&self.metadata.types, "__runtime_types");
        let types_mod = type_gen.generate_types_mod();
        let types_mod_ident = types_mod.ident();
        let modules = self.metadata.pallets.iter().map(|pallet| {
            let mod_name = format_ident!("{}", pallet.name.to_string().to_snake_case());
            let mut calls = Vec::new();
            for call in &pallet.calls {
                let call_structs = self.generate_call_structs(&type_gen, call);
                calls.extend(call_structs)
            }

            let event = if let Some(ref event) = pallet.event {
                let event_type = type_gen.resolve_type_path(event.ty.id(), &[]);
                quote! {
                    pub type Event = #event_type;
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

        let outer_event_variants = self.metadata.pallets.iter().filter_map(|p| {
            let variant_name = format_ident!("{}", p.name);
            let mod_name = format_ident!("{}", p.name.to_string().to_snake_case());
            let index = proc_macro2::Literal::u8_unsuffixed(p.index);

            p.event.as_ref().map(|_| {
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

    fn generate_call_structs(&self, type_gen: &TypeGenerator, call: &PalletCallMetadata<PortableForm>) -> Vec<TokenStream2> {
        let ty = call.ty;
        let name = type_gen.resolve_type_path(ty.id(), &[]);
        use crate::generate_types::TypePath;
        match name {
            TypePath::Parameter(_) => panic!("Call type should be a Type"),
            TypePath::Type(ref ty) => {
                let ty = ty.ty();

                let type_def = ty.type_def();
                if let scale_info::TypeDef::Variant(variant) = type_def {
                    variant
                        .variants()
                        .iter()
                        .map(|var| {
                            use heck::CamelCase;
                            let name =
                                format_ident!("{}", var.name().to_string().to_camel_case());
                            let args = var.fields().iter().filter_map(|field| {
                                field.name().map(|name| {
                                    let name = format_ident!("{}", name);
                                    let ty =
                                        type_gen.resolve_type_path(field.ty().id(), &[]);
                                    quote! { #name: #ty }
                                })
                            });

                            quote! {
                                #[derive(Debug, ::codec::Encode, ::codec::Decode)]
                                pub struct #name {
                                    #( #args ),*
                                }
                            }
                        })
                        .collect::<Vec<_>>()
                } else {
                    panic!("Call type should be an variant/enum type")
                }
            }
        }
    }
}
