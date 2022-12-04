#![allow(dead_code, unused_imports)]
use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    input! {
        k: usize,
    }

    let prime_num_list = prime_factorize(k.clone());
    // println!("{:?}", prime_num_list);

    let mut ans = 2;
    for i in 0..prime_num_list.len() {
        let mut kesu = 0;
        let mut kakeru = 1;
        while kakeru <= prime_num_list[i][1]{
            kesu += 1;
            kakeru += 1;
            let mut kesu2 = kesu.clone();
            while kesu2 % prime_num_list[i][0] == 0 {
                kesu2 /= prime_num_list[i][0];
                kakeru += 1;
            }
        }
        // let cand = prime_num_list[i][0] * prime_num_list[i][1];
        let cand = prime_num_list[i][0] * kesu;
        ans = max(ans, cand);
    }
    println!("{}", ans);


    // let mut ans = 0;
    // let mut kk = k;
    // println!("kk:{}", kk);

    // let mut kaijo: usize = 1;
    // for i in 2..(k+1) {
    //     kaijo *= i;
    //     if kaijo % kk == 0 {
    //         println!("i: {}, kk: {}, kaijo: {}", i, kk, kaijo);
    //         ans = i;
    //         break
    //     }
    // }
    // println!("{}", ans);

}


// 素因数分解
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