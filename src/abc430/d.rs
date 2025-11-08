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
    input! {
        n: usize,
        mut x: [usize; n],
    }
    // x.sort();
    // println!("x = {:?}", x);

    let mut set = BTreeSet::new();
    set.insert(0_usize);
    let mut dists = BTreeMap::new();
    let inf = 1_000_000_000_000_000_000;
    dists.insert(0_usize, inf);
    let mut ans = inf;    

    for i in 0..n {
        let xi = x[i];

        let left = *set.range(..=xi).rev().next().unwrap();
        let dist_l = xi - left;
        if let Some(&pre_dist) = dists.get(&left) {
            if pre_dist > dist_l {
                ans = ans - pre_dist + dist_l;
                dists.insert(left, dist_l);
            }
        }
        if let Some(&right) = set.range(xi..).next() {
            let dist_r = right - xi;
            let dist = min(dist_l, dist_r);
            dists.insert(xi, dist);
            ans += dist;

            if let Some(&pre_dist) = dists.get(&right) {
                if pre_dist > dist_r {
                    ans = ans - pre_dist + dist_r;
                    dists.insert(right, dist_r);
                }
            }
        }
        else {
            let dist = dist_l;
            dists.insert(xi, dist);
            ans += dist;
        }
        set.insert(xi);
        // println!("set = {:?}", set);
        // println!("dists = {:?}", dists);
        println!("{}", ans);
    }

}