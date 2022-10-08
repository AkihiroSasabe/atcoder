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
        l: usize,
        r: usize
    }
    let mut a: Vec<char> = "atcoder".chars().collect();

    for i in (l-1)..r {
        print!("{}", a[i])
    }




}