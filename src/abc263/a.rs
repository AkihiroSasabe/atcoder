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
        a: usize,
        b: usize,
        c: usize,
        d: usize,
        e: usize,
    }

    let mut list = vec![0; 15];
    list[a] += 1;
    list[b] += 1;
    list[c] += 1;
    list[d] += 1;
    list[e] += 1;
    list.sort();
    list.reverse();

    if list[0] == 3 && list[1] == 2 {
        println!("Yes");
    }
    else {
        println!("No");
    }

    // let mut counter = vec![];
    // for i in 0..list.len() {
        
    // }



}