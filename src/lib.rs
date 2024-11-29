// Copyright (C) 2024 Etham Uppal. All rights reserved.

use std::{cmp, collections::BinaryHeap};

use baa::BitVecValue;

struct Letter {
    index: usize,
    frequency: f64,
}

impl PartialEq for Letter {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index && self.frequency == other.frequency
    }
}

impl Eq for Letter {}

impl PartialOrd for Letter {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Letter {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.frequency.total_cmp(&other.frequency)
    }
}

#[derive(Debug)]
enum Node {
    Leaf { value: char },
    Interior { left: usize, right: usize },
}

fn construct_prefix_code(
    result: &mut Vec<(char, BitVecValue)>,
    tree: &[Node],
    current: usize,
    code: u64,
    depth: u32,
) {
    match &tree[current] {
        Node::Leaf { value } => {
            result.push((*value, BitVecValue::from_u64(code, depth)));
        }
        Node::Interior { left, right } => {
            #[allow(clippy::identity_op)]
            construct_prefix_code(
                result,
                tree,
                *left,
                (code << 1) | 0,
                depth + 1,
            );
            construct_prefix_code(
                result,
                tree,
                *right,
                (code << 1) | 1,
                depth + 1,
            );
        }
    }
}

pub fn compute_prefix_code(
    alphabet: &[(char, f64)],
) -> Vec<(char, BitVecValue)> {
    if alphabet.len() <= 2 {
        alphabet
            .iter()
            .enumerate()
            .map(|(index, (value, _))| {
                (*value, BitVecValue::from_u64(index as u64, 1))
            })
            .collect()
    } else {
        let mut tree = alphabet
            .iter()
            .cloned()
            .map(|(value, _)| Node::Leaf { value })
            .collect::<Vec<_>>();
        let mut letters =
            BinaryHeap::from_iter((0..alphabet.len()).map(|index| {
                cmp::Reverse(Letter {
                    index,
                    frequency: alphabet[index].1,
                })
            }));

        while !letters.is_empty() {
            let cmp::Reverse(first) = letters.pop().expect("invariant");
            let cmp::Reverse(second) = letters.pop().expect("invariant");

            tree.push(Node::Interior {
                left: first.index,
                right: second.index,
            });

            if !letters.is_empty() {
                letters.push(cmp::Reverse(Letter {
                    index: tree.len() - 1,
                    frequency: first.frequency + second.frequency,
                }));
            }
        }

        let mut result = vec![];

        assert!(!tree.is_empty());
        construct_prefix_code(&mut result, &tree, tree.len() - 1, 0, 0);

        result
    }
}
