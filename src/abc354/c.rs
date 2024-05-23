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
    }
    let mut ac = vec![];
    for i in 0..n {
        input!{
            ai: isize, // つよさ
            ci: isize, // こすと
        }
        ac.push((ai, -ci, i as isize + 1));
    }
    ac.sort();

    // ac.reverse();
    // let mut pre_a = ac[n-1][0];
    let mut max_c = ac[n-1].1;
    let mut rs = vec![ac[n-1].2];
    for i in (0..n-1).rev() {
        if ac[i].1 > max_c {
            rs.push(ac[i].2);
        }
        max_c = max(max_c, ac[i].1);
    }
    rs.sort();
    println!("{}", rs.len());
    for r in rs {
        print!("{} ", r);
    }


}