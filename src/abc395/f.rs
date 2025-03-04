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
    // 2025-03-02 19:04-21:00 (1h56min)
    // 2025-03-04 20:02-20:16 (12min)
    // 2h8min
    input! {
        n: usize,
        x: isize,
    }
    let mut u = vec![];
    let mut d = vec![];

    let mut min_sum = (1_usize << 62) as isize;
    let mut sums = vec![];
    for i in 0..n {
        input!{
            ui: isize,
            di: isize,
        }
        u.push(ui);
        d.push(di);
        let sum = ui+di;
        min_sum = min(min_sum, sum);

        sums.push(sum);
    }

    // めぐる式二分探索

    // 関数じゃなくて、クロージャーを使うと、引数を少なく出来る。
    let judge = |mid: isize| -> bool {

        // mid: 上の歯と下の歯の和

        // dp[i][0] := i番目までで、実現可能な最小の上歯長
        // dp[i][1] := i番目までで、実現可能な最大の上歯長
        let mut dp = vec![[0; 2]; n];
        // println!("mid = {:?}", mid);
        // println!("sums[0] = {:?}", sums[0]);
        let kezuru_i = sums[0] - mid;
        let kezuru_max = min(kezuru_i, u[0]);
        let kezuru_min = max(kezuru_i - d[0], 0);
        dp[0][0] = u[0] - kezuru_max;
        dp[0][1] = u[0] - kezuru_min;
        
        for i in 1..n {
            // 条件1 の要請 Ui​+Di​=H
            let kezuru_i = sums[i] - mid;
            let kezuru_max = min(kezuru_i, u[i]);
            let kezuru_min = max(kezuru_i - d[i], 0);
            let min_u1 = u[i] - kezuru_max;
            let max_u1 = u[i] - kezuru_min;

            // 条件2 の要請 ∣Ui​−Ui+1​∣≤ X
            let min_u2 = dp[i-1][0] - x;
            let max_u2 = dp[i-1][1] + x;

            // 閉区間[min_u1,max_u1] と, 閉区間[min_u2,max_u2]のandの閉区間が無いとき、解無しでmidは実現不可
            let min_u = max(min_u1, min_u2);
            let max_u = min(max_u1, max_u2);
            if min_u > max_u {return false}

            dp[i][0] = min_u;
            dp[i][1] = max_u;
        }
        return true
    };

    // 実現可能なHの最大を調べる。
    let mut ng = min_sum;
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

    let mut ans = 0;
    for i in 0..n {
        ans = ans + sums[i] - ok;
    }

    println!("{}", ans);
}