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
        t: usize,
    }
    let mut n = vec![];
    let mut s = vec![];
    for i in 0..t {
        input!{
            ni: usize,
            si: Chars
        }
        n.push(ni);
        s.push(si);
    }

    let inf: usize = 1 << 60;

    for i in 0..t {
        let ni = n[i];
        let si = &s[i];
        let mut dp = vec![vec![inf; 3]; n[i]];
        if si[0] == '1' {
            dp[0][0] = 1;
            dp[0][1] = 0;
        } else {
            dp[0][0] = 0;
            dp[0][1] = 1;
        }

        for j in 1..ni {
            if si[j] == '0' {
                dp[j][0] = dp[j-1][0];
                dp[j][1] = min(dp[j-1][0] + 1, dp[j-1][1] + 1);
                dp[j][2] = min(dp[j-1][1], dp[j-1][2]);
            }
            else {
                dp[j][0] = dp[j-1][0] + 1;
                dp[j][1] = min(dp[j-1][0], dp[j-1][1]);
                dp[j][2] = min(dp[j-1][1] + 1, dp[j-1][2] + 1);
            }
        }
        let mut ans = min(dp[ni-1][0], dp[ni-1][1]);
        let mut ans = min(ans, dp[ni-1][2]);
        println!("{}", ans);
    }
}