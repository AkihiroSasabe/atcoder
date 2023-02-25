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

    }
    let mut a = vec![];
    let mut b = vec![];

    for i in 0..n {
        input! {
            a_i: isize,
            b_i: isize,
        }
        a.push(a_i);
        b.push(b_i);
    }
    for i in 0..n {
        println!("{}", a[i]+b[i]);
    }
}