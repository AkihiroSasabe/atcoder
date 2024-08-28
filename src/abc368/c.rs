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
    input! {
        n: usize,
        mut h: [isize; n],
    }
    let mut t: isize = 0;

    for i in 0..n {
        let r = t % 3;
        if r == 0 {
            // そのまま
        }
        else if r == 1 {
            if h[i] > 1 {
                h[i] -= 4;
                t += 2;
            }
            else {
                // i を殺して次へ
                t += 1;
                continue
            }
        }
        else if r == 2 {
            h[i] -= 3;
            t += 1;
        }

        if h[i] <= 0 {
            continue
        }

        let r2 = h[i] % 5;
        let d2 = h[i] / 5;
        if r2 == 0 {
            t += d2 * 3;
        }
        else if r2 == 1 {
            t += d2 * 3;
            t += 1;
        }
        else if r2 == 2 {
            t += d2 * 3;
            t += 2;
        }
        else {
            t += d2 * 3;
            t += 3;
        }


        // let r = h[i] % 3;
        // t += r;
        // h[i] -= r;
        // t += h[i] / 3;
    }
    println!("{}", t);


}