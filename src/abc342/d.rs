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
        mut a: [usize; n]
    }
    let mut hash = HashMap::new();
    let mut ans = 0;
    for i in (0..n).rev() {
        if a[i] == 0 {
            ans += n - 1 - i;
            *hash.entry(0).or_insert(0) += 1;
        }
        else {
            let aaa: Vec<Vec<usize>> = prime_factorize(a[i]);
            let mut temp = 1;
            for x in aaa {
                let base = x[0];
                let exp = x[1];
                if exp % 2 == 1 {
                    temp *= base;
                }
            }
            if hash.contains_key(&temp) {
                let diff = hash[&temp];
                ans += diff;
            }
            if hash.contains_key(&0) {
                let diff = hash[&0];
                ans += diff;
            }
            *hash.entry(temp).or_insert(0) += 1;
        }
    }
    println!("{}", ans);

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