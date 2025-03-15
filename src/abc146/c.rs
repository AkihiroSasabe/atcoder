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
use num::{BigUint, ToPrimitive};
use num_bigint::ToBigUint;
fn main() {
    // 2025-03-06 ?-20:43
    // 2025-03-07 12:31-
    input! {
        a: usize,
        b: usize,
        x: usize,
    }

    solve(a,b,x);
    // solve_wa(a,b,x);

}
fn solve(a: usize, b: usize, x: usize) {
    // めぐる式二分探索
    // 関数じゃなくて、クロージャーを使うと、引数を少なく出来る。
    let judge = |mid: usize| -> bool {
        let cost = a * mid + b * get_keta(mid);
        return cost <= x
    };
    // fn judge(mid: usize) -> bool {
    //    return true
    // }

    let mut ng = 1_000_000_000;
    let mut ok = 0;
    if judge(ng) {
        ok = ng;
    }
    while (ng as i128 - ok as i128).abs() > 1 {
        let mid = (ng + ok) / 2;
        let is_ok = judge(mid);
        if is_ok {
            ok = mid;
        }
        else {
            ng = mid;
        }
    }
    println!("{}", ok);
}

fn solve_wa(a: usize, b: usize, x: usize) {
    let mut pow = vec![];

    let mut temp = 1;

    for i in 0..15 {
        pow.push(temp-1);
        temp = temp * 10;
    }

    let mut ans = 0;
    for d in 1..11 {
        println!("pow[{d}] = {:?}", pow[d]);
        if x < b * d {continue}
        // 残りの金
        let r = x - b * d;
        let mut n = r / a;
        println!("n = {:?}", n);
        if n > pow[d] {
            n = pow[d];
        }
        println!("(d,n, get_keta(n)) = {:?}", (d,n, get_keta(n)));
        if get_keta(n)  == d {
            ans = max(ans, n);
        }
    }
    // 9_999_999_999 10桁
    println!("{}", ans);
    // 1_000_000_000 10桁
    // 999999999
    // 1000000000
    // 100000000000
    // 49999999994
    // 49_999_999_994

}

fn get_keta(mut x: usize) -> usize {
    let mut keta = 1;
    while x / 10 != 0 {
        x /= 10;
        keta += 1;
    }
    return keta
}