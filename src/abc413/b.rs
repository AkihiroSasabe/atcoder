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
        s: [Chars; n],
    }

    let mut set = HashSet::new();
    for i in 0..n {
        for j in 0..n {
            if i == j {continue;}
            let mut temp = vec![];
            for si in s[i].iter() {
                temp.push(*si);
            }
            for sj in s[j].iter() {
                temp.push(*sj);
            }
            set.insert(temp);
        }
    }
    println!("{}", set.len());
}