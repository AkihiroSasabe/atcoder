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
        m: usize,
        ss1: Chars,
        ss2: Chars,
        ss3: Chars,
    }
    let mut s1 = vec![];
    for i in ss1 {
        s1.push(i as usize - '0' as usize);
    }
    let mut s2 = vec![];
    for i in ss2 {
        s2.push(i as usize - '0' as usize);
    }
    let mut s3 = vec![];
    for i in ss3 {
        s3.push(i as usize - '0' as usize);
    }
    let s_all = vec![&s1, &s2, &s3];
    let initial_ans = 1_000_000_000;
    let mut ans = initial_ans;
    // 3人の押す順番
    for perm in (0..3).permutations(3) {
        for num in 0..10 {
            let mut detected_flag = false;
            for i in 0..m {
                if s_all[perm[0]][i % m] != num {continue}
                for j in i+1..2*m {
                    if s_all[perm[1]][j % m] != num {continue}
                    for k in j+1..3*m {
                        if s_all[perm[2]][k % m] != num {continue}
                        else {
                            ans = min(ans, k);
                            detected_flag = true;
                            break
                        }
                    }
                    if detected_flag {break}
                }
                if detected_flag {break}
            }
        }
    }
    if ans == initial_ans {
        println!("-1");
    }
    else {
        println!("{}", ans);
    }
}