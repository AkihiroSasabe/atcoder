#![allow(dead_code, unused_imports)]
use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    input! {
        a: usize,
        b: usize,
    }
    let xx: usize = 1_000_000_000_000_000_001;
    let x = xx as f64;
    let mut t_0 = a as f64;                             // 時刻0で飛んだ時に要する時間
    let mut t_1 = a as f64 / 2.0_f64.sqrt() + b as f64; // 時刻1で飛んだ時に要する時間

    // t_n = a / (1+n).sqrt() + n * b は、時刻nで飛んだ時に要する時間。下に凸な関数
    // n=0が極小値の右側にあるとき (n>=0について単調増加なので、n=0が最小値となる)
    if t_0 <= t_1 {
        println!("{}", t_0);
    }
    // n=0が極小値の左側にあるとき
    else {
        // let n = (a as f64 / 2.0 / b as f64).powf(2.0/3.0) - 1.0;
        let n = (a as f64 / 2.0 / b as f64).powf(2.0/3.0) as isize - 1;
        // println!("n: {}", n);
        let mut ans = t_0;
        let offset: Vec<isize> = vec![-5,-4,-3,-2,-1,0,1,2,3,4,5];
        for i in 0..offset.len() {
            if offset[i] + n < 0 {continue}
            let n_around = (offset[i] + n) as usize;

            let t_1 = a as f64 / ((1+n_around) as f64).sqrt() + (b * n_around) as f64;
            // println!("n_around: {}, t_1: {}", n_around, t_1);
            if t_1 < ans {
                ans = t_1;
            }
            // ans = min(t_1, ans);
        }
        println!("{}", ans);
        
        // 2分探索
        // let mut low: usize = 0;
        // let mut high = 1_000_000_000_000_000_000;
        // // let mut high = 1 + a / b;
        // // let mut high = max(a, b);
        // while high - low > 1 {
        //     let mut mid = (low + high) / 2;
        //     let t_0 = a as f64 / ((1+mid) as f64).sqrt() + (b * mid) as f64;
        //     let t_1 = a as f64 / ((1+mid+1) as f64).sqrt() + (b * mid+1) as f64;
        //     // 最低長さがmid以上が実現可能だったとき
        //     if t_1 < t_0 {
        //         low = mid;
        //     }
        //     // 最低長さがmid以上が実現不可能だったとき
        //     else {
        //         high = mid;
        //     }
        // }
        // let t_1 = a as f64 / ((1+low+1) as f64).sqrt() + (b * low+1) as f64;
        // println!("{}", t_1);
    }
}