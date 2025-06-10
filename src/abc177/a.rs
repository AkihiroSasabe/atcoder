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
    // バチャ
    // 12:02 まで
    // 2025-06-07 11:22- 
    input! {
        d: usize, // 距離
        t: usize, // 時刻
        s: usize, // 速度
    }
    if s * t >= d {
        println!("Yes");
    }
    else {
        println!("No");
    }

}