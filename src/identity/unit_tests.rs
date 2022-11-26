#![allow(clippy::unwrap_used)]

use std::any::Any;
use assert2::assert;
#[allow(unused_imports)]
use super::*;

#[test]
fn identity_returns_self() {
    /* Given an impossible-to-reconstruct Voldemort type (every instance has an unnameable type and
    a unique`TypeId`) */

    struct Foo<F>(F) where F: Fn();
    let unique_type = Foo(|| {});

    // Snapshot `unique_type`'s `TypeId`
    let expected_type_id = unique_type.type_id();

    // Prove `identity` works by first establishing even "identical" distinct Voldemort types have
    // distinct`TypeId`'s
    let distinct_type = Foo(|| {});
    assert!(unique_type.type_id() != distinct_type.type_id());

    let sut = identity;

    /* When we give `unique_type` to `identity()` (move operation) */
    let res = sut(unique_type);

    /* Then assert we received the very same type back from `identity()` */
    assert!(res.type_id() == expected_type_id);
}