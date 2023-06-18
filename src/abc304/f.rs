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
        s: Chars,
    }
    let MODULUS = 998244353;


    let prime_factorized_list = prime_factorize(n);
    let mut yakusu_map = HashMap::new();
    let mut keys = vec![];
    let mut values = vec![];
    for i in 0..prime_factorized_list.len() {
        keys.push(prime_factorized_list[i][0]);
        values.push(prime_factorized_list[i][1]);
    }

    // 約数を列挙する (公式解答のやり方の方がスマートなので、そちらを参考にすると良い)
    saiki(0, &keys, &values, 1, &mut yakusu_map);
    // println!("{:?}", yakusu_map);
    let mut yakusu_list = vec![];
    for (k, v) in &yakusu_map {
        yakusu_list.push(*k);
    }








}

fn saiki(current_depth: usize, keys: &Vec<usize>, values: &Vec<usize>, yakusu: usize, yakusu_list: &mut HashMap<usize, usize>) {
    if current_depth == keys.len() {
        yakusu_list.insert(1, 1);
        return
    }

    for i in 0..(values[current_depth]+1) {
        let new_yakusu = yakusu * (keys[current_depth].pow(i as u32));
        yakusu_list.insert(new_yakusu, 1);
        saiki(current_depth+1, keys, values, new_yakusu, yakusu_list);
    }
}

// 素因数分解 (No.75の使いまわし)
fn prime_factorize(mut x: usize) -> Vec<Vec<usize>> {
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
            prime_num_list.push(vec![i, exponent]);
        }
    }
    if x != 1 {
        prime_num_list.push(vec![x, 1]);
    }
    return prime_num_list
}