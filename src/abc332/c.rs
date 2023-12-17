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
        m: usize, // 無地T
        s: Chars,
    }

    let mut muji_now = m;
    let mut logo_now = 0;

    let mut ans = 0;
    for i in 0..n {
        if s[i] == '0' {
            // reset
            muji_now = m;
            logo_now = 0;
        }
        if s[i] == '1' {
            if muji_now > 0 {
                muji_now -= 1;
            }
            else {
                logo_now += 1;
            }
        }
        if s[i] == '2' {
            logo_now += 1;
        }
        ans = max(ans, logo_now);
    }
    println!("{}", ans);

}