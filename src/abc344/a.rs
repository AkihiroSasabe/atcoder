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

    let mut flag_1 = false;
    let mut flag_2 = false;

    for i in 0..s.len() {
        if s[i] == '|' {
            if !flag_1 {
                flag_1 = true;
                continue
            }
            else {
                flag_2 = true;
                continue
            }
        }
        if flag_1 && !flag_2 {
            continue
        }
        print!("{}", s[i]);
    }
}