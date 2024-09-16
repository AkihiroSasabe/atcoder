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
use rand::Rng;
fn main() {
    input! {
        s: [char; 3]
    }

    let mut a = 0;
    let mut b = 0;
    let mut c = 0;

    if s[0] == '<' {
        a -= 1;
        b += 1;
    }
    else {
        a += 1;
        b -= 1;
    }
    if s[2] == '<' {
        b -= 1;
        c += 1;
    }
    else {
        b += 1;
        c -= 1;
    }


    if s[1] == '<' {
        a -= 1;
        c += 1;
    }
    else {
        a += 1;
        c -= 1;
    }

    let mut aaa = vec![(a,0),(b,1),(c,2),];
    aaa.sort();
    // let lowercase: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    let uppercase: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();


    println!("{}", uppercase[aaa[1].1]);




}