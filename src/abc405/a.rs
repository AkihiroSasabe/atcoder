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
    input! 
    {
        r: usize,
        x: usize,
    }

    if x == 1 {
        if 1600 <= r && r <= 2999 {
            println!("Yes");
        }
        else {
            println!("No");
        }
    }
    else {
        if 1200 <= r && r <= 2399 {
            println!("Yes");
        }
        else {
            println!("No");
        }
    }


}