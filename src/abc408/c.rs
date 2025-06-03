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
    }
    let mut l = vec![];
    let mut r = vec![];
    for i in 0..m {
        input!{
            li: Usize1,
            ri: Usize1,
        }
        l.push(li);
        r.push(ri);
    }
    let mut cum = vec![0; n];
    for i in 0..m {
        cum[l[i]] += 1;

        if r[i] + 1 < n {
            cum[r[i] + 1] -= 1;
        }
    }
    for i in 1..n {
        cum[i] += cum[i - 1];
    }

    let mut ans = m as isize;
    for i in 0..n {
        ans = min(ans, cum[i]);
    }
    println!("{}", ans);


}