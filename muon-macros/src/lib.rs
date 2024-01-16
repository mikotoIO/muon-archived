use quote::quote;
use syn::{parse_macro_input, DeriveInput, Fields};

use crate::type_repr::ScyllaType;

mod type_repr;

fn testy() {
    const LEFT: &'static [&'static str] = &["Hello", "World", "!"];
}

#[proc_macro_derive(Entity)]
pub fn entity_macro_derive(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(tokens as DeriveInput);

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

    let field_cql = fields
        .iter()
        .map(|x| format!("    {} {},", x.0, x.1.to_cql_type()))
        .collect::<Vec<_>>()
        .join("\n");
    let pkey_statement = "    PRIMARY KEY (id)";
    println!(
        "CREATE TABLE {} {{\n{}\n{}\n}}",
        name, field_cql, pkey_statement
    );
    let field_names = fields.iter().map(|x| x.0.clone()).collect::<Vec<_>>();
    let field_types = fields
        .iter()
        .map(|x| x.1.to_cql_type())
        .collect::<Vec<_>>();

    let name_str = name.to_string();

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
        }
    }
    .into()
}
