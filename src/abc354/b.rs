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
        n: usize
    }
    let mut s = vec![];
    let mut c = vec![];
    // let mut cs = vec![];
    let mut t = 0;
    for i in 0..n {
        input!{
            si: Chars,
            ci: usize
        }
        // s.push((si, i));
        s.push(si);
        c.push(ci);
        t += ci;
    }
    s.sort();
    for i in 0..n {
        if t % n == i {
            for si in s[i].iter() {
                print!("{}", si);
            }
            println!("");
            return;
        }
    }


}