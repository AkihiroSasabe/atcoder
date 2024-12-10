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
fn main() {
    input! {
        n: usize
    }
    if n == 1 {
        println!("0");
        return;
    }

    let ps = sieve_of_eratosthenes((10.0 + n  as f64).sqrt() as usize + 10);
    // println!("ps = {:?}", ps);
    let mut cand = 2;
    let mut set = BTreeSet::new();

    for p in ps.clone() {
        let mut temp = 1;
        let mut is_ok = true;
        for _ in 0..8 {
            temp *= p;
            if temp > n {
                is_ok = false;
                break
            }
        }
        if is_ok {
            if temp <= n {
                set.insert(temp);
            }
        }
    }

    for i in 0..ps.len() {
        if ps[i] * ps[i] > n {break}
        for j in i+1..ps.len() {
            let mut temp = ps[i] * ps[i] * ps[j];
            if temp >= n {break}

            if temp * ps[j] <= n {
                set.insert(ps[i] * ps[i] * ps[j] * ps[j]);
            } 
            else {
                break
            }
        }
    }

    // println!("set = {:?}", set);
    println!("{}", set.len());


    // while cand * cand * cand *cand *cand * cand * cand * cand <= n {
    //     ans += 1;
    //     let list = prime_factorize(cand);
    //     if list.len() > 1 {continue}
    //     if list[0] > 1 {continue}
    //     set.insert(cand * cand * cand *cand *cand * cand * cand * cand);
    //     cand += 1;
    // } 

    // let mut cand = 2;
    // let mut cands = vec![];
    // while cand * cand <= n {
    //     cands.push(cand);
    // }


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

// エラトステネスの篩(ふるい)
// n以下の素数を全て列挙する為のアルゴリズムO(n*log(log(n)))
fn sieve_of_eratosthenes(n: usize) -> Vec<usize> {
    // prime_judge[i] := iが素数なら1, そうでなければ0
    let mut prime_judge = vec![1; n + 1];
    prime_judge[0] = 0;
    prime_judge[1] = 0;

    // prime_list := n以下の素数を格納したリスト
    let mut prime_list = vec![];
    for i in 2..(n+1) {
        if prime_judge[i] == 0 {continue}
        for j in 2..((n/i)+1) {
            prime_judge[j * i] = 0;
        }
        prime_list.push(i);
    }
    return prime_list;
}