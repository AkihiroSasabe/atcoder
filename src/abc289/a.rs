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
        s: Chars
    }
    let mut s_copy = vec![];

    for i in 0..s.len() {
        if s[i] == '0' {
            s_copy.push('1');
        }
        else if s[i] == '1' {
            s_copy.push('0');
        }
    }
    for i in 0..s_copy.len() {
        print!("{}", s_copy[i]);
    }
}