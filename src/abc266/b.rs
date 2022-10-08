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
        n: isize
    }
    let waru = 998_244_353;
    let mut x = n % waru;
    if x < 0 {
        x += waru;
    }
    println!("{}", x);
}