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
    // 2024-04-06 12:00-12:25 (25min) 
    // 2024-04-08 21:47-21:59 (12min)
    // 37min
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }
    // 決めうち二分探索
    // 最小の長さ

    if judge(1, k, &a) {
        // 1が可能ならそれが答え
        println!("1");
        return
    }
    let mut ng = 1; // 1はK回の切断で実現不可能な長さ(小さい過ぎる。)
    let mut ok = 2_000_000_000; // K回の切断で実現できる長さ(これの最小を求めたい。初期値は十分に大きくしとく)
    while (ng as i128 - ok as i128).abs() > 1 {
        let mid = (ng + ok) / 2;
        if judge(mid, k, &a) {
            ok = mid;
        }
        else {
            ng = mid;
        }
    }
    println!("{}", ok);
    

    // 各丸太について、
    // 切る回数が決まっていれば、
    // 等分して切るのが良い
    // 例
    // 7 => 1回 => 3.5ずつ切る
    // 9 => 2回 => 3ずつで

    // 全員の最大公約数とか
    // 全部の丸太の長さの和を取って、それを等分することを目指せばよいのでは?

    // log2(10^9) = 29.897 -> 30回切ったら終わり -> 終わらない。毎回半分にするわけじゃない

}

fn judge(min_length: usize, k: usize, a: &Vec<usize>) -> bool {
    let mut num_cut= 0;
    for i in 0..a.len() {
        let d = a[i] / min_length;
        let r = a[i] % min_length;
        num_cut += d;
        if r == 0 {
            num_cut -= 1;
        }
    }
    return num_cut <= k
}