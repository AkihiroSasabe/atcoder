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
fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut ans = 0;
    for i in 0..n {
        for j in i..n {
            let mut sum = a[i] + a[j];
            
            while sum % 2 == 0 {
                sum /= 2;
            }
            ans += sum;
        }
    }
    println!("{}", ans);

}