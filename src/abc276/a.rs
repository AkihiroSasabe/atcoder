#![allow(dead_code, unused_imports)]
use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    input! {
        s: Chars
    }
    let mut total = s.len();
    let mut ans = -1;
    for i in 0..total {
        if s[total - 1 -i] == 'a' {
            ans = (total -i) as isize;
            break
        }
    }
    println!("{}", ans);

}