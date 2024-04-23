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
    

    // let mut set = BTreeSet::new();
    let mut poss = vec![0; n];
    for i in 0..n {
        a[i] -= 1;
        // if a[i] != i {
        //     set.insert(i);
        // }
        poss[a[i]] = i;
    }

    let mut ans = vec![];
    for i in 0..n-1 {
        if a[i] != i {
            let ni = poss[i];
            // 入れ替え
            ans.push([min(i + 1, ni + 1), max(i + 1, ni + 1)]);

            let val = a[i];
            a[i] = i;
            a[ni] = val;

            poss[i] = i;
            poss[val] = ni;

            // let ni = set.iter().rev().next().unwrap();
        }
    }
    let num = ans.len();
    println!("{}", num);
    for i in 0..num {
        println!("{} {}", ans[i][0], ans[i][1]);
    }

}
