#![allow(dead_code, unused_imports)]
use proconio::input;
use itertools::Itertools;
use core::num;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::BTreeMap;
use std::hash::Hash;
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::collections::HashSet;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    // 2023-10-12 21:38-22:20 (42min)
    // 2023-10-13 12:07-12:45 (38min)
    // 2023-10-13 20:30-21:00 (30min)
    // 110min = 1h50min
    input! {
        t: usize,
    }
    let mut n = vec![];
    let mut d = vec![];
    let mut k = vec![];
    for i in 0..t {
        input! {
            ni: usize,
            di: usize,
            ki: usize
        }
        n.push(ni);
        d.push(di);
        k.push(ki);
    }

    for i in 0..t {
        let ni = n[i];
        let di = d[i];
        let ki = k[i];

        let diff_angle = di % ni;
        let gcd: usize = gcd(ni, diff_angle);
        let cycle_time = ni / gcd;

        let amari = (ki - 1) % cycle_time;
        let num_loop = (ki - 1) / cycle_time;
        
        let ans = (num_loop + amari * diff_angle) % ni;
        // println!("i={i}, ni={ni}, di={di}, ki={ki}-----------");
        // println!("diff_angle={diff_angle}, cycle_time = {cycle_time}, amari = {amari}, num_loop = {num_loop}");
        println!("{}", ans);

    }

    // TLE解
    // for i in 0..t {
    //     let ni = n[i];
    //     let di = d[i];
    //     let ki = k[i];

    //     if di == 1 {
    //         println!("{}", ki - 1);
    //         continue;
    //     }

    //     // 合計di回のループで終わる。
    //     // 1回のループで何回カウントするか?
    //     // z回目の始点はどこか?

    //     let mut hash = HashSet::new();
    //     let mut x = 0;
    //     let mut count = 1;
    //     loop {
    //         while hash.contains(&x) {
    //             x += 1;
    //         }
    //         hash.insert(x);
    //         // 一番右
    //         let diff_count = (ni - 1 - x) / di;
    //         x = x + diff_count * di;
    //         count = count + diff_count;
    //         // println!("i ={}, x = {}, count = {}", i, x, count);
    //         if count >= ki {
    //             x -= (count - ki) * di;
    //             println!("{}", x);
    //             break
    //         }
    //         else {
    //             x = (x + di) % ni;
    //             count += 1;
    //         }
    //     }
    // }
}


// ユークリッドの互除法で最大公約数を求める (Euclidean Algorithm)
// ユークリッドの互除法とは、x < y のとき、gcd(x, y)=gcd(x, y % x)
fn gcd(x: usize, y:usize) -> usize {
    if y == 0 {
        // 任意の整数xは0の約数と言える(∵0 % x == 0)ので、0とxの最大公約数はx
        return x;
    }
    else {
        return gcd(y, x % y);
    }
}