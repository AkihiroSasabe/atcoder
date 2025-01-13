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
    input! {
        n: usize,
        mut a: [usize; n]
    }
    a.sort();

    // 決め打ち二ブタン?
    // k娘作れるか？
    // 小さいやつはなるべく上
    // 大きいやつはなるべく下にいってほしい
    // 貪欲?

    // めぐる式二分探索

    // 関数じゃなくて、クロージャーを使うと、引数を少なく出来る。
    let judge = |mid: usize| -> bool {
        for i in 0..mid {
            if a[i] * 2 > a[n-mid+i] {
                return false
            } 
        }
        return true
    };
    // fn judge(mid: usize) -> bool {
    //    return true
    // }

    let mut ng = n/2;
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