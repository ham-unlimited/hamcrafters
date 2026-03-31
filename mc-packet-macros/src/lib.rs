#![deny(missing_docs)]
#![forbid(unsafe_code)]

//! Utility macros for the mac-packet crate.
use proc_macro::TokenStream;
use quote::quote;
use syn::{Item, LitInt, parse_macro_input};

/// Implements the [McPacket] trait on the provided struct or enum with the [PACKET_ID] set to the provided value.
#[proc_macro_attribute]
pub fn mc_packet(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as LitInt);
    let packet_id: i32 = args.base10_parse().expect("Arg must be a valid i32");

    let input_ast = parse_macro_input!(input as Item);
    let item_name = match &input_ast {
        Item::Struct(s) => s.ident.clone(),
        Item::Enum(e) => e.ident.clone(),
        _ => panic!("#[mc_packet] can only be applied to structs or enums"),
    };

    let generated = quote! {
        #input_ast

        impl McPacket for #item_name {
            const PACKET_ID: i32 = #packet_id;
        }
    };

    TokenStream::from(generated)
}
