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
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    }
    let ps = sieve_of_eratosthenes(b+d);
    let mut set = HashSet::new();
    for p in ps {
        set.insert(p as usize);
    }

    for x in a..=b {
        let mut is_takahashi_win = true;
        
        for y in c..=d {
            if set.contains(&(x+y)) {
                is_takahashi_win = false;
            }
        }
        if is_takahashi_win {
            println!("Takahashi");
            return;
        }
    }
    println!("Aoki");
}


// エラトステネスの篩(ふるい)
// n以下の素数を全て列挙する為のアルゴリズムO(n*log(log(n)))
fn sieve_of_eratosthenes(n: usize) -> Vec<usize> {
    // is_prime_list[i] := iが素数なら true , そうでなければ false
    let mut is_prime_list = vec![true; n + 1]; // この初期化でO(N)かかる!
    is_prime_list[0] = false; // 0は素数ではない
    is_prime_list[1] = false; // 1は素数ではない

    // prime_list := n以下の素数を格納したリスト
    let mut prime_list = vec![];
    
    // ここの計算で、O(N/2 + N/3 + N/5 + N/7 + N/11 + ...)  = O(N (1/2 + 1/3 + 1/5 + 1/7 + 1/11 + ... )) = O(Nlog(logN))かかる。
    // ※素数の逆数和は、log(logN)と漸近していくため。自然数の逆数和は、logNに漸近する。
    for i in 2..(n+1) {
        if !is_prime_list[i] {continue}
        for j in 2..((n/i)+1) {
            // i の倍数が素数ではないことを記録
            is_prime_list[j * i] = false;
        }
        prime_list.push(i);
    }
    return prime_list;
}