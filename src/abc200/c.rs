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
fn main() {
    input! {
        n: usize,
        mut a: [usize; n]
    }

    // cum[i][r] := 剰余がrとなる項が、iまでに何個あるか?
    let mut cum: Vec<Vec<usize>> = vec![vec![0; 200]; n];
    for i in 0..n {
        a[i] %= 200;
        cum[i][a[i]] += 1;
    }
    for i in 1..n {
        for num in 0..200 {
            cum[i][num] += cum[i-1][num];
        }
    }

    let mut ans = 0;
    for j in 1..n {
        ans += cum[j-1][a[j]];
    }
    println!("{}", ans);


}