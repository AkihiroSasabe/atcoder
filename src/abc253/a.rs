use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use proconio::marker::Chars;
fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }
    let mut x = vec![a,b,c];
    x.sort();
    if x[1] == b {
        println!("Yes");
    }
    else {
        println!("No");
    }
}