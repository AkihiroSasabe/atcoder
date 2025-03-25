#![allow(dead_code, unused_imports)]
use proconio::{input, marker::{Usize1, Isize1}};
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
use num::{BigUint, ToPrimitive};
use num_bigint::ToBigUint;
fn main() {
    input! {
        a: usize, // 以上
        b: usize, // 以下
        wk: usize,
    }
    let w = wk * 1000;

    // UNSATISFIABLE 
    // ちょうどWキロぐらむ
    // 個数の最小、最大を求める問題。
    let mut max_num = 0;
    let max_a_num = w / a;
    let r = w % a;
    if r > (b-a) * max_a_num {
        println!("UNSATISFIABLE");
        return;
    }
    else {
        max_num = max_a_num;
    }

    // 最小数を求める
    let mut min_num = 1 << 60;
    let mut is_ok = false;
    for num in 0..max_num+1 {
        let min_w = num * a;
        let max_w = num * b;
        if min_w <= w && w <= max_w {
            is_ok = true;
            min_num = num;
            break
        }
    }
    if !is_ok {
        println!("UNSATISFIABLE");
        return;
    }
    println!("{} {}", min_num, max_num);



}