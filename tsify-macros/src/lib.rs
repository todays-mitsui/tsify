mod attrs;
mod container;
mod ctxt;
mod parser;
mod type_alias;
mod typescript;
mod wasm_bindgen;

use container::Container;
use parser::Parser;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_attribute]
pub fn tsify(
    attrs: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item: syn::Item = parse_macro_input!(item);
    let attrs = proc_macro2::TokenStream::from(attrs);

    match item {
        syn::Item::Type(item_type) => type_alias::expend(item_type),
        _ => Err(
            darling::Error::custom("#[tsify] can only be applied to type alias.").with_span(&attrs),
        ),
    }
    .unwrap_or_else(|err| err.write_errors())
    .into()
}

#[proc_macro_derive(Tsify, attributes(tsify, serde))]
pub fn expand_tsify(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let item: DeriveInput = parse_macro_input!(input);

    expand(item).unwrap_or_else(|err| err.write_errors()).into()
}

fn expand(input: DeriveInput) -> Result<proc_macro2::TokenStream, darling::Error> {
    let cont = Container::from_derive_input(&input)?;

    let parser = Parser::new(&cont);
    let decl = parser.parse();

    let (impl_generics, ty_generics, where_clause) = cont.generics().split_for_impl();

    let ident = cont.ident();
    let decl_str = decl.to_string();

    let mut tokens = quote! {
        #[automatically_derived]
        impl #impl_generics Tsify for #ident #ty_generics #where_clause {
            const DECL: &'static str = #decl_str;
        }
    };

    if cfg!(feature = "wasm-bindgen-impl") {
        tokens.extend(wasm_bindgen::expand(&cont, decl));
    }

    cont.check()?;

    Ok(tokens)
}
