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
    //バチャ
    // 2025-07-24 19:57-21:37
    input! {
        n: usize
    }
    let r = n % 10;
    match r {
        2 => println!("hon"),
        4 => println!("hon"),
        5 => println!("hon"),
        7 => println!("hon"),
        9 => println!("hon"),
        0 => println!("pon"),
        1 => println!("pon"),
        6 => println!("pon"),
        8 => println!("pon"),
        3 => println!("bon"),
        _ => unreachable!(),
    }
}