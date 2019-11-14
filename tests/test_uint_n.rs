#[allow(unused_imports)]
extern crate uint_n;
use uint_n::{bits, field};

extern crate num;
use num::{BigUint, Num};
use std::ops::*;

#[test]
fn basic_uint_test() {
    #[field(7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffed)]
    struct Mod25519;
    #[bits(256)]
    struct Uint256;

    let x1 = Uint256::from(254);
    let x2 = Uint256::from(3);
    assert_eq!(Uint256::from(257), x1 + x2);

    assert_eq!(
        Uint256::from("8000000000000000000000000000000000000000000000000000000000000000"),
        Uint256::from(1) << 255
    );
    assert_eq!(
        Uint256::from("7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"),
        Uint256::from("8000000000000000000000000000000000000000000000000000000000000000")
            - Uint256::from(1)
    );
    assert_eq!(
        Uint256::from("ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"),
        Uint256::from(0) - Uint256::from(1)
    );
    assert_eq!(
        Uint256::from(1),
        Uint256::from("ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff")
            + Uint256::from(2)
    );

    let x3 = Uint256::from(1) << 256;
    assert_eq!(Uint256::from(0), x3);

    let x4 = Uint256::from(3) << 255;
    assert_eq!(
        Uint256::from("8000000000000000000000000000000000000000000000000000000000000000"),
        x4
    );
}
