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
        a: [usize; n],
        b: [usize; n],
    }
    let max_a = *a.iter().max().unwrap();
    let min_b = *b.iter().min().unwrap();


    if max_a > min_b {
        println!("0");
    }
    else {
        println!("{}", 1 + min_b - max_a);
    }


}