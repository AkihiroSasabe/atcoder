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
        l: usize,
        r: usize,
    }
    // 長さ2^N+1で、いい整数
    // 貪欲なDPで良さそう

    let mut pre_l = l;
    let mut ans = vec![];

    loop {
        let count = 60;
        for i in (0..count+1).rev() {
            let j = pre_l / (1 << i);
            // println!("j = {:?}", j);
            let amari = pre_l % (1 << i);
            if amari != 0 {
                continue
            }
            let nl = (1 << i) * j;
            let nr = (1 << i) * (j+1);
            if nr > r {
                continue
            }
            else {
                ans.push(vec![nl, nr]);
                break
            }
        }
        // println!("ans = {:?}", ans);
        pre_l = ans[ans.len()-1][1];
        if ans[ans.len()-1][1] == r {
            break
        }
    }
    println!("{}", ans.len());
    for i in 0..ans.len() {
        println!("{} {}", ans[i][0], ans[i][1]);
    }
}