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
    // 2023-12-23 21:26-22:50 (74min) <- 解法は当日、ぎりぎり思いついたが、間に合わなかった。
    // 2024-01-02 16:23-16:54 (31min) <- 解き方をわかっていて、単純に実装に時間が掛かっている。
    // 105min 
    input! {
        n: usize,
        k: usize,
        mut a: [usize; k]
    }
    for i in 0..k {
        a[i] -= 1;
    }
    let mut ans = 0;

    if k % 2 == 0 {
        for i in 0..k/2 {
            ans += (a[2*i+1] - a[2*i]);
        }
    }
    else {
        // 奇数
        if k == 1 {
            println!("0");
            return;
        }
        else {
            // foraward, baskward cumlation
            let mut f_cum = vec![0; k];
            let mut b_cum = vec![0; k];
            for i in 1..k {
                if i % 2 == 1 {
                    if i < k {
                        f_cum[i] = f_cum[i-1] + a[i] - a[i-1];
                    }
                }
                else {
                    f_cum[i] = f_cum[i-1];
                }
            }
            for i in (0..k-1).rev() {
                if i % 2 == 1 {
                    b_cum[i] = b_cum[i+1] + a[i+1] - a[i];
                }
                else {
                    b_cum[i] = b_cum[i+1];
                }
            }
            // println!("f_cum = {:?}", f_cum);
            // println!("b_cum = {:?}", b_cum);

            // 誰をハブるか?
            ans = min(f_cum[k-1], b_cum[0]);
            for i in 1..k-1 {
                let temp;
                if i % 2 == 0 {
                    temp = f_cum[i] + b_cum[i];
                }
                else {
                    temp = f_cum[i-1] + b_cum[i+1] + a[i+1] - a[i-1];
                }
                // println!("i = {i}, temp = {:?}", temp);
                ans = min(ans, temp);
            }
        }
    }

    println!("{}", ans);

}