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
    // 2025-05-10 15:52-16:57 (1h5min)
    // 2025-05-13 20:34-20:58 (24min)
    // 2025-05-13 20:58-21:21 (23min, debug)
    // Total: 1h52min
    input! {
        n: usize,
        m: usize,
        h: [isize; n],
        mut w: [isize; m],
    }
    // solve_brute_force(n,m,h,w);
    solve(n,m,h,w);

    // エネルギー図を描くと、 hをソートし、h[x] を抜いた後は、
    // 隣り合うペアの差が最小になる。
    // つまり、 Σ h[i+1] - h[i] (i=0,2,..., n-2) が最小。

    // 例
    // h0= [1,2,2,2,2,2,10,10]
    // h1= [1, 300, 300, 400, 500, 1000]
    // 真ん中: 1199
    // 隣同士：299 + 100 + 500 = 899
    // Σ(a[i+1] - a[i])
    // (a[n-1] - a[n-2]) + (a[n-3] - a[n-4]) + ... + (a[1] - a[0])

}

// 愚直解法 (TLE)
fn solve_brute_force(n: usize, m: usize, mut h: Vec<isize>, mut w: Vec<isize>) {
    h.sort();
    // i := 仲間はずれ
    for i in 0..n {
        let mut cand = 0;
        let mut h2 = h.clone();
        
        let mut sensei_cont = 1_000_000_000_000_000_000;
        for j in 0..m {
            sensei_cont = min(sensei_cont, (h[i] - w[j]).abs());
        }
        h2.remove(i);
        let mut students_cont = 0;
        for j in 0..n-1 {
            if j % 2 == 0 {
                students_cont += (h2[j] - h2[j+1]).abs();
            }
        }
        cand = sensei_cont + students_cont;
        println!("(i, cand) = {:?}", (i, cand));
    }
}


fn solve(n: usize, m: usize, mut h: Vec<isize>, mut w: Vec<isize>) {

    h.sort();
    w.sort();
    // println!("w = {:?}", w);

    // 仲間外れをN人の中から1人選ぶ

    // 0 を仲間はずれにするとき
    // (0), (1~2),(3~4),(5~6) ... , (n-2~n-1)
    // x(偶数) を仲間外れにしたとき
    
    let mut cum0 = vec![0; n];
    let mut cum1 = vec![0; n];
    
    for i in 1..n {
        if i % 2 == 0 {
            let diff = (h[i] - h[i-1]).abs();
            cum0[i-1] = diff;
        }
    }
    // println!("cum0 = {:?}", cum0);
    for i in 1..n {
        cum0[i] += cum0[i-1];
    }
    // println!("cum0 = {:?}", cum0);


    // n-1を仲間はずれにするとき
    // (0-1), (2-3), (4-5), ..., (n-3~n-2), (n-1)
    // x(奇数) を仲間外れにしたとき
    for i in 0..n-1 {
        if i % 2 == 0 {
            let diff = (h[i] - h[i+1]).abs();
            cum1[i] = diff;
        }
    }
    // println!("cum1 = {:?}", cum1);
    for i in 1..n {
        cum1[i] += cum1[i-1];
    }
    // println!("cum1 = {:?}", cum1);

    let diffs= sensei(&h, &w);
    // println!("diffs = {:?}", diffs);
    let ans_first = cum0[n-1] + diffs[0];
    let ans_last = cum1[n-1] + diffs[n-1];
    // println!("ans_first = {:?}", ans_first);
    // println!("ans_last = {:?}", ans_last);
    let mut ans = min(ans_first, ans_last);
    for i in 1..n-1 {
        // i := 仲間はずれ
        let mut cand = 1_000_000_000_000_000_000;
        if i % 2 == 1 {
            let mut cont1 = 0;
            if i >= 2 {
                cont1 += cum1[i-2];
            }
            cont1 += cum0[n-1] - cum0[i];
            cont1 += (h[i-1] - h[i+1]).abs();
            let cont2 = diffs[i];
            cand = min(cand, cont1 + cont2);
        }
        else {
            // i := 偶数が仲間はずれ
            let cont1 = cum1[i-1] + cum0[n-1] - cum0[i];
            let cont2 = diffs[i];
            cand = min(cand, cont1 + cont2);
        }
        // println!("i = {}, cand = {:?}", i, cand);
        ans = min(ans, cand);
    }
    println!("{}", ans);
}

fn sensei(h: &Vec<isize>, w: &Vec<isize>) -> Vec<isize> {
    // diffs[x] := h[x] を仲間はずれにしたとき、先生との最小誤差

    let n = h.len();
    let m = w.len();
    let mut diffs = vec![0; n];
    for x in 0..n {
        let ind = w.lower_bound(&h[x]);
        let mut diff = 1_000_000_000_000_000_000;

        if ind != m {
            let diff0 = (w[ind] - h[x]).abs();
            diff = min(diff, diff0);
        }

        if ind != 0 {
            let diff1 = (w[ind-1] - h[x]).abs();
            diff = min(diff, diff1);
        }
        diffs[x] = diff;
    }
    return diffs
}