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

use proc_macro2::{Ident, Span, TokenStream as TokenStream2, TokenStream};
use quote::{format_ident, quote, ToTokens};
use scale_info::{
    form::PortableForm, prelude::num::NonZeroU32, Field, PortableRegistry, Type, TypeDef,
    TypeDefPrimitive,
};
use std::collections::BTreeMap;

pub struct TypeGenerator<'a> {
    type_registry: &'a PortableRegistry,
}

impl<'a> TypeGenerator<'a> {
    /// Construct a new [`TypeGenerator`].
    pub fn new(type_registry: &'a PortableRegistry) -> Self {
        Self { type_registry  }
    }

    /// Generate a module containing all types defined in the supplied type registry.
    pub fn generate_types_mod(
        &self,
        root_mod: &'static str,
    ) -> Module<'a> {
        let root_mod_ident = Ident::new(root_mod, Span::call_site());
        let mut root_mod = Module::new(root_mod_ident.clone(), root_mod_ident.clone());

        for (id, ty) in self.type_registry.enumerate() {
            if ty.path().namespace().is_empty() {
                // prelude types e.g. Option/Result have no namespace, so we don't generate them
                continue;
            }
            self.insert_type(
                ty.clone(),
                id,
                ty.path().namespace().to_vec(),
                &root_mod_ident,
                &mut root_mod,
            )
        }

        root_mod
    }

    fn insert_type(
        &self,
        ty: Type<PortableForm>,
        id: NonZeroU32,
        path: Vec<String>,
        root_mod_ident: &Ident,
        module: &mut Module<'a>,
    ) {
        let segment = path.first().expect("path has at least one segment");
        let mod_ident = Ident::new(segment, Span::call_site());

        let child_mod = module
            .children
            .entry(mod_ident.clone())
            .or_insert(Module::new(mod_ident, root_mod_ident.clone()));

        if path.len() == 1 {
            child_mod.types.insert(id, ModuleType { ty, type_registry: self.type_registry });
        } else {
            self.insert_type(
                ty,
                id,
                path[1..].to_vec(),
                root_mod_ident,
                child_mod,
            )
        }
    }
}

/// # Panics
///
/// If no type with the given id found in the type registry.
pub fn resolve_type_path(
    type_registry: &PortableRegistry,
    id: NonZeroU32,
    parent_type_params: &[TypeParameter],
) -> syn::Type {
    if let Some(parent_type_param) = parent_type_params
        .iter()
        .find(|tp| tp.concrete_type_id == id)
    {
        let ty = &parent_type_param.name;
        return syn::Type::Path(syn::parse_quote! { #ty });
    }

    let ty = type_registry
        .resolve(id)
        .expect(&format!("No type with id {} found", id));

    let type_params = ty
        .type_params()
        .iter()
        .map(|tp| resolve_type_path(type_registry, tp.id(), parent_type_params))
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
                syn::parse_quote! { #ty< #( #type_params ),* > }
            };
            syn::Type::Path(path)
        }
        TypeDef::Sequence(sequence) => {
            let type_param =
                resolve_type_path(type_registry, sequence.type_param().id(), parent_type_params);
            let type_path = syn::parse_quote! { Vec<#type_param> };
            syn::Type::Path(type_path)
        }
        TypeDef::Array(array) => {
            let array_type =
                resolve_type_path(type_registry, array.type_param().id(), parent_type_params);
            let array_len = array.len() as usize;
            let array = syn::parse_quote! { [#array_type; #array_len] };
            syn::Type::Array(array)
        }
        TypeDef::Tuple(tuple) => {
            let tuple_types = tuple
                .fields()
                .iter()
                .map(|type_id| resolve_type_path(type_registry, type_id.id(), parent_type_params));
            let tuple = syn::parse_quote! { (#( # tuple_types, )* ) };
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
        TypeDef::Phantom(phantom) => {
            let type_param =
                resolve_type_path(type_registry, phantom.type_param().id(), parent_type_params);
            let type_path = syn::parse_quote! { core::marker::PhantomData<#type_param> };
            syn::Type::Path(type_path)
        }
        TypeDef::Compact(compact) => {
            // todo: change the return type of this method to include info that it is compact
            // and should be annotated with #[compact] for fields
            resolve_type_path(type_registry, compact.type_param().id(), parent_type_params)
        }
    }
}


#[derive(Debug)]
pub struct Module<'a> {
    name: Ident,
    root_mod: Ident,
    children: BTreeMap<Ident, Module<'a>>,
    types: BTreeMap<NonZeroU32, ModuleType<'a>>,
}

impl<'a> ToTokens for Module<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.name;
        let root_mod = &self.root_mod;
        let modules = self.children.values();
        let types = self.types.values().clone();

        tokens.extend(quote! {
            pub mod #name {
                use super::#root_mod;
                #( #modules )*
                #( #types )*
            }
        })
    }
}

impl<'a> Module<'a> {
    pub fn new(name: Ident, root_mod: Ident) -> Self {
        Self {
            name,
            root_mod,
            children: BTreeMap::new(),
            types: BTreeMap::new(),
        }
    }

    /// Returns the module with the given path, if any
    pub fn get_mod(&'a self, path_segs: &[&'static str]) -> Option<&'a Module<'a>> {
        let (mod_name, rest) = path_segs.split_first()?;
        let mod_ident = Ident::new(mod_name, Span::call_site());
        let module = self.children.get(&mod_ident)?;
        if rest.is_empty() {
            Some(module)
        } else {
            module.get_mod(rest)
        }
    }
}

#[derive(Debug)]
pub struct ModuleType<'a> {
    type_registry: &'a PortableRegistry,
    ty: Type<PortableForm>,
}

impl<'a> quote::ToTokens for ModuleType<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let type_params = self
            .ty
            .type_params()
            .iter()
            .enumerate()
            .map(|(i, tp)| {
                let tp_name = format_ident!("_{}", i);
                TypeParameter {
                    concrete_type_id: tp.id(),
                    name: tp_name,
                }
            })
            .collect::<Vec<_>>();

        let type_name = self.ty.path().ident().map(|ident| {
            let type_params = if !type_params.is_empty() {
                let tps = type_params.iter().map(|tp| tp.name.clone());
                quote! { < #( #tps ),* > }
            } else {
                quote! {}
            };
            let ty = format_ident!("{}", ident);
            let path = syn::parse_quote! { #ty #type_params};
            syn::Type::Path(path)
        });

        match self.ty.type_def() {
            TypeDef::Composite(composite) => {
                let type_name = type_name.expect("structs should have a name");
                let fields = self.composite_fields(composite.fields(), &type_params, true);
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
                        self.composite_fields(v.fields(), &type_params, false)
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
    }
}

impl<'a> ModuleType<'a> {
    fn composite_fields(
        &self,
        fields: &[Field<PortableForm>],
        type_params: &[TypeParameter],
        is_struct: bool,
    ) -> TokenStream2 {
        let named = fields.iter().all(|f| f.name().is_some());
        let unnamed = fields.iter().all(|f| f.name().is_none());
        if named {
            let fields = fields.iter().map(|field| {
                let name = format_ident!("{}", field.name().expect("named field without a name"));
                let ty = resolve_type_path(self.type_registry, field.ty().id(), type_params);
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
                let ty = resolve_type_path(self.type_registry,field.ty().id(), type_params);
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
}

pub struct TypeParameter {
    concrete_type_id: NonZeroU32,
    name: proc_macro2::Ident,
}

#[cfg(test)]
mod tests {
    use super::*;
    use scale_info::{meta_type, Registry, TypeInfo};

    const MOD_PATH: &'static [&'static str] = &["chameleon_core", "generate_types", "tests"];

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
        let portable_types: PortableRegistry = registry.into();

        let type_gen = TypeGenerator::new(&portable_types);
        let types = type_gen.generate_types_mod("root");
        let tests_mod = types.get_mod(MOD_PATH).unwrap();

        assert_eq!(
            tests_mod.into_token_stream().to_string(),
            quote! {
                pub mod tests {
                    use super::root;
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
        let portable_types: PortableRegistry = registry.into();

        let type_gen = TypeGenerator::new(&portable_types);
        let types = type_gen.generate_types_mod("root");
        let tests_mod = types.get_mod(MOD_PATH).unwrap();

        assert_eq!(
            tests_mod.into_token_stream().to_string(),
            quote! {
                pub mod tests {
                    use super::root;

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
        let portable_types: PortableRegistry = registry.into();

        let type_gen = TypeGenerator::new(&portable_types);
        let types = type_gen.generate_types_mod("root");
        let tests_mod = types.get_mod(MOD_PATH).unwrap();

        assert_eq!(
            tests_mod.into_token_stream().to_string(),
            quote! {
                pub mod tests {
                    use super::root;

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
        let portable_types: PortableRegistry = registry.into();

        let type_gen = TypeGenerator::new(&portable_types);
        let types = type_gen.generate_types_mod("root");
        let tests_mod = types.get_mod(MOD_PATH).unwrap();

        assert_eq!(
            tests_mod.into_token_stream().to_string(),
            quote! {
                pub mod tests {
                    use super::root;
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
        let portable_types: PortableRegistry = registry.into();

        let type_gen = TypeGenerator::new(&portable_types);
        let types = type_gen.generate_types_mod("root");
        let tests_mod = types.get_mod(MOD_PATH).unwrap();

        assert_eq!(
            tests_mod.into_token_stream().to_string(),
            quote! {
                pub mod tests {
                    use super::root;
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
        let portable_types: PortableRegistry = registry.into();

        let type_gen = TypeGenerator::new(&portable_types);
        let types = type_gen.generate_types_mod("root");
        let tests_mod = types.get_mod(MOD_PATH).unwrap();

        assert_eq!(
            tests_mod.into_token_stream().to_string(),
            quote! {
                pub mod tests {
                    use super::root;
                    pub struct S {
                        pub a: Option<bool>,
                        pub b: Option<u32>,
                    }
                }
            }
            .to_string()
        )
    }

    #[test]
    fn generics() {
        #[allow(unused)]
        #[derive(TypeInfo)]
        struct Foo<T> {
            a: T,
        }

        #[allow(unused)]
        #[derive(TypeInfo)]
        struct Bar {
            b: Foo<u32>,
        }

        let mut registry = Registry::new();
        registry.register_type(&meta_type::<Bar>());
        let portable_types: PortableRegistry = registry.into();

        let type_gen = TypeGenerator::new(&portable_types);
        let types = type_gen.generate_types_mod("root");
        let tests_mod = types.get_mod(MOD_PATH).unwrap();

        assert_eq!(
            tests_mod.into_token_stream().to_string(),
            quote! {
                pub mod tests {
                    use super::root;
                    pub struct Bar {
                        pub b: Foo<u32>,
                    }
                    pub struct Foo<_0> {
                        pub a: _0,
                    }
                }
            }
            .to_string()
        )
    }

    #[test]
    fn generics_nested() {
        #[allow(unused)]
        #[derive(TypeInfo)]
        struct Foo<T, U> {
            a: T,
            b: Option<(T, U)>,
        }

        #[allow(unused)]
        #[derive(TypeInfo)]
        struct Bar<T> {
            b: Foo<T, u32>,
        }

        let mut registry = Registry::new();
        registry.register_type(&meta_type::<Bar<bool>>());
        let portable_types: PortableRegistry = registry.into();

        let type_gen = TypeGenerator::new(&portable_types);
        let types = type_gen.generate_types_mod("root");
        let tests_mod = types.get_mod(MOD_PATH).unwrap();

        assert_eq!(
            tests_mod.into_token_stream().to_string(),
            quote! {
                pub mod tests {
                    use super::root;
                    pub struct Bar<_0> {
                        pub b: Foo<_0, u32>,
                    }

                    pub struct Foo<_0, _1> {
                        pub a: _0,
                        pub b: Option<(_0, _1)>,
                    }
                }
            }
            .to_string()
        )
    }

    #[test]
    fn modules() {
        mod modules {
            pub mod a {
                #[allow(unused)]
                #[derive(scale_info::TypeInfo)]
                pub struct Foo {}

                pub mod b {
                    #[allow(unused)]
                    #[derive(scale_info::TypeInfo)]
                    pub struct Bar {
                        a: super::Foo,
                    }
                }
            }

            pub mod c {
                #[allow(unused)]
                #[derive(scale_info::TypeInfo)]
                pub struct Foo {
                    a: super::a::b::Bar,
                }
            }
        }

        let mut registry = Registry::new();
        registry.register_type(&meta_type::<modules::c::Foo>());
        let portable_types: PortableRegistry = registry.into();

        let type_gen = TypeGenerator::new(&portable_types);
        let types = type_gen.generate_types_mod("root");
        let tests_mod = types.get_mod(MOD_PATH).unwrap();

        assert_eq!(
            tests_mod.into_token_stream().to_string(),
            quote! {
                pub mod tests {
                    use super::root;
                    pub mod modules {
                        use super::root;
                        pub mod a {
                            use super::root;

                            pub mod b {
                                use super::root;

                                pub struct Bar {
                                    a: root::a::Foo,
                                }
                            }

                            pub struct Foo {}
                        }

                        pub mod c {
                            use super::root;

                            pub struct Foo {
                                a: root::a::b::Bar,
                            }
                        }
                    }
                }
            }
            .to_string()
        )
    }
}
