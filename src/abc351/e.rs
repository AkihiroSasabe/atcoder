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
    // 2024-04-27 22:01-22:40 (39min)
    // 2024-04-28 10:46-11:29 (43min, evimaとkyopro_friendsと公式の解説見た。)
    // 82min
    input! {
        n: usize,
    }
    let mut odd_u = vec![];
    let mut odd_v = vec![];
    let mut even_u = vec![];
    let mut even_v = vec![];
    for i in 0..n {
        input! {
            xi: isize,
            yi: isize,
        }
        if (xi + yi) % 2 == 0 {
            let ui = (xi - yi) / 2;
            let vi = (xi + yi) / 2;
            even_u.push(ui);
            even_v.push(vi);
        }
        else {
            let ui = (xi + 1 - yi) / 2;
            let vi = (xi + 1 + yi) / 2;
            odd_u.push(ui);
            odd_v.push(vi);
        }
    }
    even_u.sort();
    even_v.sort();
    odd_u.sort();
    odd_v.sort();

    let mut ans = 0;
    if even_u.len() > 0 {
        let mut num = 0;
        for i in 0..even_u.len()-1 {
            num += even_u.len() - 1 - i;
            num -= i;
            ans += (even_u[i+1] - even_u[i] + even_v[i+1] - even_v[i]) * num as isize;
        }
    }

    if odd_u.len() > 0 {
        let mut num = 0;
        for i in 0..odd_u.len()-1 {
            num += odd_u.len() - 1 - i;
            num -= i;
            ans += (odd_u[i+1] - odd_u[i] + odd_v[i+1] - odd_v[i]) * num as isize;
        }
    }
    println!("{}", ans);
}