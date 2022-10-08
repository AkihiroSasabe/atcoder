use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let mut flag = true;
    if s.len() > t.len() {
        flag = false;
    }
    else {
        for i in 0..s.len() {
            if s[i] != t[i] {
                flag = false;
                break
            }
        }
    }
    if flag {
        println!("Yes");
    }
    else {
        println!("No");
    }
}