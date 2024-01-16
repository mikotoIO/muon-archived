use quote::quote;
use syn::{parse_macro_input, DeriveInput, Fields};

mod type_repr;

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
        .map(|x| {
            (
                x.ident.unwrap().to_string(),
                type_repr::to_scylla_type(x.ty),
            )
        })
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

    quote! {
        impl muon_core::Entity for #name {
            fn scratch() {
                println!("lol:D");
            }
        }
    }
    .into()
}
