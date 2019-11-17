extern crate proc_macro;
use proc_macro::*;
use quote::quote;
use syn::*;

use common::*;

#[proc_macro_attribute]
pub fn bits(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_ast: DeriveInput = parse(item.clone()).unwrap();
    let struct_name = &item_ast.ident;
    let num_bits =
        usize::from_str_radix(&attr.into_iter().next().unwrap().to_string(), 10).unwrap();
    let bytes = (num_bits + 7) / 8;

    let struct_def = build_struct(struct_name, bytes, num_bits);
    let item_impl = impl_struct_common(struct_name, bytes);
    let struct_def_special = quote! {
        impl #struct_name {
            /// Get the largest number that can be represented by this integer type.
            fn max() -> Self {
                (BigUint::from(2u32).shl(#num_bits) - BigUint::from(1u16)).into()
            }
            /// Get the mod value of this integer type.
            fn mod_val() -> BigUint {
                BigUint::from(1u8) << #num_bits
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
