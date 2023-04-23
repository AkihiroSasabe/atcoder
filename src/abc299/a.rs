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
        s: Chars
    }
    let mut list = vec![];
    for i in 0..n {
        if s[i] == '|' {
            list.push('|')
        }
        if s[i] == '*' {
            list.push('*')
        }
    }
    if list[1] == '*' {
        println!("in");
    }
    else {
        println!("out");
    }
    // let mut first = false;
    // let mut mid = false;
    // // let mut last = false;
    // for i in 0..n {
    //     if s[i] == '|' {
    //         if first {
    //             if mid {
    //                 println!("in");
    //             }
    //             else {
    //                 println!("out");
    //             }
    //             return
    //         }
    //         else {
    //             first = true;
    //         }
    //     }
    //     if s[i] == '*' {
    //         mid = true;
    //     }
    // }
}