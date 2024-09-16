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
    // 2024-09-11 20:21-20:39 (18min) 解放思いつき
    // 2024-09-11 21:05-21:21 (16min) floatにしないといけないことに気がつく
    // 2024-09-16 12:03-12:32 (29min)
    // 63min
    input! {
        n: usize,
        t: [usize; n],
        v: [usize; n],
    }

    let v_max = *v.iter().max().unwrap();
    let t_max: usize = t.iter().sum();
    let t_max2 = 2 *t_max;
    let v_max2 = 2 *v_max;

    // 往路の観点で、到達可能か?
    // v_forward[ti][vi] := 時刻 ti / 2 で、vi / 2 が可能
    let mut v_forward = vec![vec![false; v_max2+5]; t_max2+5];
    v_forward[0][0] = true;
    let mut now = 0;
    for i in 0..n {
        for dt in 0..2*t[i] {
            now += 1;
            for vi in 0..v_max2+1 {
                if vi > 2 * v[i] {continue}
                v_forward[now][vi] |=  v_forward[now-1][vi];
                v_forward[now][vi] |= v_forward[now-1][vi+1];
                if vi == 0 {continue}
                v_forward[now][vi] |= v_forward[now-1][vi-1];
            }
        }
    }

    // 復路の観点で到達可能か?
    // v_backward[ti][vi] := 時刻 ti / 2 で、vi / 2 が可能
    let mut v_backward: Vec<Vec<bool>> = vec![vec![false; v_max2+5]; t_max2+5];
    v_backward[t_max2][0] = true;
    let mut now = t_max2;
    for i in (0..n).rev() {
        for dt in 0..2*t[i] {
            now -= 1;
            for vi in 0..v_max2+1 {
                if vi > 2 * v[i] {continue}
                v_backward[now][vi] |= v_backward[now+1][vi];
                v_backward[now][vi] |= v_backward[now+1][vi+1];
                if vi == 0 {continue}
                v_backward[now][vi] |= v_backward[now+1][vi-1];
            }
        }
    }

    let mut v_both = vec![vec![false; v_max2+5]; t_max2+5];
    let mut v_maxes: Vec<f64> = vec![0.0; t_max2+5];
    for ti in 0..t_max2+1 {
        for vi in 0..v_max2+1 {
            if v_forward[ti][vi] && v_backward[ti][vi] {
                // v_maxes[ti] = max(v_maxes[ti], vi as f64 / 2.0);
                v_maxes[ti] = v_maxes[ti].max(vi as f64 / 2.0);
            }
        }
    }
    // println!("v_maxes = {:?}", v_maxes);
    let mut ans = 0.0;
    for ti in 1..t_max2+1 {
        // 速度平均
        let v_pre = v_maxes[ti-1];
        let v_now = v_maxes[ti];
        let t_width = 0.5;
        let diff = (v_pre + v_now) * t_width / 2.0;
        ans += diff;
    }
    println!("{}", ans);
}