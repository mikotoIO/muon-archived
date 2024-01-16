use quote::quote;
use syn::{parse_macro_input, DeriveInput, Fields};

use crate::type_repr::ScyllaType;
use darling::FromDeriveInput;

mod type_repr;

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(entity))]
struct EntityMacroOptions {
    pkey: Option<String>,
}

#[proc_macro_derive(Entity, attributes(entity))]
pub fn entity_macro_derive(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(tokens as DeriveInput);
    let opts = EntityMacroOptions::from_derive_input(&input).expect("Wrong options");

    let name = input.ident.clone();

    let input = match input.data {
        syn::Data::Struct(x) => x.fields,
        _ => unreachable!(),
    };

    let entity_fields = match input {
        Fields::Named(x) => x.named.into_iter().collect::<Vec<_>>(),
        _ => unreachable!(),
    };
    let fields = entity_fields
        .into_iter()
        .map(|x| (x.ident.unwrap().to_string(), ScyllaType::from(x.ty)))
        .collect::<Vec<_>>();

    let field_names = fields.iter().map(|x| x.0.clone()).collect::<Vec<_>>();
    let field_types = fields.iter().map(|x| x.1.to_cql_type()).collect::<Vec<_>>();

    let name_str = name.to_string();

    let pkey = opts.pkey.unwrap_or_else(|| "id".to_string());

    quote! {
        impl muon_core::Entity for #name {
            fn scratch() {
                println!("lol:D");
            }

            fn __entity_name() -> &'static str {
                #name_str
            }

            fn __entity_fields() -> &'static [&'static str] {
                const FIELDS: &'static [&'static str] = &[#(#field_names),*];
                FIELDS
            }

            fn __entity_field_types() -> &'static [&'static str] {
                const FIELD_TYPES: &'static [&'static str] = &[#(#field_types),*];
                FIELD_TYPES
            }

            fn __entity_meta() -> &'static muon_core::EntityMetadata {
                const PKEY: muon_core::EntityMetadata = muon_core::EntityMetadata { pkey: #pkey };
                &PKEY
            }
        }
    }
    .into()
}
