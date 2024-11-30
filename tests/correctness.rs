// Copyright (C) 2024 Etham Uppal. All rights reserved.

use proptest::prelude::*;

use huffman_idk as huffman;

mod common;

#[derive(Debug)]
struct Alphabet(Vec<(char, f64)>);

impl Arbitrary for Alphabet {
    type Parameters = ();
    type Strategy = BoxedStrategy<Alphabet>;

    fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
        proptest::collection::vec(
            proptest::char::any(),
            proptest::collection::size_range(0..500),
        )
        .prop_flat_map(|chars| {
            proptest::collection::vec(0.0..1.0, chars.len()).prop_map(
                move |mut probs| {
                    let total = probs.iter().sum::<f64>();
                    probs.iter_mut().for_each(|p| *p /= total);

                    Self(chars.iter().cloned().zip(probs).collect())
                },
            )
        })
        .boxed()
    }
}

proptest! {
    #[test]
    fn check_prefix_code(alphabet: Alphabet) {
        let prefix_code = huffman::compute_prefix_code(&alphabet.0);
        assert!(
            common::is_prefix_code(&prefix_code),
            "{:?} was not a prefix code",
            prefix_code
        );
    }
}
