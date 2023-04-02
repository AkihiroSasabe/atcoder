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
        s: Chars
    }
    let mut kako = s[0];
    let mut flag = true;
    for i in 1..n {
        if kako == s[i] {
            flag = false;
            break
        }
        kako = s[i];
    }
    if flag {
        println!("Yes");
    }
    else {
        println!("No");
    }

}