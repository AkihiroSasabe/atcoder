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
    let mut l = vec![];
    let mut r = vec![];
    let mut lr = vec![];
    let mut rl = vec![];
    for i in 0..n {
        input!{
            li: isize,
            ri: isize,
        }
        l.push(li);
        r.push(ri);
        lr.push((li, ri));
        rl.push((ri, li));
    }
    let mut count = 0;
    let mut cands = l.clone();
    for i in 0..n {
        count += l[i];
    }

    for i in 0..n {
        if count == 0 {continue}
        else if count > 0 {continue}
        if count < 0 {
            if count.abs() < r[i] - l[i] {
                cands[i] = l[i] + count.abs();
                count = 0;
            }
            else {
                cands[i] = r[i];
                count += r[i] - l[i];
            }
        }
    }

    if count == 0 {
        println!("Yes");
        for i in 0..n {
            print!("{} ", cands[i]);
        }
    }
    else {
        println!("No");
    }


}