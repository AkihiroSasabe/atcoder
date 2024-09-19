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
    // 2024-09-17 19:49-20:14 (25min)
    // 2024-09-17 20:14-20:49 (35min)
    // 60 min
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
        e: usize, // 100g の水に解ける砂糖
        f: usize, // 水 + 砂糖 の合計の最大
    }

    // 砂糖水の質量と、解けている砂糖の量

    // dp[水][砂糖] := 可能か?
    // let MAX_SIZE = 8_000;
    let MAX_SIZE = 6_001;
    // let MAX_SIZE = 401;
    let mut dp: Vec<Vec<bool>> = vec![vec![false; MAX_SIZE]; MAX_SIZE];
    dp[0][0] = true;

    for water in 0..f+1 {
        for sugar in 0..f+1 {
            dp[water + 100*a][sugar] |= dp[water][sugar];
            dp[water + 100*b][sugar] |= dp[water][sugar];
            dp[water][sugar + c] |= dp[water][sugar];
            dp[water][sugar + d] |= dp[water][sugar];
        }
    }
    // for wat in 0..f+1 {
    //     println!("dp[{wat}] = {:?}", dp[wat]);
    // }


    // sg / (sg + wt) < sg2 / (sg2 + wt2)


    let mut ans_mix = 100*a;
    let mut ans_sugar = 0;
    // let mut max_concentration = 0.0;
    for water in 0..f+1 {
        for sugar in 0..f+1 {
            if !dp[water][sugar] {continue}
            if water + sugar > f {continue}
            if 100 * sugar > water * e {continue}

            // let concentration = sugar as f64 / (water + sugar) as f64;
            // println!("water = {}, sugar = {}, concentration = {}", water, sugar, concentration);
            // if max_concentration < concentration {
            if ans_sugar * (water + sugar) < sugar * ans_mix  {
                // max_concentration = concentration;
                ans_mix = water + sugar;
                ans_sugar = sugar;
            }
        }
    }

    println!("{} {}", ans_mix, ans_sugar);

}