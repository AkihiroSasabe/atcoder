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
    input! {
        n: usize,
        c: isize,
        m: usize,
    }
    let mut t = vec![];
    let mut p = vec![];
    for i in 0..m {
        input!{
            ti: usize, // 街
            pi: isize, // 参加の報酬
        }
        t.push(ti-1);
        p.push(pi-1);
    }
    p.reverse();
    t.reverse();
    // 設けの最大
    // 所持金：10^(10^100)

    let INF = 1 << 60;
    let mut dp = vec![-INF; n];
    dp[t[0]] = max(dp[t[0]], t[i] as isize * (-c) + p[0]);

    for i in 0..m {
        dp[t[i]] = max(dp[t[i]], t[i] as isize * (-c) + p[i]);
        let ti = t[i];
        
    }

    let mut ans = 0;
    for i in 0..n {
        ans = max(ans, dp[i]);
    }



}   