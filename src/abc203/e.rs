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
    // 2025-03-29 13:17-14:22 (1h5min)
    input! {
        n: isize,
        m: usize,
    }
    let mut rows = BTreeMap::new();
    for i in 0..m {
        input!{
            yi: isize, // 直感的に、yとxは逆にしておく。
            xi: isize,
        }
        rows.entry(yi).or_insert(vec![]).push(xi);
    }

    // 到達可能な地点を、シミュレーションしていく。
    let mut reachables = BTreeSet::new();
    // スタート地点を追加
    reachables.insert(n);

    for (yi, row) in rows {
        // println!("yi = {:?} ----", yi);
        // println!("reachables = {:?}", reachables);
        let mut next_reach = BTreeSet::new();
        let mut next_unreach = BTreeSet::new();
        for xi in row {
            // yi, xi に到達可能か判定する。
            if reachables.contains(&(xi-1)) || reachables.contains(&(xi+1))  {
                next_reach.insert(xi);
            }
            else {
                // y = yi で、xiに到達できない場合
                if reachables.contains(&xi) {
                    next_unreach.insert(xi);
                }
            }
        }
        // println!("next_unreach = {:?}", next_unreach);
        // println!("next_reach = {:?}", next_reach);
        for v in next_unreach {
            reachables.remove(&v);
        }
        for v in next_reach {
            reachables.insert(v);
        }
    }
    // println!("reachables = {:?}", reachables);
    let ans = reachables.len();
    println!("{}", ans);
}