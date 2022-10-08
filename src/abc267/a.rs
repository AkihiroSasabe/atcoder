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
        s: String
    }
    let ss = vec!["Monday", "Tuesday", "Wednesday", "Thursday", "Friday"];

    for i in 0..5 {
        if s == ss[i] {
            println!("{}", 5 - i);
        }
    }

}