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
    // 2025-01-25 (Sat.) 21:15-22:39 (1h24min)
    input! {
        n: usize, // <= 5000
        x: usize, // <= 5000 karori-　max
    } 
    let mut vac = vec![vec![]; 3];
    let mut sum_a = 0; // N個全部食べたときの、接種量の総和
    for i in 0..n {
        input!{
            vsi: Usize1, // しゅるい
            asi: usize, // りょう
            csi: usize, // かろり
        }
        vac[vsi].push((asi, csi));
        sum_a += asi;
    }

    // 全体的に大きくしたい。平均を大きくしたい?
    // きめうち二分探索?

    // 3つdpを用意?
    // dp[vi][カロリーcc] := タイプviの、接種カロリーがccのとき、接種量の最大量
    let mut dp = vec![vec![0; x+1]; 3];
    for vi in 0..3 {
        for i in 0..vac[vi].len() {
            let pre_dp = dp[vi].clone();
            let (ai, ci) = vac[vi][i];
            for cc in 0..x+1 {
                if cc + ci > x {continue}
                dp[vi][cc + ci] = max(dp[vi][cc + ci], pre_dp[cc] + ai);
            }
        }
        // println!("dp[vi] = {:?}", dp[vi]);
    }

    // めぐる式二分探索
    // 関数じゃなくて、クロージャーを使うと、引数を少なく出来る。
    let judge = |mid: usize| -> bool {
        // 3つとも、mid以上の接種量がいる

        // cal := 必要なカロリー
        let mut cal = 0;
        for vi in 0..3 {
            let ind = dp[vi].lower_bound(&mid);
            cal += ind;
        }

        return cal <= x
    };

    // let mut ng = 100_000_000_000;
    let mut ng = sum_a;
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