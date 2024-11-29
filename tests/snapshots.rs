// Copyright (C) 2024 Etham Uppal. All rights reserved.

use baa::BitVecValue;
use insta::assert_debug_snapshot;

mod common;

fn helper(alphabet: &[(char, f64)]) -> Vec<(char, BitVecValue)> {
    let code = huffman::compute_prefix_code(alphabet);
    assert!(common::is_prefix_code(&code));
    code
}

#[test]
fn few_characters() {
    assert_debug_snapshot!(helper(&[('a', 1.0),]));

    assert_debug_snapshot!(helper(&[('a', 0.5), ('b', 0.5),]));
}

#[test]
fn slow_path() {
    assert_debug_snapshot!(helper(&[('a', 0.25), ('b', 0.25), ('c', 0.5)]));

    assert_debug_snapshot!(helper(&[
        ('a', 0.2),
        ('b', 0.2),
        ('c', 0.2),
        ('b', 0.2),
        ('c', 0.2)
    ]));
}

#[test]
fn weird_cases() {
    assert_debug_snapshot!(helper(&[('a', 0.0), ('b', 1.0),]));

    assert_debug_snapshot!(helper(&[('a', 0.0), ('b', 0.5), ('c', 0.5)]));

    assert_debug_snapshot!(helper(&[
        ('a', 0.0),
        ('b', 0.0),
        ('c', 0.0),
        ('d', 0.5),
        ('e', 0.5)
    ]));
}
