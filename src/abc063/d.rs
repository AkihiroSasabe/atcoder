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
    // 2024-09-28 17:47-18:49 (1h2min。解説見始めた。よく考えたら、N回for分を回して良かった。決め打ち二分探索には、気付いていた)
    // 2024-09-30 19:32-19:36 (4min)
    // Total 1h6min
    input! {
        n: usize,
        a: isize,
        b: isize,
        mut h: [isize; n],
    }
    h.sort();

    // 決め打ち2分探索: X 回
    // 全員に X * B 引く。
    // (A-B) で割った余り...
    // (h[i] - X * B ) / (A-B)
    // (h[i] - X * B ) % (A-B)

    // 18:05 思いつき
    let mut max_num = h[0] / b + 1;
    for i in 1..n {
        max_num += h[i] / b + 1;
    }

    // めぐる式二分探索
    fn judge(mid: isize, a: isize, b: isize, h: &Vec<isize>) -> bool {
        let n = h.len();

        let mut num_bomb = 0;
        for i in 0..n {
            if h[i] > b * mid {
                num_bomb += (h[i] - (b * mid)) / (a-b);
                if (h[i] - (b * mid)) % (a-b) != 0 {
                    num_bomb += 1;
                }
            }
        }
        
        return num_bomb <= mid
    }
    let mut ng = 0;
    let mut ok = max_num;
    while (ng as i128 - ok as i128).abs() > 1 {
        let mid = (ng + ok) / 2;
        let is_ok = judge(mid, a, b, &h);
        if is_ok {
            ok = mid;
        }
        else {
            ng = mid;
        }
    }
    println!("{}", ok);


}