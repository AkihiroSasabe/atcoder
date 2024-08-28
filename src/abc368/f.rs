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
    // 2024-08-24 22:16-22:40 (24min)
    // 2024-08-26
    // 実質的にNim (石取りゲーム) と同じ問題。a[i]を素因数分解したときの、各指数の合計が、山iのコイン数に等しい。  
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut states = vec![];
    let mut max_num = 1;
    for i in 0..n {
        let p_list: Vec<(usize, usize)> = prime_factorize(a[i]);
        // println!("p_list = {:?}", p_list);
        let mut num = 0;
        for (pi, ei) in p_list {
            num += ei;
        }
        // println!("num = {:?}", num);
        states.push(num);
        max_num = max(max_num, num);
    }
    // 山iに
    let mut grandy = vec![0; max_num+1];
    for i in 0..max_num+1 {
        grandy[i] = i;
    }

    let mut xor_sum: usize = 0;
    for i in 0..n {
        xor_sum ^= grandy[states[i]];
    }
    if xor_sum != 0 {
        println!("Anna");
    }
    else {
        println!("Bruno");
    }

}

// 素因数分解
fn prime_factorize(mut x: usize) -> Vec<(usize, usize)> {
    // prime_num_list[i] := (素数p_i, 指数exp_i) が、格納されたリスト
    // 
    // 例: x = 48 = 2^4 * 3^1 のとき、
    // let prime_num_list: Vec<(usize, usize)> = prime_factorize(48);
    // prime_num_list = [(2, 4), (3, 1)]

    // let root_x = (x as f64).sqrt() as usize;
    let mut prime_num_list = vec![];
    let mut i = 1;
    while i * i <= x {
    // for i in 2..(root_x+1) {
        i += 1;
        let mut exponent = 0;
        while x % i == 0 {
            x /= i;
            exponent += 1;
        }
        if exponent != 0 {
            prime_num_list.push((i, exponent));
        }
    }
    if x != 1 {
        prime_num_list.push((x, 1));
    }
    return prime_num_list
}
