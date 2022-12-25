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
        n: usize
    }

    // p * q^3 <= Nなので、q<N^(1/3)
    let q_cubic_root = (n as f64).powf(1.0/3.0); 
    // println!("n={}, q_cubic_root={}, {}", n, q_cubic_root, q_cubic_root as usize, );
    // println!("{:?}", sieve_of_eratosthenes(n));

    let prime_list = sieve_of_eratosthenes(q_cubic_root as usize);
    // println!("{:?}", prime_list);

    // 2分探索
    let mut ans = 0;
    for q_index in 1..prime_list.len() {
        let q = prime_list[q_index];
        let mut max_p = n / (q*q*q);
        max_p = min(max_p, q-1);
        let over_p_index = prime_list.upper_bound(&max_p);
        if over_p_index == 0 {continue}
        let max_p_index = over_p_index - 1;
        // println!("q:{} max_p:{}, over_p_index:{} max_p_index:{}", q, max_p, over_p_index, max_p_index);
        ans += max_p_index + 1;
    }
    println!("{}", ans);
}

fn is_ok(prime_list: &Vec<usize>, p: usize, q: usize, n: usize) -> bool {
    let k = prime_list[p] * prime_list[q] * prime_list[q] * prime_list[q];
    return k <= n
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