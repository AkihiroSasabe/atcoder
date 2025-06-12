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
        s: Chars,
    }
    let mut is_un_readable = true;

    for i in 0..s.len() {
        if i % 2 == 0 {
            // 英小文字
            if !s[i].is_ascii_lowercase() {
                is_un_readable = false;
            }
        }
        else {
            if !s[i].is_ascii_uppercase() {
                is_un_readable = false;
            }
        }
    }

    if is_un_readable {
        println!("Yes");
    }
    else {
        println!("No");
    }
}