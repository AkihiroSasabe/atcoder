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
        h: usize,
        w: usize,
        k: usize,
        c: [Chars; h],
    }


    let mut ans = 0;
    for mask in 0..1<<(h+w) {
        let mut hs = vec![false; h];
        for i in 0..h {
            if mask & (1 << i) != 0 {
                hs[i] = true;
            }
        }
        let mut ws = vec![false; w];
        for i in 0..w {
            if mask & (1 << (i+h)) != 0 {
                ws[i] = true;
            }
        }

        let mut count = 0;
        for y in 0..h {
            for x in 0..w {
                if hs[y] | ws[x] {
                    continue
                }
                else {
                    if c[y][x] == '#' {
                        count += 1;
                    }
                }
            }
        }
        // println!("mask = {:012b} ---- ---- ----", mask);
        // println!("count = {:?}", count);
        // println!("hs = {:?}", hs);
        // println!("ws = {:?}", ws);
        if count == k {

            ans+=1;
        }
    }
    println!("{}", ans);



}