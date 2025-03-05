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
        s: Chars
    }

    let n = s.len();
    let mut ans: usize = 0;
    for i in 0..n {
        for j in i+1..n {
            for k in j+1..n {
                if j-i == k-j {
                    if s[i] == 'A' && s[j] == 'B' && s[k] == 'C' {
                        ans += 1;
                    }
                }
            }
        }
    }
    println!("{}", ans);


}