#![allow(dead_code, unused_imports)]
use proconio::{input, marker::Usize1};
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
use rand::Rng;
fn main() {
    input! {
        n: usize,
        s: Chars
    }

    // 3
    // 1,/,2

    if n % 2 == 0 {
        println!("No");
        return
    }

    for i in 0..n/2 {
        if s[i] != '1' {
            println!("No");
            // println!("000");
            return
        }
    }
    if s[n/2] != '/' {
        println!("No");
        // println!("111");
            return
    }
    for i in n/2+1..n {
        if s[i] != '2' {
            println!("No");
            // println!("222");
            return
        }
    }
    println!("Yes");


}