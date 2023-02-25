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
use std::collections::HashSet;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [isize; n],
        q: usize
    }
    let mut l = vec![];
    let mut r = vec![];
    let mut rl = vec![];
    for i in 0..q {
        input! {
            l_i: usize,
            r_i: usize,
        }
        l.push(l_i-1);
        r.push(r_i-1);
        rl.push(vec![r_i-1, l_i-1, i]);
    }
    rl.sort();
    rl.reverse();
    let r_i = rl[0][0];
    let l_i = rl[0][1];
    let ii = rl[0][2];

    while {
        for kez in 0..k {
            let aaa = a[r_i];
            
        }
    }


}