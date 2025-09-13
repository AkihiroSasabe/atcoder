#![allow(dead_code, unused_imports)]
use proconio::{input, marker::{Usize1, Isize1}};
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
use num::{BigUint, ToPrimitive};
use num_bigint::ToBigUint;
fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }
    // if n == 2 {
    //     println!("Yes");
    //     return
    // }
    // let r = a[1] / a[0];
    // if a[1] % a[0] != 0 {
    //     println!("No");
    //     return
    // }
    // for i in 0..n-1 {
    //     if a[i+1] != a[i] * r {
    //         println!("No");
    //         return
    //     }
    // }

    for i in 0..n-2 {
        if a[i+1] * a[i+1] != a[i] * a[i+2] {
            println!("No");
            return
        }
    }
    println!("Yes");
}