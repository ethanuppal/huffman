// Copyright (C) 2024 Etham Uppal. All rights reserved.

use baa::{BitVecOps, BitVecValue};

pub fn is_prefix_code(code: &[(char, BitVecValue)]) -> bool {
    for i in 0..code.len() {
        for j in (i + 1)..code.len() {
            let bit_str_i = code[i].1.to_bit_str();
            let bit_str_j = code[j].1.to_bit_str();
            if bit_str_i.starts_with(&bit_str_j)
                || bit_str_j.starts_with(&bit_str_i)
            {
                return false;
            }
        }
    }
    true
}
