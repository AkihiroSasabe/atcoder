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
    // 2024-05-08 19:19-19:38 (19min)
    input! {
        n: usize,
        t: usize,
    }
    let mut a = vec![];
    let mut b = vec![];
    let mut ab = vec![];
    for i in 0..n {
        input!{
            ai: usize, // 時間
            bi: usize, // 美味しさ
        }
        a.push(ai);
        b.push(bi);
        ab.push([ai, bi]);
    }
    // 時間が短い順に食べると良い
    // 時間が長い奴は、なるべく後。
    ab.sort();

    // 1度に1つの料理のみ を注文

    // 食べ終わった時刻をTとする
    // dp := {key: 食べ終わる時刻, value: その時刻における幸せの最大値}
    let mut dp = BTreeMap::new();
    dp.insert(0, 0);
    for i in 0..n {
        let mut temps = vec![];
        for (&time, &happy) in dp.iter() {
            // ラストオーダーT以降は、新たに食べ始めることは出来ないので、スキップ
            if time >= t {continue}
            let next_time = time + ab[i][0];
            let next_happy = happy + ab[i][1];
            temps.push((next_time, next_happy));
        }
        for (nt, nh) in temps {
            if dp.contains_key(&nt) {
                let mut ph = dp.get_mut(&nt).unwrap();
                *ph = max(*ph, nh);
            }
            else {
                dp.insert(nt, nh);
            }
        }
    }
    let mut ans = 0;
    for (ti, hi) in dp {
        ans = max(ans, hi);
    } 
    println!("{}", ans);


}