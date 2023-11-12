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
        q: usize,
        s: Chars,
    }
    let mut l = vec![];
    let mut r = vec![];
    for i in 0..q {
        input! {
            l_i: usize,
            r_i: usize,
        }
        l.push(l_i - 1);
        r.push(r_i - 1);
    }
    let mut cum = vec![0; n];
    let mut hantei = vec![0; n];

    for i in 0..(n-1) {
        if s[i] == s[i+1] {
            hantei[i] = 1;
        }
        if i == 0 {
            cum[i] = hantei[i];
        }
        else {
            cum[i] = cum[i-1] + hantei[i];
        }
    }
    if n > 1 {
        cum[n-1] = cum[n-2];
    }

    // println!("hantei = {:?}", hantei);
    // println!("cum    = {:?}", cum);

    for i in 0..q {
        
        let mut ans = if l[i] == 0 {
            cum[r[i]]
        }
        else {
            cum[r[i]] - cum[l[i]-1]
        };
        // println!("ans = {:?}", ans);
        if l[i] == r[i] && hantei[l[i]] == 1 {
            ans = 0;
        }
        else if hantei[r[i]] == 1 {
            ans -= 1;
        }
        println!("{}", ans);
    }

}