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
    // 2024-06-21 20:02-20:48 (46min)
    input! {
        n: usize
    }
    // 七五数とは約数をちょうど 75 個持つ正の整数
    // N: 100
    // N!

    // let p_list = prime_factorize(n);

    // n以下の素数を抜き出し
    let prime_list = sieve_of_eratosthenes(n);
    // println!("prime_list = {:?}", prime_list);
    // println!("prime_list = {:?}", prime_list.len()); // 25個
    // prime_list = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97]
    // 二個選ぶ 
    // 素因数分解を全部にやる。

    // 23_400 = 2^4 * 3^4 * 5^2
    // (4+1) * (4+1) * (2+1) = 5 * 5 * 3 = 75

    // 75 = 3^1 * 5^2
    // 素因数分解した結果、指数が、(2,4,4) or (2,24) になっていればok
    // a^2 * b^4 * c^4
    // a^2 * b^24
    // let x = 32400;
    // let x = 75; // 3^1 * 5^2
    // let p_list = prime_factorize(x);
    // println!("p_list = {:?}", p_list);
    
    // map := N! の素因数分解の結果 <素数、指数>
    let mut map: BTreeMap<usize, usize> = BTreeMap::new();
    for x in 2..n+1 {
        let p_list = prime_factorize(x);
        for (p,e) in p_list {
            *map.entry(p).or_insert(0) += e;
        }
    }
    // println!("map = {:?}", map);

    // (2,4,4)のパターンを考える。
    let mut ans: usize = 0;
    for comb in prime_list.iter().combinations(3) {
        let p0 = comb[0];
        let p1 = comb[1];
        let p2 = comb[2];
        // println!("comb = {:?}", comb);
        // println!("{} {} {}", *map.get(p0).unwrap(), *map.get(p1).unwrap(), *map.get(p2).unwrap());

        if *map.get(p0).unwrap() >= 2 && *map.get(p1).unwrap() >= 4 && *map.get(p2).unwrap() >= 4 {
            ans += 1;
        }
        if *map.get(p0).unwrap() >= 4 && *map.get(p1).unwrap() >= 2 && *map.get(p2).unwrap() >= 4 {
            ans += 1;
        }
        if *map.get(p0).unwrap() >= 4 && *map.get(p1).unwrap() >= 4 && *map.get(p2).unwrap() >= 2 {
            ans += 1;
        }
    }

    // (2,24)のパターンと、(4,14)のパターン
    for comb in prime_list.iter().combinations(2) {
        let p0 = comb[0];
        let p1 = comb[1];
        // println!("comb = {:?}", comb);
        // println!("{} {} {}", *map.get(p0).unwrap(), *map.get(p1).unwrap(), *map.get(p2).unwrap());

        if *map.get(p0).unwrap() >= 2 && *map.get(p1).unwrap() >= 24  {
            ans += 1;
        }
        if *map.get(p0).unwrap() >= 24 && *map.get(p1).unwrap() >= 2  {
            ans += 1;
        }
        if *map.get(p0).unwrap() >= 4 && *map.get(p1).unwrap() >= 14  {
            ans += 1;
        }
        if *map.get(p0).unwrap() >= 14 && *map.get(p1).unwrap() >= 4  {
            ans += 1;
        }
    }

    // (74)のパターン
    for (p, e) in map {
        if e >= 74 {
            ans += 1;
        }
    }

    println!("{}", ans);
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