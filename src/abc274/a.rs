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
        a: f64,
        b: f64,
    }
    let mut num = b / a;
    // let base_number = 1000;
    // let rounded = (num * base_number).round() / base_number;
    // println!("{:.3}", rounded);
    println!("{:.3}", num);

}