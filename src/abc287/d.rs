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
        s: Chars,
        t: Chars
    }
    let mut hash = HashMap::new();
    for i in 0..t.len() {
        if t[i] == '?' {
            hash.insert(i, 0);
        }
    }


    // let mut pre = VecDeque::new();
    // let mut post = VecDeque::new();
    
    let mut post_flag = true;
    let post = s.len() - t.len();
    let mut dame_post = 0;

    for i in 0..t.len() {
        if s[post+i] != t[i] && s[post+i] != '?' && t[i] != '?' {
            post_flag = false;
            dame_post = max(post+i, dame_post);
        }
    }
    if post_flag {
        println!("Yes");
    }
    else {
        println!("No");
    }


    let mut pre_flag = true;
    let mut post_flag = false;
    for x in 0..t.len()  {
        if s[x] != t[x] && s[x] != '?' && t[x] != '?' {
            pre_flag = false;
        }
        let post: isize = s.len() as isize + 1 - t.len() as isize  + x as isize;
        if (dame_post as isize) < post {
            post_flag = true;
        }
        if post_flag && pre_flag {
            println!("Yes");
        }
        else {
            println!("No");
        }
    }
}