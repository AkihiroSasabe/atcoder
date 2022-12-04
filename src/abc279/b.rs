#![allow(dead_code, unused_imports)]
use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    input! {
        s: Chars,
        t: Chars,
    }
    let mut flag = false;
    for i in 0..s.len() {
        let mut break_flag = false;
        for j in 0..t.len() {
            if i+j >= s.len() {
                break_flag = true;
                break
            }
            if s[i+j] != t[j] {
                break_flag = true;
                break
            }
        }
        if !break_flag {
            flag = true;
        }
    }
    if flag {
        println!("Yes");
    }
    else {
        println!("No");
    }

}