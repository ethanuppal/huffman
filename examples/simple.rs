// Copyright (C) 2024 Etham Uppal. All rights reserved.

use huffman_idk as huffman;

fn main() {
    println!(
        "{:?}",
        huffman::compute_prefix_code(&[('a', 0.25), ('b', 0.25), ('c', 0.5)])
    );
}
