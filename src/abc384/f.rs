#![allow(dead_code, unused_imports)]
use proconio::{input, marker::{Usize1, Isize1}};
use itertools::Itertools;
use std::{cmp::{max, min}, io::IntoInnerError};
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
    // 2024-12-14 22:02-22:40 (32min)
    // 2024-12-15 10:58-11:47 first submit (49min)
    // 2024-12-15 11:47-12:10 debug (23min)
    // 2024-12-15 17:05-18:11 (1h6min) セグ木を疑う
    // 2024-12-15 18:11-19:40 (1h49min) 解説見た
    // 2024-12-16 19:01-19:39 (38min)
    // Total 317 min
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut ans = 0;

    // 自分自身との和 Σ[i=0,..,n-1] (Ai + Ai)
    let mut inner = 0;
    for i in 0..n {
        let mut temp = a[i] + a[i];
        while temp % 2 == 0 {
            temp /= 2;
        }
        inner += temp;
    }

    // let max_2a =  2 * (*a.iter().max().unwrap());

    // sums[i].0 := a[i] と適当な相方の和が 2^k (k>=0) の倍数になるときの相方の和
    // sums[i].1 := a[i] と適当な相方の和が 2^k (k>=0) の倍数になるときの相方の個数
    let mut sums = vec![(0,0); n];
    // 例えば、 f(5+43)=f(48)=f(3*(2^4))=3 について考える。
    // r        5 % r   43 % r
    // 2-5=32   5       11      // 5 + 11 = 16 <- 32で割れない
    // 2^4=16   5       11      // 5 + 11 = 16 <- 16で割れる
    // 2^3=8    5       3       // 5 + 3 = 8   <- 8でも割れるけど、(5+43)は16で割りたいので、ここでは除外したい。
    // 2^2=4    1       3       // 1 + 3 = 4   <- 4でも割れるけど、(5+43)は16で割りたいので、ここでは除外したい。
    // 2^1=2    1       1       // 1 + 1 = 2   <- 2でも割れるけど、(5+43)は16で割りたいので、ここでは除外したい。

    // let max_bit = 64;
    // let max_bit = 32;
    let max_bit = 25; // math.log2(20_000_000) = 24.253496664211536
    for b in (0..max_bit).rev() {
        // println!("2^{} -------------", b);

        // btree := {key: r := 2^bで割った余り, [2^bで割った余りがrなai, ...]}
        let mut btree = HashMap::new();
        // btree2 := {key: a[i]剰余, val:剰余が同じa[i]の和} が格納されている。
        let mut btree2 = HashMap::new();
        let modulus = (1 << b);
        for i in 0..n {
            let r = a[i] % modulus;
            btree.entry(r).or_insert(vec![]).push(a[i]);
            *btree2.entry(r).or_insert(0) += a[i];
        }
        // println!("btree = {:?}", btree);
        for i in 0..n {
            let r = a[i] % modulus;
            let r2 = (modulus - r) % modulus;
            if let Some(sum) =btree2.get(&r2) {
                let sum2 = sum - sums[i].0;
                let len2 = btree.get(&r2).unwrap().len() - sums[i].1;
                let temp = (sum2 + a[i] * len2) / modulus;
                // println!("i={i}, r={}, r2 = {:?}", r, r2);
                // println!("temp = {:?}", temp);
                ans += temp;
                sums[i].0 += sum2; // 和
                sums[i].1 += len2 // 個数
            }
        }
    }
    println!("{}", (ans - inner) / 2 + inner);
}