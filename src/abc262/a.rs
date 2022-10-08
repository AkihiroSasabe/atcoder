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
        y: usize
    }

    let amari = y % 4;
    let ans: usize;
    if amari == 0 {
        ans = y + 2;
    }
    else if amari == 1 {
        ans = y + 1;
    }
    else if amari == 2 {
        ans = y;
    }
    else {
        ans = y + 3;
    }
    println!("{}", ans);

}