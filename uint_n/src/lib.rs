extern crate proc_macro;
use proc_macro::*;
use quote::quote;
use syn::*;

use common::*;

#[proc_macro_attribute]
pub fn field(attr: TokenStream, item: TokenStream) -> proc_macro::TokenStream {
    let item_ast: DeriveInput = parse(item.clone()).unwrap();
    let struct_name = &item_ast.ident;
    let mod_str = attr.into_iter().next().unwrap().to_string();
    let mod_str_len = mod_str.len();
    let num_bits = mod_str_len * 4;
    let bytes = (num_bits + 7) / 8;

    let struct_def = build_struct(struct_name, bytes, num_bits);
    let item_impl = impl_struct_common(struct_name, bytes);
    let struct_def_special = quote! {
        impl #struct_name {
            /// Get the largest number that can be represented by this integer type.
            fn max() -> Self {
                (Self::mod_val() - BigUint::from(1u16)).into()
            }
            /// Get the mod value of this integer type.
            fn mod_val() -> BigUint {
                // TODO: make safe?
                BigUint::from_str_radix(#mod_str, 16).unwrap()
            }
        }
    };

    let new_item = quote! {
        #struct_def
        #item_impl
        #struct_def_special
    };

    new_item.into()
}
