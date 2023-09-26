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
        d: usize,
        s: [Chars; n]
    }

    let mut ans = 0;
    let mut current = 0;
    for ddd in 0..d {
        let mut flag = true;
        for i in 0..n {
            if s[i][ddd] == 'x' {
                flag = false;
            }
        }
        if flag {
            current += 1;
        }
        else {
            current = 0;
        }
        ans = max(ans, current);
    }
    println!("{}", ans);

}