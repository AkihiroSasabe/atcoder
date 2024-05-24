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
    // 2024-05-24 19:58-
    input! {
        s: Chars
    }

    // let mut is_YYxx = true;
    // let mut is_xxYY = true;

    let mut is_MMxx = false;
    let mut is_xxMM = false;

    let zero = '0' as usize;
    let pre = (s[0] as usize - zero) * 10 + s[1] as usize-  zero;
    let post = (s[2] as usize - zero) * 10 + s[3] as usize-  zero;

    if 1 <= pre && pre <= 12 {
        is_MMxx = true;
    } 
    if 1 <= post && post <= 12 {
        is_xxMM = true;
    }

    if is_MMxx && is_xxMM {
        println!("AMBIGUOUS");
    }
    else if !is_MMxx && is_xxMM {
        println!("YYMM");
    }
    else if is_MMxx && !is_xxMM {
        println!("MMYY");
    }
    else {
        println!("NA");
    }


}