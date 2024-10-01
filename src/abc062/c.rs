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
use rand::Rng;
fn main() {
    // 2024-09-30 19:51-20:14 (23min)
    input! {
        h: usize,
        w: usize,
    }
    let mut ans = h * w;

    // (1)縦に0本,横に2本 -> O(1)
    // (2)縦に1本 -> O(W)
    // (3)横に1本 -> O(H)
    // (4)縦に2本,横に0本 -> O(1)


    // (1)縦に0本,横に2本 -> O(1)
    let smin = (h / 3) * w;
    let mut smax = (h / 3) * w;
    if h % 3 != 0 {
        smax += w;
    }
    ans = min(ans, smax - smin);

    // (2)縦に1本 -> O(W)
    for i in 1..h {
        // 下側で切る
        let mut s0 = i * w;
        let mut s1 = (h-i) * (w / 2);
        let mut s2 = h * w - s0 - s1;
        if 2 * i > h {
            // 上側で切る
            s0 = (h-i) * w;
            s1 = i * (w / 2);
            s2 = h * w - s0 - s1;
        }
        let mut smin = min(s0, s1);
        smin = min(smin, s2);

        let mut smax = max(s0, s1);
        smax = max(smax, s2);
        ans = min(ans, smax - smin);
    }
    
    // (3)横に1本 -> O(H)
    for i in 1..w {
        // 右側で切る
        let mut s0 = i * h;
        let mut s1 = (w-i) * (h / 2);
        let mut s2 = h * w - s0 - s1;
        if 2 * i > w {
            // 左側で切る
            s0 = (w-i) * h;
            s1 = i * (h / 2);
            s2 = h * w - s0 - s1;
        }
        let mut smin = min(s0, s1);
        smin = min(smin, s2);

        let mut smax = max(s0, s1);
        smax = max(smax, s2);
        ans = min(ans, smax - smin);
    }

    // (4)縦に2本,横に0本 -> O(1)
    let smin = (w / 3) * h;
    let mut smax = (w / 3) * h;
    if w % 3 != 0 {
        smax += h;
    }
    ans = min(ans, smax - smin);
    println!("{}", ans);

}