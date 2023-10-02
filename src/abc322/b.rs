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
        m: usize,
        s: Chars,
        t: Chars,
    }
    
    let mut head = true;
    for i in 0..n {
        if s[i] != t[i] {
            head = false;
            break;
        }
    }

    let mut tail = true;
    for i in 0..n {
        if s[s.len()-1-i] != t[t.len()-1-i] {
            tail = false;
            break;
        }
    }

    if head && tail {
        println!("0");
    }
    else if head && !tail {
        println!("1");
    }
    else if !head && tail {
        println!("2");
    }
    else {
        println!("3");
    }

}