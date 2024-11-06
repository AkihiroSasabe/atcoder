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
        n: usize,
        mut t: isize,
        p: isize,
        l: [isize; n],
    }

    let mut ans = 0;
    loop {
        let mut count = 0;
        for i in 0..n {
            if l[i] >= t {
                count += 1;
            }
        }
        if count >= p {
            println!("{}", ans);
            return
        }
        t -= 1;
        ans += 1;
    }


}