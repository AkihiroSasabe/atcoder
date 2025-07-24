#![allow(dead_code, unused_imports)]
use proconio::{input, marker::{Usize1, Isize1, Chars}};
use itertools::Itertools;
use std::cmp::{max, min, Ordering, Reverse};
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::collections::{VecDeque, BinaryHeap, HashMap, BTreeMap, HashSet, BTreeSet};
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
use rand::Rng;
use num::{BigUint, ToPrimitive, Integer};
use num_bigint::ToBigUint;
fn main() {
    // 2025-07-19 21:50-22:40 (50min)
    // 2025-07-20 11:56-12:30 (34min)
    // Total: 84min
    input! {
        h: usize,
        w: usize,
        a: [[isize; w]; h],
        p: [isize; h+w-1],
    }
    // めぐる式二分探索
    // 関数じゃなくて、クロージャーを使うと、引数を少なく出来る。
    let judge = |mid: isize| -> bool {
        return judge_goal(&a, &p, mid as isize);
    };
    // fn judge(mid: usize) -> bool {
    //    return true
    // }
    
    let mut ng = 0;
    let mut ok = p.iter().sum();
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

fn judge_goal(a: &Vec<Vec<isize>>, p: &Vec<isize>, life: isize) -> bool {

    let h = a.len();
    let w = a[0].len();
    let mut dp = vec![vec![0; w+1]; h+1];
    dp[0][0] = life;
    let mut is_left = vec![vec![false; w+1]; h+1];
    let mut is_seen = vec![vec![false; w+1]; h+1];
    is_seen[0][0] = true;

    for y in 0..h {
        for x in 0..w {
            let time = y + x;
            if !is_seen[y][x] {continue}
            if dp[y][x] + a[y][x] < p[time] {continue}
            is_left[y][x] = true;
            dp[y][x+1] = max(dp[y][x+1], dp[y][x] + a[y][x]- p[time]);
            dp[y+1][x] = max(dp[y+1][x], dp[y][x] + a[y][x] - p[time]);
            is_seen[y][x+1] = true;
            is_seen[y+1][x] = true;
        }
    }
    let is_ok = is_left[h-1][w-1];

    // println!("life = {:?} =====================", life);
    // println!("is_seen ----");
    // for y in 0..h {
    //     for x in 0..w {
    //         print!("{} ", is_seen[y][x]);
    //     }
    //     println!("");
    // }
    // println!("is_left ----");    
    // for y in 0..h {
    //     for x in 0..w {
    //         print!("{} ", is_left[y][x]);
    //     }
    //     println!("");
    // }
    // println!("dp ----");
    // for y in 0..h {
    //     for x in 0..w {
    //         print!("{} ", dp[y][x]);
    //     }
    //     println!("");
    // }
    // println!("is_ok = {}", is_ok);


    return is_ok
}