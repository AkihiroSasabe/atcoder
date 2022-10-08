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
        l1: usize,
        r1: usize,
        l2: usize,
        r2: usize,
    }
    let answer: usize;
    // if l1 <= l2 && l2 <= r1 {
    //     answer = r1 - l2;
    // }
    // else if l2 <= l1 && l1 <= r2 {
    //     answer = r2 - l1;
    // }
    // else {
    //     answer = 0;
    // }
    if l1 <= l2  && l2 <= r1{
        let right = min(r1, r2);
        answer = right - l2;
    }
    else if l2 <= l1 && l1 <= r2 {
        let right = min(r1, r2);
        answer = right - l1;
    }
    else {
        answer = 0;
    }
    println!("{}", answer);

}