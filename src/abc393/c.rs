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
        m: usize,
    }
    // 自己ループ
    // 多重辺
    let mut g = vec![BTreeSet::new(); n];
    let mut ans: usize = 0;
    // let mut u = vec![];
    // let mut v = vec![];
    for i in 0..m {
        input!{
            ui: Usize1,
            vi: Usize1,
        }
        if g[ui].contains(&vi) {
            ans += 1;
            continue
        }
        if ui == vi {
            ans += 1;
            continue
        }
        g[ui].insert(vi);
        g[vi].insert(ui);
        // u.push(ui);
        // v.push(vi);
    }
    println!("{}", ans);

}