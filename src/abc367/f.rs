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
    // 2024-08-20 20:44-21:00 (16min)
    // 2024-08-21 19:20-19:58 (38min)
    // 54 min
    
    // hash
    // (S == T) => H(S) == H(T): 真
    // H(S) == H(T) => (S == T): 偽 (ただし、高確率で成立する)

    // (S != T) => H(S) != H(T): 偽
    // H(S) != H(T) => (S != T): 真


    input! {
        n: usize,
        q: usize,
        a: [usize; n],
        b: [usize; n],
    }
    let mut la = vec![];
    let mut ra = vec![];
    let mut lb = vec![];
    let mut rb = vec![];
    for i in 0..q {
        input!{ 
            lai: usize,
            rai: usize,
            lbi: usize,
            rbi: usize,
        }
        la.push(lai-1);
        ra.push(rai-1);

        lb.push(lbi-1);
        rb.push(rbi-1);
    }

    let mut rnd = rand::thread_rng();
    let mut hash: Vec<usize> = vec![];
    let modulus = 998_244_353; // 素数じゃなくてもいいし、もっと大きい数字の方がハッシュの衝突確率が減る
    for i in 0..n+1 {
        // let x: usize = rand::random();
        let x: usize = rnd.gen_range(1..modulus);
        hash.push(x);
    }
    let mut cum_a = vec![];
    let mut cum_b = vec![];
    for i in 0..n {
        cum_a.push(hash[a[i]] % modulus);
        cum_b.push(hash[b[i]] % modulus);
        // cum_a.push(a[i] % modulus); // a[i] <= N <= 2*10^5 だと、衝突しやすくなってしまうので、乱数を使って適当に拡大
        // cum_b.push(b[i] % modulus);
    }
    for i in 1..n {
        cum_a[i] = (cum_a[i-1] + cum_a[i]) % modulus;
        cum_b[i] = (cum_b[i-1] + cum_b[i]) % modulus;
    }

    for i in 0..q {
        // 長さが異なれば即終了
        if ra[i] - la[i] != rb[i] - lb[i] {
            println!("No");
            continue
        }

        let hash_a = if la[i] > 0 {
            (modulus + cum_a[ra[i]] - cum_a[la[i]-1]) % modulus
        } else {
            cum_a[ra[i]]
        };

        let hash_b = if lb[i] > 0 {
            (modulus + cum_b[rb[i]] - cum_b[lb[i]-1]) % modulus
        } else {
            cum_b[rb[i]]
        };
        if hash_a == hash_b {
            // 一致する可能性が高い
            println!("Yes");
        }
        else {
            // 不一致確定
            println!("No");
        }
    }
}