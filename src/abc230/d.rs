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
    // 2023-01-06 11:46- 12:26 (40min)
    input! {
        n: usize,
        d: usize,
    }
    // let mut l = vec![];
    // let mut r = vec![];
    let mut rl = vec![];
    for i in 0..n {
        input! {
            l_i: usize,
            r_i: usize,
        }
        rl.push(vec![r_i - 1, l_i - 1]);
        // l.push(l_i);
        // r.push(r_i);
    }

    // 貪欲法で解いてみる
    rl.sort();
    let mut index = 0;
    let mut ans = 1;
    let mut flag = false;
    loop {
        let punch_start = rl[index][0];
        let punch_end = punch_start + d - 1;
        if index == n - 1 {break} 
        while break_judge(&rl, punch_start, punch_end, index+1) {
            index += 1;
            if index == n - 1 {
                flag = true;
                break
            }
        }
        if flag {break}
        index += 1;
        ans += 1;
    }
    println!("{}", ans);
}

fn break_judge(rl: &Vec<Vec<usize>>, start: usize, end: usize, mut index: usize) -> bool {
    let r = rl[index][0];
    let l = rl[index][1];
    let mut flag = false;
    if start <= r && r <= end {
        flag = true;
    }
    else if start <= l && l <= end {
        flag = true;
    }
    else if r <= start && end <= l {
        flag = true;
    }
    else if start <= r && l <= end {
        flag = true;
    }
    return flag
}