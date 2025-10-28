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
    }
    let a_max = *a.iter().max().unwrap();
    let mut poss = vec![vec![]; a_max + 1];
    for i in 0..n {
        poss[a[i]].push(i);
    }
    let mut ans: usize = 0;

    for x in 1..=a_max {
        if poss[x].len() < 2 {
            continue;
        }
        let cont1 = poss[x].len() * (poss[x].len()-1) / 2;
        let cont2 = (n - poss[x].len());
        let cnt = cont1 * cont2;
        ans += cnt;
    }
    println!("{}", ans);









}