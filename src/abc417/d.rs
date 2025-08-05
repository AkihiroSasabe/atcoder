#![allow(dead_code, unused_imports)]
use proconio::{input, marker::{Usize1, Isize1, Chars}};
use itertools::Itertools;
use std::{cmp::{max, min, Ordering, Reverse}, vec};
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::collections::{VecDeque, BinaryHeap, HashMap, BTreeMap, HashSet, BTreeSet};
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
use rand::Rng;
use num::{BigUint, ToPrimitive, Integer};
use num_bigint::ToBigUint;
fn main() {
    // 2025-08-02 22:13-22:40 (27min)
    // 2025-08-03 9:38-10:49 (1h11min)
    // 2025-08-03 10:49-11:16 (27min) 落ち着いて考える。
    // 2025-08-03 16:13-17:34 (1h21min)
    // Total: 3h27min
    input! {
        n: usize
    }
    let mut p = vec![];
    let mut a = vec![];
    let mut b = vec![];
    let mut a_sum = 0;
    for i in 0..n {
        input!{
            pi: usize,// 価値
            ai: usize, // 上げ度
            bi: usize, // 下げ度
        }
        p.push(pi);
        a.push(ai);
        b.push(bi);
        a_sum += ai;
    }
    input! {
        q: usize,
        x: [usize; q],
    }
    // 考察
    
    
    // let p_max = *p.iter().max().unwrap();
    // let a_max = *a.iter().max().unwrap();

    let p_max = 500;
    let a_max = 500;

    let mut seen = vec![vec![false; p_max + a_max + 1]; n];
    // dp2[i][xi] := テンションxiだったやつが、i番目のプレゼントを貰ったあとどこに移動するか?
    let mut dp2: Vec<Vec<usize>> = vec![vec![0; p_max + a_max + 1]; n];
    
    fn dfs(tension: usize, depth: usize, seen: &mut Vec<Vec<bool>>, dp2: &mut Vec<Vec<usize>>, p: &Vec<usize>, a: &Vec<usize>, b: &Vec<usize>) -> usize {
        let n = p.len();
        if depth == n {
            return tension;
        }
        if seen[depth][tension] {
            return dp2[depth][tension];
        }

        let next_tension = if tension <= p[depth] {
            tension + a[depth]
        } else {
            tension.saturating_sub(b[depth])
        };

        dp2[depth][tension] = dfs(next_tension, depth + 1, seen, dp2, p, a, b);
        seen[depth][tension] = true;
        return dp2[depth][tension];
    }

    let mut b_cum = b.clone();
    for i in 0..n-1 {
        b_cum[i+1] = b_cum[i] + b_cum[i+1];
    }

    // println!("(p_max, a_max, p_max+a_max) = {:?}", (p_max, a_max, p_max+a_max));

    for i in 0..q {
        let xi = x[i];
        if xi <= p_max + a_max {
            let ans = dfs(xi, 0, &mut seen, &mut dp2, &p, &a, &b);
            println!("{}", ans);
        }
        else {
            // テンションが、初めて p_max + a_max 以下になるのはいつか?
            let ind = b_cum.lower_bound(&(xi - p_max - a_max));
            // println!("ind = {:?}", ind);
            if ind == n {
                let ans = xi - b_cum[n-1];
                println!("{}", ans);
            }
            else {
                let res = xi- b_cum[ind];
                let ans = dfs(res, ind+1, &mut seen, &mut dp2, &p, &a, &b);
                println!("{}", ans);
            }
        }
    }
}
