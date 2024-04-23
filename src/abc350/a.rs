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
        s: Chars
    }

    let num = 
        (s[3] as usize - '0' as usize) * 100 + 
        (s[4] as usize  - '0' as usize) * 10 + 
        (s[5] as usize  - '0' as usize);
    // println!("num = {:?}", num);

    for i in 1..350 {
        if i == 316 {continue}
        if num == i {
            println!("Yes");
            return
        }
    }
    println!("No");
}