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
    // 2024-05-14 21:20-21:31 (11min)
    input! {
        n: usize, // 宝箱 <= 12
        m: usize, // 鍵 <= 10^3個
    }
    
    let mut a = vec![];
    let mut masks = vec![];
    for i in 0..m {
        input!{
            ai: usize, // 鍵の値段
            bi: usize,
            ci: [usize; bi],
        }
        a.push(ai);
        let mut mask = 0;
        for j in 0..bi {
            mask |= 1 << (ci[j] - 1);
        }
        masks.push(mask);
    }

    // 全ての宝箱をあける最小値段

    let INF = 1<< 60;
    let mut dp = vec![INF; 1<<n];    
    dp[0] = 0;
    for mask in 0..1<<n {
        for i in 0..m {
            let key_mask = masks[i];
            dp[mask | key_mask] = min(dp[mask | key_mask], dp[mask] + a[i]);
        }
    }

    if dp[(1<<n)-1] == INF {
        println!("-1");
    }
    else {
        println!("{}", dp[(1<<n)-1]);
    }
    

}