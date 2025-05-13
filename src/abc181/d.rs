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
        ss: Chars
    }
    let n = ss.len();
    let mut s: Vec<usize> = ss.iter().map(|x| x.to_digit(10).unwrap() as usize).collect();
    let mut counter = vec![0; 10];
    for &si in s.iter() {
        counter[si] += 1;
    }

    if n <= 3 {
        for perm in s.iter().permutations(n) {
            let mut pow = 1;
            let mut x = 0;
            for i in 0..n {
                x += pow * perm[i];
                pow *= 10;
            }
            if x % 8 == 0 {
                println!("Yes");
                return;
            }
        }
        println!("No");
        return;
    }


    // 8の倍数 <=> a * 1000 + b * 8;
    // 下3桁が8で割れればok

    // 1000 / 8 = 125
    for i in 1..126 {
        let mut x = 8 * i;
        let mut set = BTreeMap::new();
        for _ in 0..3 {
            let r = x % 10;
            *set.entry(r).or_insert(0) += 1;
            x /= 10;
        }
        let mut is_ok = true;

        for (key, num) in set {
            if counter[key] < num {
                is_ok = false;
            }
        }
        if is_ok {
            println!("Yes");
            return;
        }
    }
    println!("No");








}