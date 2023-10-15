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
use std::collections::HashSet;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    input! {
        s: Chars
    }
    let MODULUS = 998244353;
    let mut dp: Vec<Vec<usize>> = vec![vec![0; 3100]; 3100];
    if s[0]== '(' {
        dp[0][1] += 1;
    }
    else if s[0] == '?' {
        dp[0][1] += 1;
    }
    // println!("dp[0][0] = {}", dp[0][0]);
    // println!("dp[0][1] = {}", dp[0][1]);
    // println!("====");

    for i in 1..s.len() {
        for over_left in 0..s.len()+1 {
            if s[i] == '(' {
                // println!("i={i}, s[{i}]=(, over_left={over_left}");
                dp[i][over_left+1] += dp[i-1][over_left];
                dp[i][over_left+1] %= MODULUS;
            }
            else if s[i] == '?' {
                // println!("i={i}, s[{i}]=?, over_left={over_left}");
                dp[i][over_left+1] += dp[i-1][over_left];
                dp[i][over_left+1] %= MODULUS;
                if over_left != 0 {
                    dp[i][over_left-1] += dp[i-1][over_left];
                    dp[i][over_left-1] %= MODULUS;
                }
            }
            else {
                // println!("i={i}, s[{i}]=), over_left={over_left}");
                if over_left != 0 {
                    dp[i][over_left-1] += dp[i-1][over_left];
                    dp[i][over_left-1] %= MODULUS;
                }
            }
            // println!("dp[{i}][{over_left}] = {}", dp[i][over_left]);
        }
    }
    println!("{}", dp[s.len()-1][0]);





}