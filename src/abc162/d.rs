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
        n: usize, // 4000以下
        s: Chars,
    }

    let mut dp = vec![vec![0; 3]; n];
    let mut rev_dp: Vec<Vec<usize>> = vec![vec![0; 3]; n];

    for i in 0..n {
        if s[i] == 'R' {
            dp[i][0] = 1;
            rev_dp[i][0] = 1;

        } else if s[i] == 'G' {
            dp[i][1] = 1;
            rev_dp[i][1] = 1;
        } else if s[i] == 'B' {
            dp[i][2] = 1;
            rev_dp[i][2] = 1;
        }
    }
    for i in 0..n {
        if i > 0 {
            dp[i][0] += dp[i-1][0];
            dp[i][1] += dp[i-1][1];
            dp[i][2] += dp[i-1][2];

            rev_dp[n-i-1][0] += rev_dp[n-i][0];
            rev_dp[n-i-1][1] += rev_dp[n-i][1];
            rev_dp[n-i-1][2] += rev_dp[n-i][2];
        }
    }
    // println!("dp = {:?}", dp);
    // println!("rev_dp = {:?}", rev_dp);

    let mut ans = 0;
    for pos in 1..n-1 {
        let color = match s[pos] {
            'R' => 0,
            'G' => 1,
            'B' => 2,
            _ => unreachable!(),
        };
        let color1 = (color + 1) % 3;
        let color2 = (color + 2) % 3;
        let mut cont = 0;
        cont += dp[pos-1][color1] * rev_dp[pos+1][color2];
        cont += dp[pos-1][color2] * rev_dp[pos+1][color1];
        ans += cont;

        for pos2 in 0..pos {
            let diff = pos - pos2;
            let pos3 = pos + diff;
            if pos3 >= n {continue}
            if s[pos] != s[pos2] && s[pos2] != s[pos3] && s[pos] != s[pos3] {
                ans -= 1;
            }
        }
    }
    println!("{}", ans);



}