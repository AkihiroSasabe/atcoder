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
        s: Chars
    }
    let lowercase: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    let uppercase: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    for i in 0..26 {
        if s[0] == lowercase[i] {
            println!("No");
            return;
        }
    }

    for i in 1..s.len() {
        for j in 0..26 {
            if s[i] == uppercase[j] {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");

}