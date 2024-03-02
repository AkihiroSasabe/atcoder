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
        a: isize,
        b: isize,
        d: isize,
    }

    let mut ans = vec![];
    ans.push(a);
    loop {
        let last = ans[ans.len()-1];
        let nx = last + d;
        if nx > b {
            break
        }
        ans.push(nx);
    }
    for x in ans {
        print!("{} ", x);
    }
}