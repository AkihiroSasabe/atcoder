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
        n: usize,
        m: usize,
        l: [usize; n]
    }

    let mut cum = vec![0; n];
    cum[0] = l[0];
    for i in 1..n {
        cum[i] = l[i] + cum[i-1] + 1;
    }
    let max_li = l.iter().max().unwrap();

    // 条件: m行に収まる or 収まらない. 横幅Wの最小値が知りたい
    // めぐる式二分探索
    let mut ng = 0;
    let mut ok = 1_000_000_000_000_000_000; // 10^18 (横幅の最大値は、max(L_i) * max(N) = 10^9 * 2*10^5 = 2*10^14なので、これ以上なら何でもい)
    while (ng as i128 - ok as i128).abs() > 1 {
        let mid = (ng + ok) / 2;
        if judge(mid, &cum, *max_li, n, m) {
            ok = mid;
        }
        else {
            ng = mid;
        }
    }
    println!("{}", ok);

}

fn judge(w_max: usize, cum: &Vec<usize>, max_li: usize, n: usize, m: usize) -> bool {
    // m行に収まるか判定する関数
    // w_max: 1行の長さ上限
    // cum: 文字数の累積和
    // n: 全単語数
    // m: 最大行数

    // println!("---- w_max={w_max} ----");
    if max_li > w_max {
        return false
    }

    let mut num_row = 0;
    let mut length_to_check = w_max;
    loop {
        let lb: usize = cum.lower_bound(&(length_to_check + 1));
        // println!("~~ lb = {} ~~", lb);
        num_row += 1;
        if lb == n {break}
        length_to_check = cum[lb-1] + 1 + w_max;
        // println!("length_to_check = {}", length_to_check);
    }
    // println!("num_row = {}", num_row);
    return num_row <= m
}