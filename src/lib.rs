extern crate proc_macro;
use proc_macro::*;
use quote::quote;
use syn::*;

#[proc_macro_attribute]
pub fn bits(attr: TokenStream, item: TokenStream) -> proc_macro::TokenStream {
    let item_ast: DeriveInput = parse(item.clone()).unwrap();
    let struct_name = &item_ast.ident;
    let num_bits =
        usize::from_str_radix(&attr.into_iter().next().unwrap().to_string(), 10).unwrap();
    let bytes = (num_bits + 7) / 8;

    let new_item = quote! {
        #[derive(Clone, Copy)]
        struct #struct_name ([u8; #bytes]);

        impl #struct_name {
            pub fn new() -> Self {
                Self([0u8; #bytes])
            }

            fn max() -> BigUint {
                BigUint::from(2u32).shl(#num_bits)
            }

            fn mod_val() -> BigUint {
                BigUint::from(1u8) << #num_bits
            }

            pub fn bits() -> usize {
                #num_bits
            }

            pub fn raw(&self) -> [u8; #bytes] {
                self.0
            }

            pub fn bit(&self, i: usize) -> u8 {
                let tmp: BigUint = (*self >> i).into();
                (tmp & BigUint::from(1u128)).to_bytes_le()[0]
            }

            #[allow(dead_code)]
            pub fn from_literal(x: u128) -> Self {
                let big_x = BigUint::from(x);
                if big_x > #struct_name::max().into() {
                    panic!("literal too big for type {}", stringify!(#struct_name));
                }
                big_x.into()
            }

            #[allow(dead_code)]
            pub fn from_hex(x: &str) -> Self {
                let big_x = BigUint::from_str_radix(x, 16)
                    .unwrap_or_else(|_| panic!("string is not a valid hex number {}", x));
                if big_x > #struct_name::max().into() {
                    panic!("literal too big for type {}", stringify!(#struct_name));
                }
                big_x.into()
            }

            /// Returns 2 to the power of the argument
            #[allow(dead_code)]
            pub fn pow2(x: usize) -> #struct_name {
                BigUint::from(1u32).shl(x).into()
            }

            /// Returns self to the power of the argument
            #[allow(dead_code)]
            pub fn pow(&self, exp: &#struct_name) -> #struct_name {
                let a: BigUint = (*self).into();
                let b: BigUint = (*exp).into();
                let m: BigUint = #struct_name::mod_val().into();
                let c: BigUint = a.modpow(&b, &m);
                c.into()
            }

            /// Returns self^-1
            #[allow(dead_code)]
            pub fn inv(&self) -> #struct_name {
                let m = #struct_name::mod_val()-BigUint::from(2u32);
                let s: BigUint = (*self).into();
                s.modpow(&m, &#struct_name::mod_val()).into()
            }

            #[allow(dead_code)]
            pub fn to_bytes_le(&self) -> Vec<u8> {
                BigUint::from_bytes_be(&self.0).to_bytes_le()
            }

            #[allow(dead_code)]
            pub fn from_bytes_le(v: &[u8]) -> #struct_name {
                BigUint::from_bytes_le(v).into()
            }
        }

        impl From<BigUint> for #struct_name {
            fn from(x: BigUint) -> #struct_name {
                let x = x % #struct_name::mod_val();
                let repr = x.to_bytes_be();
                if repr.len() > #bytes {
                    panic!("BigUint too big for type {}", stringify!(#struct_name))
                }
                let mut out = [0u8; #bytes];
                let upper = out.len();
                let lower = upper - repr.len();
                out[lower..upper].copy_from_slice(&repr);
                #struct_name(out)
            }
        }

        impl From<u128> for #struct_name {
            fn from(x: u128) -> #struct_name {
                #struct_name::from_literal(x)
            }
        }

        impl From<&str> for #struct_name {
            fn from(x: &str) -> #struct_name {
                #struct_name::from_hex(x)
            }
        }

        impl Into<BigUint> for #struct_name {
            fn into(self) -> BigUint {
                BigUint::from_bytes_be(&self.0)
            }
        }

        impl std::fmt::Display for #struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                let uint: BigUint = (*self).into();
                write!(f, "{}", uint)
            }
        }

        impl std::fmt::Debug for #struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                let uint: BigUint = (*self).into();
                write!(f, "{}", uint)
            }
        }

        impl std::cmp::PartialEq for #struct_name {
            fn eq(&self, rhs: &#struct_name) -> bool {
                let a: BigUint = (*self).into();
                let b: BigUint = (*rhs).into();
                a == b
            }
        }

        impl Eq for #struct_name {}

        impl PartialOrd for #struct_name {
            fn partial_cmp(&self, other: &#struct_name) -> Option<std::cmp::Ordering> {
                let a: BigUint = (*self).into();
                let b: BigUint = (*other).into();
                a.partial_cmp(&b)
            }
        }

        impl Ord for #struct_name {
            fn cmp(&self, other: &#struct_name) -> std::cmp::Ordering {
                self.partial_cmp(other).unwrap()
            }
        }

        /// **Warning**: wraps on overflow.
        impl Add for #struct_name {
            type Output = #struct_name;
            fn add(self, rhs: #struct_name) -> #struct_name {
                let a: BigUint = self.into();
                let b: BigUint = rhs.into();
                let c: BigUint = a + b;
                let d: BigUint = c % #struct_name::mod_val();
                d.into()
            }
        }

        /// **Warning**: wraps on underflow.
        impl Sub for #struct_name {
            type Output = #struct_name;
            fn sub(self, rhs: #struct_name) -> #struct_name {
                let a: BigUint = self.into();
                let b: BigUint = rhs.into();
                let c: BigUint = if b > a {
                    #struct_name::mod_val() - b + a
                } else {
                    a - b
                };
                c.into()
            }
        }

        /// **Warning**: wraps on overflow.
        impl Mul for #struct_name {
            type Output = #struct_name;
            fn mul(self, rhs: #struct_name) -> #struct_name {
                let a: BigUint = self.into();
                let b: BigUint = rhs.into();
                let c: BigUint = a * b;
                let d: BigUint = c % #struct_name::mod_val();
                d.into()
            }
        }

        /// **Warning**: panics on division by 0.
        impl Div for #struct_name {
            type Output = #struct_name;
            fn div(self, rhs: #struct_name) -> #struct_name {
                let a: BigUint = self.into();
                let b: BigUint = rhs.into();
                let c: BigUint = a / b;
                c.into()
            }
        }

        /// **Warning**: panics on division by 0.
        impl Rem for #struct_name {
            type Output = #struct_name;
            fn rem(self, rhs: #struct_name) -> #struct_name {
                let a: BigUint = self.into();
                let b: BigUint = rhs.into();
                let c: BigUint = a % b;
                c.into()
            }
        }

        impl Shr<usize> for #struct_name {
            type Output = #struct_name;
            fn shr(self, rhs: usize) -> #struct_name {
                let a: BigUint = self.into();
                let a = a >> rhs;
                a.into()
            }
        }

        impl Shl<usize> for #struct_name {
            type Output = #struct_name;
            fn shl(self, rhs: usize) -> #struct_name {
                let a: BigUint = self.into();
                let a = a << rhs;
                a.into()
            }
        }

        impl Index<usize> for #struct_name {
            type Output = u8;
            fn index(&self, i: usize) -> &u8 {
                &self.0[i]
            }
        }
    };

    let new_item = TokenStream::from(new_item);
    new_item
}
