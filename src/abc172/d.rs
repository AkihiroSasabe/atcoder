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
    }
    let mut f = vec![1; n+1];
    // 1 はf(X) のどのXに寄与するか? (全X)
    // 2 はf(X) のどのXに寄与するか? N/2個

    for i in 2..n+1 {
        let mut temp = i;
        while temp <= n {
            f[temp] += 1;
            temp += i;
        }
    }

    let mut ans = 0;
    for k in 1..n+1 {
        ans += k * f[k];
    }
    println!("{}", ans);


}