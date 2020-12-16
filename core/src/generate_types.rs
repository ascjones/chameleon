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

use scale_info::{form::{CompactForm, FormString}, RegistryReadOnly, TypeDef, Type, TypeDefPrimitive, TypeDefComposite, TypeDefVariant, Field};
use proc_macro2::{TokenStream as TokenStream2};
use quote::{
    quote,
    format_ident,
    IdentFragment,
};

// todo: [AJ] this could be a separate crate so can be used from other macros to generate e.g. all runtime types
pub fn generate<S>(root_mod: &str, types: &RegistryReadOnly<S>) -> TokenStream2
where
    S: FormString + From<&'static str> + IdentFragment,
{
    let mut tokens = TokenStream2::new();
    for (_, ty) in types.enumerate() {
        ty.generate_type(&mut tokens, ty, types);
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

trait GenerateType<S: FormString> {
    fn type_name(&self, ty: &Type<CompactForm<S>>) -> S;
    fn generate_type(&self, tokens: &mut TokenStream2, ty: &Type<CompactForm<S>>, types: &RegistryReadOnly<S>);
}

impl<S> GenerateType<S> for Type<CompactForm<S>>
where
    S: FormString + From<&'static str> + IdentFragment,
{
    fn type_name(&self, ty: &Type<CompactForm<S>>) -> S {
        self.type_def().type_name(ty)
    }

    fn generate_type(&self, tokens: &mut TokenStream2, ty: &Type<CompactForm<S>>, types: &RegistryReadOnly<S>) {
        self.type_def().generate_type(tokens, ty, types)
    }
}

impl<S> GenerateType<S> for TypeDef<CompactForm<S>>
where
    S: FormString + From<&'static str> + IdentFragment,
{
    fn type_name(&self, ty: &Type<CompactForm<S>>) -> S {
        match self {
            TypeDef::Composite(composite) => composite.type_name(ty),
            TypeDef::Variant(variant) => variant.type_name(ty),
            TypeDef::Sequence(_) => todo!(),
            TypeDef::Array(_) => todo!(),
            TypeDef::Tuple(_) => todo!(),
            TypeDef::Primitive(primitive) => primitive.type_name(ty)
        }
    }

    fn generate_type(&self, tokens: &mut TokenStream2, ty: &Type<CompactForm<S>>, types: &RegistryReadOnly<S>) {
        match self {
            TypeDef::Composite(composite) => composite.generate_type(tokens, ty, types),
            TypeDef::Variant(variant) => variant.generate_type(tokens, ty, types),
            TypeDef::Sequence(_) => {}
            TypeDef::Array(_) => {}
            TypeDef::Tuple(_) => {}
            TypeDef::Primitive(primitive) => primitive.generate_type(tokens, ty, types)
        }
    }
}

impl<S> GenerateType<S> for TypeDefComposite<CompactForm<S>>
where
    S: FormString + From<&'static str> + IdentFragment,
{
    fn type_name(&self, ty: &Type<CompactForm<S>>) -> S {
        ty.path().ident().expect("structs should have a name")
    }

    fn generate_type(&self, tokens: &mut TokenStream2, ty: &Type<CompactForm<S>>, types: &RegistryReadOnly<S>) {
        let type_name = format_ident!("{}", self.type_name(ty));
        let fields = type_fields(self.fields(), types);
        let ty_toks =
            quote! {
                pub struct #type_name #fields
            };
        tokens.extend(ty_toks);
    }
}

impl<S> GenerateType<S> for TypeDefVariant<CompactForm<S>>
where
    S: FormString + From<&'static str> + IdentFragment,
{
    fn type_name(&self, ty: &Type<CompactForm<S>>) -> S {
        ty.path().ident().expect("enums should have a name")
    }

    fn generate_type(&self, tokens: &mut TokenStream2, ty: &Type<CompactForm<S>>, types: &RegistryReadOnly<S>) {
        let type_name = format_ident!("{}", self.type_name(ty));
        let variants = self.variants().iter()
            .map(|variant| {
                let variant_name = format_ident!("{}", variant.name());
                let fields = if variant.fields().is_empty() {
                    quote! { }
                } else {
                    type_fields(variant.fields(), types)
                };
                quote! {
                    #variant_name #fields
                }
            });
        let ty_toks = quote! {
            pub enum #type_name {
                #( #variants )*,
            }
        };
        tokens.extend(ty_toks);
    }
}

impl<S> GenerateType<S> for TypeDefPrimitive
where
    S: FormString + IdentFragment + From<&'static str>,
{
    fn type_name(&self, _ty: &Type<CompactForm<S>>) -> S {
        match self {
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
        }.into()
    }

    fn generate_type(&self, _tokens: &mut TokenStream2, _ty: &Type<CompactForm<S>>, _types: &RegistryReadOnly<S>) {
    }
}

fn type_fields<S>(fields: &[Field<CompactForm<S>>], types: &RegistryReadOnly<S>) -> TokenStream2
where
    S: FormString + IdentFragment + From<&'static str>,
{
    let named = fields.iter().all(|f| f.name().is_some());
    let unnamed = fields.iter().all(|f| f.name().is_none());
    if named {
        let fields =
            fields
                .iter()
                .map(|field| {
                    let name = format_ident!("{}", field.name().expect("named field without a name"));
                    let ty = types.resolve(field.ty().id()).expect("type not resolved");
                    let ty = format_ident!("{}", ty.type_name(&ty));
                    quote! { pub #name: #ty }
                });
        quote! {
            {
                #( #fields, )*
            }
        }
    } else if unnamed {
        let fields =
            fields
                .iter()
                .map(|field| {
                    let ty = types.resolve(field.ty().id()).expect("type not resolved");
                    let ty = format_ident!("{}", ty.type_name(&ty));
                    quote! { pub #ty }
                });
        quote! {
            (
                #( #fields, )*
            );
        }
    } else {
        panic!("Fields must be either all named or all unnamed")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use scale_info::{Registry, TypeInfo, meta_type};

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

        let types = generate("root",&registry.into());

        assert_eq!(types.to_string(), quote! {
            mod root {
                pub struct S {
                    pub a: bool,
                    pub b: u32,
                    pub c: char,
                }
            }
        }.to_string())
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

        let types = generate("root",&registry.into());

        assert_eq!(types.to_string(), quote! {
            mod root {
                pub struct Parent {
                    pub a: bool,
                    pub b: Child,
                }

                pub struct Child {
                    pub a: i32,
                }
            }
        }.to_string())
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

        let types = generate("root",&registry.into());

        assert_eq!(types.to_string(), quote! {
            mod root {
                pub struct Parent(pub bool, pub Child,);
                pub struct Child(pub i32,);
            }
        }.to_string())
    }
}


