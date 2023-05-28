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
        s: Chars,
        t: Chars
    }
    // let mut flag = true;
    for i in 0..n {
        if s[i] != t[i] {
            if !((s[i] == '1' && t[i] == 'l') || (t[i] == '1' && s[i] == 'l')) {
                if !((s[i] == '0' && t[i] == 'o') || (t[i] == '0' && s[i] == 'o')) {
                    // flag = false;
                    println!("No");
                    return
                }
            }   
        }
    }
    println!("Yes");

}