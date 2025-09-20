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
        m: usize,
        a: [usize; m],
    }

    let mut set = BTreeSet::new();
    for i in 0..m {
        set.insert(a[i]);
    }


    let mut ans = vec![];
    for i in 1..n+1 {
        if !set.contains(&i) {
            ans.push(i);
        }
    }
    ans.sort();
    println!("{}", ans.len());
    for aaa in ans {
        print!("{} ", aaa);
    }
    println!("");

}