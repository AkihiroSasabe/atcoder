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
        m: usize,
        c: [Chars; n],
        d: [Chars; m],
        p: [usize; m+1] 
    }
    let mut ans = 0;

    for i in 0..n {
        let mut price = 0;
        for j in 0..m {
            if c[i] == d[j] {
                price = p[j+1];
            }
        }
        if price == 0 {
            price = p[0];
        }
        ans += price;
    }
    println!("{}", ans);

}