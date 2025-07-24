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
    // 2025-07-19 21:04-21:39 (35min)
    // 2025-07-20 21:39-21:50 (11min)
    // Total: 46min
    input! {
        mut n: i128,
        m: usize,
    }
    let mut a = vec![];
    let mut b = vec![];
    // let mut diffs = vec![];
    let mut diffs = BTreeMap::new();
    for i in 0..m {
        input!{
            ai: i128, // - 空き瓶
            bi: i128, // + コーラ入りビン
        }
        a.push(ai);
        b.push(bi);
        diffs.entry(ai).or_insert(BinaryHeap::new()).push(Reverse((ai-bi)));
    }
    // diffs.sort();
    // 小さいai順に。
    let mut btree = BTreeMap::new();
    let mut min_diff = 1 << 60;
    for (ai, heap) in diffs {
        if let Some(Reverse(diff)) = heap.peek() {
            min_diff = min(min_diff, *diff);
        }
        btree.insert(ai, min_diff);
    }

    let mut ans = 0;
    while n > 0 {
        // println!("n = {:?}", n);
        if let Some((ai, diff)) = btree.range(..=n).rev().next() {
            let num = (n - ai) / diff + 1;
            n -= diff * num;
            ans += num;
        }
        else {
            break
        }
    }
    println!("{}", ans);


}
