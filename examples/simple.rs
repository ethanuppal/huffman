// Copyright (C) 2024 Etham Uppal. All rights reserved.

fn main() {
    println!(
        "{:?}",
        huffman::compute_prefix_code(&[('a', 0.25), ('b', 0.25), ('c', 0.5)])
    );
}
