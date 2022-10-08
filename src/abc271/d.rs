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
        n: usize,
        s: isize,
    }
    let mut a = vec![];
    let mut b = vec![];
    for i in 0..n {
        input! {
            a_i: isize,
            b_i: isize
        }
        a.push(a_i);
        b.push(a_i);
    }
    // let mut diff = vec![];
    // for i in 0..n {
    //     diff.push(vec![b[i] - a[i], i as isize]);
    // }
    // let mut a_sum = 0;
    // for i in 0..n {
    //     a_sum += a[i];
    // }
    // diff.sort();

    // let mut diff_cum = vec![0; n];
    // diff_cum[]

    let mut min_sum = 0;
    let mut max_sum = 0;
    for i in 0..n {
        min_sum = min(a[i], b[i]);
        max_sum = max(a[i], b[i]);
    }


    // let mut dp = vec![vec![0; 10_001]; 100];
    // for i in 0..n {
    //     for j in 0..10_001 {

    //     }
    // }







}