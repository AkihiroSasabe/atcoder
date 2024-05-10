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
    // 2024-05-09 20:41-21:18 (37min)
    input! {
        n: usize,
        k: isize,
        mut a: [isize; n], // 人iの消化コスト
        mut f: [isize;n], // 飯iの食べにくさ
    }

    // X * Y 秒
    // 最大値が、チーム全体の成績
    // N人で合計K回まで修行可能

    // 一番高いコストは、一番低い食べにくさとかけるのがよいかなと。

    a.sort(); // [2,8,9]
    f.sort(); // [5,3,1]
    f.reverse();

    // k: 5
    // f: 3 2 1
    // a: 1 2 4
    // 積: 3,4,4

    // a: 0 1 1 
    // 積: 0 2 1

    // 決め打ち二分探索...?
    // 最大がXのとき、
    // 

    // count := 成績の最小値が0のときの、修行回数
    let mut count = 0;
    for i in 0..n {
        if f[i] == 0 {continue}
        count += a[i];
    }
    if count <= k {
        println!("0");
        return
    }

    // めぐる式二分探索
    let mut ng = 0;
    let mut ok = a.iter().max().unwrap() * f.iter().max().unwrap();
    while (ng as i128 - ok as i128).abs() > 1 {
        let mid = (ng + ok) / 2;
        if judge(mid, k, &a, &f) {
            ok = mid;
        }
        else {
            ng = mid;
        }
    }
    println!("{}", ok);
}

fn judge(x: isize, mut k: isize, a: &Vec<isize>, f: &Vec<isize>) -> bool{
    // 実現可能か判定する
    // 成績x

    let n = a.len();

    // 成績:100
    // 食べにくさ；10
    // 人のコスト： 10

    // 食べにくさ: 11
    // 人のコスト: 9

    // fにクソでかい奴がいたら、どれか0にするべき。
    for i in 0..n {
        let a_tg = x / f[i];
        if a[i] <= a_tg {continue}
        let diff = a[i] - a_tg;
        k -= diff;
        if k < 0 {return false}
    }
    return true
}