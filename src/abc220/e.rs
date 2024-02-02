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
    // 2024-02-02 20:01-21:29 (1h28min)
    // 2024-02-02 21:29-22:12 (43min, debug, testcaseを見て、D=1でバグがあることを発見)
    // total: 2h11m = 131m
    input! {
        n: usize,
        d: usize,
    }
    let modulus = 998244353;

    let max_exp = 1000_002;
    let mut pow2: Vec<usize> = vec![0; max_exp];
    pow2[0] = 1;
    for i in 1..max_exp {
        pow2[i] = pow2[i-1] * 2 % modulus;
    }

    // 2 頂点間の距離を、その 2 頂点を結ぶ単純パスに含まれる辺の個数

    let min_depth = d / 2 + d % 2;
    let mut dist_c0 = d / 2;
    let mut dist_c1 = d / 2 + d % 2;


    let mut ans = 0;
    let mut cum = 0;
    let bottom = n-1;
    let mut is_updated = true;
    for i in min_depth..n {
        let now_level = bottom - i;
        let num_now_level = pow2[now_level];
        // println!("---- now_level = {:?}", now_level);
        // println!("num_now_level = {:?}", num_now_level);
        
        let num_c0 = if dist_c0 != 0 {
            (pow2[dist_c0-1] + modulus) % modulus
        }
        else {
            1
        };
        let num_c1 = (pow2[dist_c1-1] + modulus) % modulus;
        let diff = num_c0 * num_c1 % modulus;
        // println!("dist_c0, dist_c1 = {:?}, {:?}", dist_c0, dist_c1);
        // println!("num_c0, num_c1, diff = {:?}, {:?}, {:?}", num_c0, num_c1, diff);

        if is_updated {
            if dist_c0 != dist_c1 {
                cum += 2 * diff % modulus;
            }
            else {
                cum += diff;
            }
        }

        if dist_c0 != 0 {
            dist_c1 += 1;
            dist_c0 -= 1;
        }
        else {
            is_updated = false;
        }
        
        cum %= modulus;
        // println!("cum = {:?}", cum);

        ans += num_now_level * cum;
        ans %= modulus;
        // println!("ans = {:?}", ans);
    }
    println!("{}", ans * 2 % modulus);

}