#![allow(clippy::unwrap_used)]

use std::any::Any;

use assert2::assert;
use bool_ext::BoolExt;
use crate::identity;

#[allow(unused_imports)]
use super::*;
#[test]
fn compose_transforms_input_a_to_expected_output_c() {
    /* Given `i32` (A) -> `bool` (B) and `bool` (B) -> `String` (C) */
    const fn is_even(a: i32) -> bool {
        a % 2 == 0
    }

    fn bool_to_string(value: bool) -> String {
        value.map("false", "true").into()
    }

    let a2b = is_even;
    let b2c = bool_to_string;
    let a = 42;

    let expected_res = (a % 2 == 0).to_string();
    let sut = compose;

    /* When */
    let res = sut(a2b, b2c)(a);

    /* Then */
    assert!(res == expected_res);
}

#[test]
fn compose_respects_identity() {
    /* Given */
    struct Foo<F>(F) where F: Fn();
    let unique_type = Foo(|| {});

    // Snapshot `unique_type`'s `TypeId`
    let expected_type_id = unique_type.type_id();

    // Prove `identity` works by first establishing even "identical" distinct Voldemort types have
    // distinct `TypeId`'s
    let distinct_type = Foo(|| {});
    assert!(unique_type.type_id() != distinct_type.type_id());

    let id_1 = identity;
    let id_2 = identity;
    let sut = compose;

    /* When */
    let res = sut(id_1, id_2)(unique_type);

    /* Then */
    assert!(res.type_id() == expected_type_id);
}