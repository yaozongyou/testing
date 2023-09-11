#![feature(proc_macro_span)]

use proc_macro::{Span, TokenStream};
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{parse_macro_input, MetaNameValue, Result};

struct TestingMeta(Punctuated<MetaNameValue, Comma>);

impl Parse for TestingMeta {
    fn parse(input: ParseStream) -> Result<Self> {
        let meta_items = input.parse_terminated(MetaNameValue::parse, Comma)?;
        Ok(TestingMeta(meta_items))
    }
}

#[proc_macro_attribute]
pub fn testing(attr: TokenStream, input: TokenStream) -> TokenStream {
    let span = Span::call_site();
    let source = span.source_file();
    let module = source.path().file_stem().unwrap().to_str().unwrap().to_owned();
    let testing_meta = parse_macro_input!(attr as TestingMeta);
    let comment = get_testing_meta(&testing_meta, "comment");
    let source_file = span.source_file().path().into_os_string().into_string().unwrap();
    let start_line = span.start().line;
    let function_name = syn::parse::<syn::ItemFn>(input.clone()).unwrap().sig.ident;

    let submit = quote! {
        inventory::submit! {
            testing::TestDescAndFn::new(std::concat!(#module, ".", stringify!(#function_name)),
            #comment,
            #source_file,
            #start_line,
            #function_name)
        }
    };

    let input = proc_macro2::TokenStream::from(input);

    let tokens = quote! {
        #input

        #submit
    };

    tokens.into()
}

fn get_testing_meta(testing_meta: &TestingMeta, name: &str) -> Option<syn::Expr> {
    for meta in &testing_meta.0 {
        for segment in &meta.path.segments {
            if segment.ident == name {
                return Some(meta.value.clone());
            }
        }
    }

    None
}
