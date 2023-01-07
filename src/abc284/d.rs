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
        t: usize,
        test: [usize; t]
    }
    for i in 0..t {
        let mut n_i = test[i];
        let mut p1 = 1;
        let mut p = 1;
        let mut q = 1;
        while p1*p1*p1 <= n_i {
            p1 += 1;
            if n_i % p1 == 0 {
                if n_i % (p1*p1) == 0 {
                    p = p1;
                    q = n_i / p / p;
                    println!("{} {}", p, q);
                    break
                }
                else {
                    q = p1;
                    p = ((n_i / q) as f64).sqrt() as usize;
                    println!("{} {}", p, q);
                    break
                }
            }
        }
    }

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
