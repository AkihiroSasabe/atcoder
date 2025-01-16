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
    // 2025-01-15 20:07-21:00 (53min)
    // 2025-01-16 00:29-00:37 (8min)
    // 61min
    input! {
        a: f64,
        b: f64,
        c: f64,
    }
    // f(t) = a*t + b*sin(pi*c*t)
    // f(t) - a*t = b*sin(pi*c*t)
    // 100 - a*t = b*sin(pi*c*t)

    // 2pi毎にみていけばよい。
    // 2pi = pi*c*dt
    let cycle = 2.0/c;
    let half_cycle = 1.0/c; // pi毎に見る
    // めぐる式二分探索

    let ff = |t: f64| -> f64 {
        return a * t + b * (PI * c * t).sin();
    };

    // 関数じゃなくて、クロージャーを使うと、引数を少なく出来る。
    let judge = |mid: usize| -> bool {
        // let t = cycle * mid as f64;
        let t = half_cycle * mid as f64;
        let ft = ff(t);
        // let ft = a*t + b * (PI * c * t).sin();
        // println!("ft = {:?}", ft);
        return ft > 100.0
    };
    // fn judge(mid: usize) -> bool {
    //    return true
    // }

    // At - b > 100
    // t > (100 + b) / A 
    // println!("cycle = {:?}", cycle);
    let mut ng = 0;
    let mut ok = ((100.0+b) / a / half_cycle) as usize + 2;
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

    // println!("ok = {:?}", ok);
    let mut t_ok = half_cycle * ok as f64;
    let mut t_ng = half_cycle * ng as f64;
    // println!("ff({}) = {:?}", t_ok, ff(t_ok));
    // println!("ff({}) = {:?}", t_ng, ff(t_ng));

    while (t_ok - t_ng).abs() > 0.000_000_000_001 {
        let mid = (t_ok + t_ng) / 2.0;
        let ft = ff(mid);
        if 100.0 < ft {
            t_ok = mid;
        } 
        else {
            t_ng = mid;
        }
    }
    println!("{}", t_ok);
    // println!("ff({}) = {:?}", t_ok, ff(t_ok));


}