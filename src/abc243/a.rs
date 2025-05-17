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
    // ばちゃ
    // 2025-05-16 20:20:00-
    input! {
        mut v: isize,
        a: isize,
        b: isize,
        c: isize,
    }

    let mut cons = vec![a,b,c];
    let mut index = 0;
    let mut anss = vec!['F','M','T'];
    loop {
        if v < cons[index] {
            println!("{}", anss[index]);
            return;
        }

        v -= cons[index];
        index += 1;
        index %= 3;
    }


    // if v < a {
    //     println!("F");
    // }
    // else if v < a + b {
    //     println!("M");
    // }

}