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
        n: usize,
        mut s: Chars
    }
    let mut head_pos = vec![];
    for i in 0..n {
        if s[i] == '-' {
            head_pos.push(i as isize);
        }
    }
    let mut cand = vec![];

    if head_pos.len() > 0 {
        if head_pos[0] != 0 {
            cand.push(head_pos[0]);
        }

        let last_index = head_pos.len()-1;
        if head_pos[last_index] != (s.len() - 1) as isize {
            cand.push(s.len() as isize - 1 - head_pos[last_index]);
        }

        let mut pre_head = head_pos[0];
        for i in 1..head_pos.len() {
            cand.push(head_pos[i] - pre_head - 1);
            pre_head = head_pos[i];
        }
    }
    if cand.len() == 0 {
        println!("-1");
    }
    else {
        let mut ans = -1;
        for i in 0..cand.len() {
            ans = max(ans, cand[i]);
        }
        if ans == 0 {
            println!("-1");
        }
        else {
            println!("{}", ans);
        }
        // println!("{}", cand.iter().max().unwrap());
    }

}