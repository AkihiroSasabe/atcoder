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
    input! {
        n: usize,
    }
    let mut k  = vec![];
    let mut a = vec![];
    let mut aa: Vec<BTreeMap<usize, usize>> = vec![BTreeMap::new(); n]; // aa[i] := サイコロiの{値: 個数}
    for i in 0..n {
        input! {
            ki: usize,
            ai: [usize; ki],
        }
        k.push(ki);
        a.push(ai.clone());
        for j in 0..ki {
            *aa[i].entry(ai[j]).or_insert(0) += 1;
        }
    }

    // 2<=N<=100
    // nC2=4950
    // 5*10^3 * 10^5



    let mut ans = 0.0;
    for comb in (0..n).combinations(2) {
        let ind0 = comb[0];
        let ind1 = comb[1];
        let mut p = 0.0;
        let mut seen: BTreeSet<usize> = BTreeSet::new();

        for i in 0..a[ind0].len() {
            if seen.contains(&a[ind0][i]) {continue}

            if let Some(&num) = aa[ind1].get(&a[ind0][i]) {
                p += (num as f64 / a[ind1].len() as f64) * (*aa[ind0].get(&a[ind0][i]).unwrap() as f64 / a[ind0].len() as f64);
                seen.insert(a[ind0][i]);
                // 
            }
        }
        // println!("ind0 = {}, ind1 = {:?}", ind0, ind1);
        // println!("p = {:?}", p);
        if ans < p {
            ans = p;
        }
    }
    println!("{}", ans);



}