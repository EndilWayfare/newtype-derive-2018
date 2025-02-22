// Copyright (c) 2015 macro-attr contributors.
// Copyright (c) 2021 Warlock <internalmike@gmail.com>
//
// Licensed under the MIT license (see LICENSE or <http://opensource.org
// /licenses/MIT>) or the Apache License, Version 2.0 (see LICENSE of
// <http://www.apache.org/licenses/LICENSE-2.0>), at your option. All
// files in the project carrying such notice may not be copied, modified,
// or distributed except according to those terms.

#![deny(warnings)]
#![recursion_limit = "128"]

use macro_attr_2018::macro_attr;
use newtype_derive_2018::*;

macro_attr! {
    #[derive(
        Copy, Clone, Eq, PartialEq, Debug,
        NewtypeAdd!, NewtypeAdd!(&self, &Self), NewtypeAdd!(i32), NewtypeAdd!(&self, i32),
        NewtypeBitAnd!, NewtypeBitAnd!(&self, &Self),
        NewtypeBitOr!, NewtypeBitOr!(&self, &Self),
        NewtypeBitXor!, NewtypeBitXor!(&self, &Self),
        NewtypeDiv!, NewtypeDiv!(&self, &Self),
        NewtypeMul!, NewtypeMul!(&self, &Self),
        NewtypeRem!, NewtypeRem!(&self, &Self),
        NewtypeSub!, NewtypeSub!(&self, &Self),
        NewtypeShl!(), NewtypeShl!(&self, &Self), NewtypeShl!(usize), NewtypeShl!(&self, usize),
        NewtypeShr!(), NewtypeShr!(&self, &Self), NewtypeShr!(usize), NewtypeShr!(&self, usize),
        NewtypeNeg!, NewtypeNeg!(&self),
        NewtypeNot!, NewtypeNot!(&self),
    )]
    pub struct Dummy(i32);
}

#[test]
fn test_arith() {
    let a = Dummy(4);
    let b = Dummy(7);

    assert_eq!(a + b, Dummy(4 + 7));
    assert_eq!(&a + &b, Dummy(4 + 7));
    assert_eq!(a + 7, Dummy(4 + 7));
    assert_eq!(&a + 7, Dummy(4 + 7));
    assert_eq!(a & b, Dummy(4 & 7));
    assert_eq!(&a & &b, Dummy(4 & 7));
    assert_eq!(a | b, Dummy(4 | 7));
    assert_eq!(&a | &b, Dummy(4 | 7));
    assert_eq!(a ^ b, Dummy(4 ^ 7));
    assert_eq!(&a ^ &b, Dummy(4 ^ 7));
    assert_eq!(a / b, Dummy(4 / 7));
    assert_eq!(&a / &b, Dummy(4 / 7));
    assert_eq!(a * b, Dummy(4 * 7));
    assert_eq!(&a * &b, Dummy(4 * 7));
    assert_eq!(a % b, Dummy(4 % 7));
    assert_eq!(&a % &b, Dummy(4 % 7));
    assert_eq!(a - b, Dummy(4 - 7));
    assert_eq!(&a - &b, Dummy(4 - 7));

    assert_eq!(a << b, Dummy(4 << 7));
    assert_eq!(&a << &b, Dummy(4 << 7));
    assert_eq!(a << 7, Dummy(4 << 7));

    assert_eq!(a >> b, Dummy(4 >> 7));
    assert_eq!(&a >> &b, Dummy(4 >> 7));
    assert_eq!(a >> 7, Dummy(4 >> 7));

    assert_eq!(-a, Dummy(-4));
    assert_eq!(-&a, Dummy(-4));
    assert_eq!(!a, Dummy(!4));
    assert_eq!(!&a, Dummy(!4));
}
