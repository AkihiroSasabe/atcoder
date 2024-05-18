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
    // 2024-05-18 13:20-13:39 (19min)
    input! {
        mut s: Chars
    }
    // 13で割って5余る数が何通りあるか?
    let modulus = 1_000_000_007;


    s.reverse();
    let n = s.len();
    let mut hatenas = vec![];
    let mut r_sum = 0;
    let mut power = 1;
    for i in 0..n{
        if s[i] == '?' {
            hatenas.push(power);
        }
        else {
            r_sum += (s[i] as usize - '0' as usize) * power;
            r_sum %= 13;
        }
        power *= 10;
        power %= 13;
    }
    let mut dp = vec![0; 13];
    dp[r_sum] = 1;

    for i in 0..hatenas.len() {
        let pre_dp = dp.clone();
        dp = vec![0; 13];
        for r in 0..13 {
            for x in 0..10 {
                // x := はてなの値
                let nr = (r + hatenas[i] * x) % 13;
                dp[nr] += pre_dp[r];
                dp[nr] %= modulus;
            }
        }
    }
    println!("{}", dp[5]);

}