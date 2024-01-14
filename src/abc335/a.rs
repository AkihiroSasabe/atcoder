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
        mut s: Chars
    }
    let index = s.len()-1;
    s[index] = '4';
    // let mut sss = vec!['2', '0', '2', '4'];
    // for i in 0..4 {
    //     let ind = s.len()-5 + i;
    //     s[ind] = sss[i];
    // }
    for i in 0..s.len() {
        print!("{}", s[i]);
    }

}