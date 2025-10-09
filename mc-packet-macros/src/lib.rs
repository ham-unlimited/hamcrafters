#![deny(missing_docs)]
#![forbid(unsafe_code)]

//! Utility macros for the mac-packet crate.
use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemStruct, LitInt, parse_macro_input};

/// Implements the [McPacket] trait on the provided struct with the [PACKET_ID] set to the provided value.
#[proc_macro_attribute]
pub fn mc_packet(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as LitInt);
    let packet_id: i32 = args.base10_parse().expect("Arg must be a valid usize");

    let input_ast = parse_macro_input!(input as ItemStruct);
    let struct_name = input_ast.ident.clone();

    let generated = quote! {
        #input_ast

        impl McPacket for #struct_name {
            const PACKET_ID: i32 = #packet_id;
        }
    };

    TokenStream::from(generated)
}
