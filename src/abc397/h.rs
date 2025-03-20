#![allow(dead_code, unused_imports)]
use proconio::{input, marker::{Usize1, Isize1}};
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::BTreeMap;
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::collections::{HashSet, BTreeSet};
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
use rand::Rng;
use num::{BigUint, ToPrimitive};
use num_bigint::ToBigUint;
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut ans = 0;
    for i in 1..n-1 {
        for j in i+1..n {
            let mut set0: BTreeSet<_> = a[0..i].iter().cloned().collect();
            let mut set1: BTreeSet<_> = a[i..j].iter().cloned().collect();
            let mut set2: BTreeSet<_> = a[j..n].iter().cloned().collect();
            let temp = set0.len() + set1.len() + set2.len();
            ans = max(ans, temp);
        }
    }
    println!("{}", ans);

}