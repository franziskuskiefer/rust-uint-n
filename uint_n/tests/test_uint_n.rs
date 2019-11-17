#[allow(unused_imports)]
extern crate uint_uint_n;
use uint_traits::Field;
use uint_uint_n::field;

extern crate num;
use num::{BigUint, Num};
use std::ops::*;

#[test]
fn basic_uint_test() {
    #[field(7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffed)]
    struct Mod25519;
}
