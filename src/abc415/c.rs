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
    // 2025-07-20 11:15-11:44 (29min)
    input! {
        t: usize, // 1<=ti<=40_000
    }
    let mut n = vec![];
    let mut s = vec![];
    for i in 0..t {
        input!{
            ni: usize, // 1<= ni <= 18
            si: Chars, // 危険な状態
        }
        n.push(ni);
        s.push(si);
    }
    // 262_144

    for i in 0..t {
        // println!("i = {} ------", i);
        let mut bans = BTreeSet::new();

        let ni = n[i];
        for j in 0..((1<<ni)-1) {
            if s[i][j] == '1' {
                bans.insert(j+1);
            }
        }
        let mut oks = vec![false; 1<<ni];
        oks[0] = true;
        for mask in 0..1<<ni {
            if !oks[mask] {continue}
            for j in 0..ni {
                let nmask = mask | (1 << j);
                if bans.contains(&nmask) {
                    continue;
                }
                oks[nmask] = true;
            }
        }

        // println!("bans");
        // for b in bans {
        //     print!("b = {:b} ", b);
        // }
        // println!("");
        // for mask in 0..1<<ni {
        //     println!("oks[{:b}] = {}", mask, oks[mask]);
        // }

        if oks[(1<<ni)-1] {
            println!("Yes");
        }
        else {
            println!("No");
        }

    }

}