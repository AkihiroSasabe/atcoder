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
use superslice::*;
fn main() {
    input! {
        n: usize
    }

    let mut keta1 = n % 16;
    let mut keta2 = (n / 16) % 16;

    let aaa: Vec<char> = "ABCDEF".chars().collect();

    let mut k1: char = '0';
    let mut k2: char = '0';
    if keta1 >= 10 {
        keta1 = keta1 - 10;
        k1 = aaa[keta1];
    }
    else {
        k1 = std::char::from_digit(keta1 as u32, 10).unwrap();
    }

    if keta2 >= 10 {
        keta2 = keta2 - 10;
        k2 = aaa[keta2];
    }
    else {
        k2 = std::char::from_digit(keta2 as u32, 10).unwrap();
    }
    
    println!("{}{}", k2, k1);
}
