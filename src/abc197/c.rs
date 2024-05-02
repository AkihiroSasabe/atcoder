#![allow(dead_code, unused_imports)]
use proconio::input;
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
fn main() {
    // 12min
    input! {
        n: usize,
        a: [usize; n]
    }

    // 平方分割? 半分全列挙?
    // 1048576
    // 全探索いけるのでは?
    let mut ans = 1 << 60;
    for mask in 0..1<<n-1 {

        let mut ors = vec![];
        ors.push(a[0]);
        for i in 0..n-1 {
            if mask & (1<<i) != 0 {
                ors.push(a[i+1]);
            }
            else {
                let last = ors.len()-1;
                ors[last] = ors[last] | a[i+1];;
            }
        }
        let mut xor = 0;
        for i in 0..ors.len() {
            xor ^= ors[i];
        }
        ans = min(ans, xor);
    }
    println!("{}", ans);

}