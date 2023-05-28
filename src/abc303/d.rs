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
        x: usize,
        y: usize,
        z: usize,
        s: Chars
    }
    let mut off_hash = HashMap::new();
    let mut on_hash = HashMap::new();
    off_hash.insert('a', x);
    off_hash.insert('A', y);
    on_hash.insert('a', y);
    on_hash.insert('A', x);

    let INF = 1 << 60;
    let mut dp = vec![vec![INF; 2]; s.len()];
    dp[0][0] = *off_hash.get(&s[0]).unwrap();
    dp[0][1] = z + *on_hash.get(&s[0]).unwrap();

    for i in 1..s.len() {
        // 元OFF -> 次もOFF
        let cost_off_to_off = dp[i-1][0] + *off_hash.get(&s[i]).unwrap();
        // 元ON -> 次はOFF
        let cost_on_to_off = dp[i-1][1] + z + *off_hash.get(&s[i]).unwrap();

        dp[i][0] = min(cost_off_to_off, cost_on_to_off);
        
        // 元OFF -> 次はON
        let cost_off_to_on = dp[i-1][0] + z + *on_hash.get(&s[i]).unwrap();
        // 元ON -> 次もON
        let cost_on_to_on = dp[i-1][1] + *on_hash.get(&s[i]).unwrap();
        dp[i][1] = min(cost_off_to_on, cost_on_to_on);
    }

    println!("{}", min(dp[s.len()-1][0], dp[s.len()-1][1]));
}