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
fn main() {
    // 2024-08-17 21:22-20:40 (1h18min)
    // 2024-08-19 19:42-20:53 (1h11min)
    // Total: 2h29min
    input! {
        n: usize,
        m: usize,
        mut a: [usize; n]
    }
    // solve_mine(n, m, a);
    solve_education(n, m, a);
}

fn solve_education(n: usize, m: usize, mut a: Vec<usize>) {
    // 2024-08-20 12:51-12:58 (7min)

    let mut dists = vec![0; 2*n];

    for i in 0..n {
        dists[i+1] = dists[i] + a[i];
    }
    for i in 0..n-1 {
        let ind = n + i;
        dists[ind+1] = dists[ind] + a[i];
    }

    let mut counts = vec![0; m];

    for i in 1..n {
        counts[dists[i] % m] += 1;
    }

    let mut ans: usize = 0;
    for i in 0..n {
        ans += counts[dists[i] % m];
        counts[dists[i+1] % m] -= 1;
        counts[dists[i+n] % m] += 1;
    }
    println!("{}", ans);


}

fn solve_mine(n: usize, m: usize, mut a: Vec<usize>) {
    let mut cum = a.clone();
    for i in 1..n {
        cum[i] += cum[i-1];
    }
    // println!("cum = {:?}", cum);
    let t = cum[n-1];

    // mの余りが、何個あるか?
    let mut ms: Vec<usize> = vec![0; m];
    for i in 0..n {
        ms[cum[i] % m] += 1;
    }
    let mut ans = 0;
    let mut ms2 = vec![0; m];
    for i in (0..n).rev() {
        let diff = (t - cum[i]) % m;
        ms[cum[i] % m] -= 1;

        let ind = (m - diff) % m;
        // println!("------ i = {:?} ----- ", i);
        // println!("diff = {:?}", diff);
        // println!("ind = {:?}", ind);
        // println!("ms = {:?}", ms);
        // println!("ms2 = {:?}", ms2);
        ans += ms[ind];
        ans += ms2[ind];
        // println!("ans = {:?}", ans);
        // ms2[(m + cum[i] - (t % m)) % m] += 1;
        ms2[(m - (t - cum[i]) % m) % m] += 1;
    }

    println!("{}", ans);

}