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
        s: Chars
    }
    if s[0] == s[1] && s[1] == s[2] {
        println!("-1");
    }
    else if s[0] == s[1] {
        println!("{}", s[2]);
    }
    else if s[1] == s[2] {
        println!("{}", s[0]);
    }
    else if s[2] == s[0] {
        println!("{}", s[1]);
    }
    else {
        println!("{}", s[0]);
    }

}