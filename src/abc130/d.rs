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
    // 2024-05-04 18:50-
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }
    let mut cum = vec![0; n];
    cum[0] = a[0];
    for i in 1..n {
        cum[i] = cum[i-1] + a[i];
    }
    // println!("cum = {:?}", cum);

    // 2分探索
    let mut ans = 0;
    for left in 0..n {
        // めぐる式二分探索
        // 右

        if left == 0 {
            if cum[left] >= k {
                ans += n;
                continue
            }
            if cum[n-1] < k {
                continue
            }
        }
        else {
            if cum[left] - cum[left-1] >= k {
                ans += n - left;
                continue
            }
            if cum[n-1] - cum[left-1] < k {
                continue
            }
        }

        let mut ng = left;
        let mut ok = n-1;
        while (ng as i128 - ok as i128).abs() > 1 {
            let mid = (ng + ok) / 2;
            let sum = if left != 0 {
                cum[mid] - cum[left-1]
            } else {
                cum[mid]
            };
            if sum >= k {
                ok = mid;
            }
            else {
                ng = mid;
            }
        }
        // println!("left, right = {:?}, {}", left, ok);
        ans += n - ok;
    }
    println!("{}", ans);


    // let mut left = 0;
    // let mut right = 0;
    // loop {
    //     let range_sum = cum[right+1] - cum[left];
    //     if range_sum < 
    // }
}