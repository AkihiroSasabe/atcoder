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
    // 2025-03-06 19:32-19:53 (21min)
    input! {
        a: usize,
        b: usize,
        q: usize,
        mut s: [isize; a],
        mut t: [isize; b],
        x: [isize; q],
    }
    let inf = 100_000_000_000_000;
    s.push(inf);
    t.push(inf);

    for i in 0..q {
        let mut ans: isize = inf;
        let xi = x[i];
        let ind0 = s.lower_bound(&xi);
        let ind1 = t.lower_bound(&xi);

        if ind0 == 0 && ind1 == 0 {
            let s1 = s[ind0];
            let t1 = t[ind1];

            // s1-t1
            ans = min(ans, max(s1, t1) - xi);
        }
        else if ind0 == 0 {
            let s1 = s[ind0];
            let t0 = t[ind1-1];
            let t1 = t[ind1];

            // s1-t1
            ans = min(ans, max(s1, t1) - xi);
            // s1-t0
            ans = min(ans, s1-t0 + min(s1-xi, xi-t0));      
        }
        else if ind1 == 0 {
            let s0 = s[ind0-1];
            let s1 = s[ind0];
            let t1 = t[ind1];
            // s1-t1
            ans = min(ans, max(s1, t1) - xi);
            // s0-t1
            ans = min(ans, t1-s0 + min(t1-xi, xi-s0));
        }
        else {
            // 4パターンある。
            let s0 = s[ind0-1];
            let s1 = s[ind0];
            let t0 = t[ind1-1];
            let t1 = t[ind1];

            // s1-t1
            ans = min(ans, max(s1, t1) - xi);
            // s0-t0
            ans = min(ans, (min(s0, t0) - xi).abs());
            // s0-t1
            ans = min(ans, t1-s0 + min(t1-xi, xi-s0));
            // s1-t0
            ans = min(ans, s1-t0 + min(s1-xi, xi-t0));      
        }
        println!("{}", ans);
    }



}