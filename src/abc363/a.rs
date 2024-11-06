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
        r: isize
    }

    let tg0 = 300;
    let tg1 = 200;
    let tg2 = 100;

    let mut ans = 1000;
    if tg2 > r {
        ans = min(ans, tg2 - r);
    }
    if tg1 > r {
        ans = min(ans, tg1 - r);
    }
    if tg0 > r {
        ans = min(ans, tg0 - r);
    }
    println!("{}", ans);    

}