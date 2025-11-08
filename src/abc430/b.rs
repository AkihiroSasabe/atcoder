#![allow(dead_code, unused_imports)]
use proconio::{input, marker::{Usize1, Isize1, Chars}};
use itertools::Itertools;
use std::cmp::{max, min, Ordering, Reverse};
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::collections::{VecDeque, BinaryHeap, HashMap, BTreeMap, HashSet, BTreeSet};
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
use rand::Rng;
use num::{BigUint, ToPrimitive, Integer};
use num_bigint::ToBigUint;
fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
    }
    // 10^4通り
    let mut set = BTreeSet::new();
    for ys in 0..n-m+1 {
        for xs in 0..n-m+1 {
            let mut hash = vec![];
            for y in ys..ys+m {
                let mut mask = 0;
                for x in xs..xs+m {
                    if s[y][x] == '#' {
                        mask += 1 << (x - xs);
                    }
                }
                hash.push(mask);
            }
            set.insert(hash);
        }
    }
    println!("{}", set.len());


}