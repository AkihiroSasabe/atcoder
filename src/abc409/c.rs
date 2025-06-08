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
    // 2025-06-07 21:03-21:14 (11min)
    // 2025-06-07 21:14-21:41 (27min, debug)
    // Total: 38min
    input! {
        n: usize,
        l: usize,
        d: [usize; n-1],
    }
    if l % 3 != 0 {
        println!("0");
        return;
    }

    let cycle = l / 3;
    let mut poss = vec![0; n];
    poss[0] = 0;
    for i in 0..n-1 {
        poss[i+1] = poss[i] + d[i] % l; 
        poss[i+1] %= l;
    }

    let mut nums = vec![0; 3*l + 10];
    for i in 0..n {
        nums[poss[i]] += 1;
    }

    let mut ans: u128 = 0;
    for p0 in 0..cycle {
        let p1 = (p0 + cycle);
        let p2 = (p0 + cycle * 2);
        ans += nums[p0] * nums[p1] * nums[p2];
        // println!("(p0, p1, p2) = {:?}", (p0, p1, p2));
    }
    println!("{}", ans);


    // let mut nums = vec![0_usize.to_biguint().unwrap(); l];
    // for i in 0..n {
    //     nums[poss[i]] += 1_usize.to_biguint().unwrap();;
    // }

    // // let mut ans: BigUint = 0;
    // let mut ans: BigUint = 0_usize.to_biguint().unwrap();
    // for p0 in 0..cycle {
    //     let p1 = (p0 + cycle);
    //     let p2 = (p0 + cycle * 2);
    //     ans += nums[p0].clone() * nums[p1].clone() * nums[p2].clone();
    // }
    // println!("{}", ans);




}