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
        s: Chars
    }
    let mut t = vec![];
    let mut res = k;
    for i in 0..n {
        if res != 0 && s[i] == 'o' {
            t.push('o');
            res -= 1;
        }
        else {
            t.push('x');
        }
    }
    for i in 0..n {
        print!("{}", t[i]);
    }

}