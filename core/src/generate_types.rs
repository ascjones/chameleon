// Copyright 2019-2020 Parity Technologies (UK) Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use proc_macro2::TokenStream as TokenStream2;
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
    prelude::{
        num::NonZeroU32,
        string::ToString,
    },
    Field,
    RegistryReadOnly,
    TypeDef,
    TypeDefPrimitive,
};

pub struct TypeGenerator<S: FormString> {
    types: RegistryReadOnly<S>,
}

impl<S> TypeGenerator<S>
where
    S: FormString + From<&'static str> + ToString + IdentFragment,
{
    /// Construct a new [`TypeGenerator`] with the given type registry.
    pub fn new(types: RegistryReadOnly<S>) -> Self {
        TypeGenerator { types }
    }

    pub fn generate(&self, root_mod: &str) -> TokenStream2 {
        let mut tokens = TokenStream2::new();
        for (_, ty) in self.types.enumerate() {
            if ty.path().namespace().is_empty() {
                // prelude types e.g. Option/Result have no namespace, so we don't generate them
                continue
            }
            let type_name = ty.path().ident().map(|ident| {
                let ty = format_ident!("{}", ident);
                let path = syn::parse_quote! { #ty };
                syn::Type::Path(path)
            });

            match ty.type_def() {
                TypeDef::Composite(composite) => {
                    let type_name = type_name.expect("structs should have a name");
                    let fields = self.composite_fields(composite.fields(), true);
                    let ty_toks = quote! {
                        pub struct #type_name #fields
                    };
                    tokens.extend(ty_toks);
                }
                TypeDef::Variant(variant) => {
                    let type_name = type_name.expect("variants should have a name");
                    let variants = variant.variants().iter().map(|v| {
                        let variant_name = format_ident!("{}", v.name());
                        let fields = if v.fields().is_empty() {
                            quote! {}
                        } else {
                            self.composite_fields(v.fields(), false)
                        };
                        quote! {
                            #variant_name #fields
                        }
                    });
                    let ty_toks = quote! {
                        pub enum #type_name {
                            #( #variants, )*
                        }
                    };
                    tokens.extend(ty_toks);
                }
                _ => (), // all built-in types should already be in scope
            }
            // ty.generate_type(&mut tokens, ty, types);
        }
        let root_mod = format_ident!("{}", root_mod);

        quote! {
            // required that this be placed at crate root so can do ::registry_types.
            // alternatively use relative paths? more complicated
            mod #root_mod {
                #tokens
            }
        }
    }

    fn composite_fields(
        &self,
        fields: &[Field<CompactForm<S>>],
        is_struct: bool,
    ) -> TokenStream2 {
        let named = fields.iter().all(|f| f.name().is_some());
        let unnamed = fields.iter().all(|f| f.name().is_none());
        if named {
            let fields = fields.iter().map(|field| {
                let name = format_ident!(
                    "{}",
                    field.name().expect("named field without a name")
                );
                let ty = self.resolve_type(field.ty().id());
                if is_struct {
                    quote! { pub #name: #ty }
                } else {
                    quote! { #name: #ty }
                }
            });
            quote! {
                {
                    #( #fields, )*
                }
            }
        } else if unnamed {
            let fields = fields.iter().map(|field| {
                let ty = self.resolve_type(field.ty().id());
                if is_struct {
                    quote! { pub #ty }
                } else {
                    quote! { #ty }
                }
            });
            let fields = quote! { ( #( #fields, )* ) };
            if is_struct {
                // add a semicolon for tuple structs
                quote! { #fields; }
            } else {
                fields
            }
        } else {
            panic!("Fields must be either all named or all unnamed")
        }
    }

    /// # Panics
    ///
    /// If no type with the given id found in the type registry.
    fn resolve_type(&self, id: NonZeroU32) -> syn::Type {
        let ty = self
            .types
            .resolve(id)
            .expect(&format!("No type with id {} found", id));

        let type_params = ty
            .type_params()
            .iter()
            .map(|tp| self.resolve_type(tp.id()))
            .collect::<Vec<_>>();

        match ty.type_def() {
            TypeDef::Composite(_) | TypeDef::Variant(_) => {
                let ident = ty
                    .path()
                    .ident()
                    .expect("custom structs/enums should have a name");
                let ty = format_ident!("{}", ident);
                let path = if type_params.is_empty() {
                    syn::parse_quote! { #ty }
                } else {
                    syn::parse_quote! { #ty<#( #type_params), *> }
                };
                syn::Type::Path(path)
            }
            TypeDef::Sequence(sequence) => {
                let type_param = self.resolve_type(sequence.type_param().id());
                let type_path = syn::parse_quote! { Vec<#type_param> };
                syn::Type::Path(type_path)
            }
            TypeDef::Array(array) => {
                let array_type = self.resolve_type(array.type_param().id());
                let array_len = array.len() as usize;
                let array = syn::parse_quote! { [#array_type; #array_len] };
                syn::Type::Array(array)
            }
            TypeDef::Tuple(tuple) => {
                let tuple_types = tuple
                    .fields()
                    .iter()
                    .map(|type_id| self.resolve_type(type_id.id()));
                let tuple = syn::parse_quote! { (#( # tuple_types )*,) };
                syn::Type::Tuple(tuple)
            }
            TypeDef::Primitive(primitive) => {
                let primitive = match primitive {
                    TypeDefPrimitive::Bool => "bool",
                    TypeDefPrimitive::Char => "char",
                    TypeDefPrimitive::Str => "String",
                    TypeDefPrimitive::U8 => "u8",
                    TypeDefPrimitive::U16 => "u16",
                    TypeDefPrimitive::U32 => "u32",
                    TypeDefPrimitive::U64 => "u64",
                    TypeDefPrimitive::U128 => "u128",
                    TypeDefPrimitive::U256 => unimplemented!("not a rust primitive"),
                    TypeDefPrimitive::I8 => "i8",
                    TypeDefPrimitive::I16 => "i16",
                    TypeDefPrimitive::I32 => "i32",
                    TypeDefPrimitive::I64 => "i64",
                    TypeDefPrimitive::I128 => "i128",
                    TypeDefPrimitive::I256 => unimplemented!("not a rust primitive"),
                };
                let ident = format_ident!("{}", primitive);
                let path = syn::parse_quote! { #ident };
                syn::Type::Path(path)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use scale_info::{
        meta_type,
        Registry,
        TypeInfo,
    };

    #[test]
    fn generate_struct_with_primitives() {
        #[allow(unused)]
        #[derive(TypeInfo)]
        struct S {
            a: bool,
            b: u32,
            c: char,
        }

        let mut registry = Registry::new();
        registry.register_type(&meta_type::<S>());

        let generator = TypeGenerator::new(registry.into());
        let types = generator.generate("root");

        assert_eq!(
            types.to_string(),
            quote! {
                mod root {
                    pub struct S {
                        pub a: bool,
                        pub b: u32,
                        pub c: char,
                    }
                }
            }
            .to_string()
        )
    }

    #[test]
    fn generate_struct_with_a_struct_field() {
        #[allow(unused)]
        #[derive(TypeInfo)]
        struct Parent {
            a: bool,
            b: Child,
        }

        #[allow(unused)]
        #[derive(TypeInfo)]
        struct Child {
            a: i32,
        }

        let mut registry = Registry::new();
        registry.register_type(&meta_type::<Parent>());

        let generator = TypeGenerator::new(registry.into());
        let types = generator.generate("root");

        assert_eq!(
            types.to_string(),
            quote! {
                mod root {
                    pub struct Parent {
                        pub a: bool,
                        pub b: Child,
                    }

                    pub struct Child {
                        pub a: i32,
                    }
                }
            }
            .to_string()
        )
    }

    #[test]
    fn generate_tuple_struct() {
        #[allow(unused)]
        #[derive(TypeInfo)]
        struct Parent(bool, Child);

        #[allow(unused)]
        #[derive(TypeInfo)]
        struct Child(i32);

        let mut registry = Registry::new();
        registry.register_type(&meta_type::<Parent>());

        let generator = TypeGenerator::new(registry.into());
        let types = generator.generate("root");

        assert_eq!(
            types.to_string(),
            quote! {
                mod root {
                    pub struct Parent(pub bool, pub Child,);
                    pub struct Child(pub i32,);
                }
            }
            .to_string()
        )
    }

    #[test]
    fn generate_enum() {
        #[allow(unused)]
        #[derive(TypeInfo)]
        enum E {
            A,
            B(bool),
            C { a: u32 },
        }

        let mut registry = Registry::new();
        registry.register_type(&meta_type::<E>());

        let generator = TypeGenerator::new(registry.into());
        let types = generator.generate("root");

        assert_eq!(
            types.to_string(),
            quote! {
                mod root {
                    pub enum E {
                        A,
                        B (bool,),
                        C { a: u32, },
                    }
                }
            }
            .to_string()
        )
    }

    #[test]
    fn generate_array_field() {
        #[allow(unused)]
        #[derive(TypeInfo)]
        struct S {
            a: [u8; 32],
        }

        let mut registry = Registry::new();
        registry.register_type(&meta_type::<S>());

        let generator = TypeGenerator::new(registry.into());
        let types = generator.generate("root");

        assert_eq!(
            types.to_string(),
            quote! {
                mod root {
                    pub struct S {
                        pub a: [u8; 32usize],
                    }
                }
            }
            .to_string()
        )
    }

    #[test]
    fn option_fields() {
        #[allow(unused)]
        #[derive(TypeInfo)]
        struct S {
            a: Option<bool>,
            b: Option<u32>,
        }

        let mut registry = Registry::new();
        registry.register_type(&meta_type::<S>());

        let generator = TypeGenerator::new(registry.into());
        let types = generator.generate("root");

        assert_eq!(
            types.to_string(),
            quote! {
                mod root {
                    pub struct S {
                        pub a: Option<bool>,
                        pub b: Option<u32>,
                    }
                }
            }
            .to_string()
        )
    }
}
