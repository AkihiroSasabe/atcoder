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
        d: usize,
    }
    let mut ts = vec![];
    let mut ls = vec![];
    for i in 0..n {
        input!{
            tsi: usize,
            lsi: usize,
        }
        ts.push(tsi);
        ls.push(lsi);
    }

    for k in 1..d+1 {
        let mut ans = 0;
        for i in 0..n {
            let w = ts[i] * (ls[i] + k);
            ans = max(ans, w); 
        }
        println!("{}", ans);
    }
    


}