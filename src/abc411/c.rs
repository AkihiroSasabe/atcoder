#![allow(dead_code, unused_imports)]
use proconio::{input, marker::{Usize1, Isize1, Chars}};
use itertools::Itertools;
use std::{cmp::{max, min, Ordering, Reverse}, vec};
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
        q: usize,
        a: [usize; q],
    }
    let mut is_white = vec![true; n+2];

    let mut ans = 0;
    for i in 0..q {
        let ai = a[i];
        if is_white[ai] {
            if !is_white[ai-1] && !is_white[ai+1] {
                ans -= 1;
            }
            else if is_white[ai-1] && is_white[ai+1] {
                ans += 1;
            }
        }
        else {
            if is_white[ai-1] && is_white[ai+1] {
                ans -= 1;
            }
            else if !is_white[ai-1] && !is_white[ai+1] {
                ans += 1;
            }
        }
        // println!("ai = {:?}", ai);
        is_white[ai] = !is_white[ai];
        // println!("is_white = {:?}", is_white);
        println!("{}", ans);

    }



}