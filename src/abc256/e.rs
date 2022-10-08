use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use proconio::marker::Chars;
use std::f64::consts::PI;
fn main() {
    input! {
        n: usize,
        // mut x: [usize; n],
        // mut c: [usize; n],
    }
    let mut cix = vec![vec![]; n];
    for i in 0..n {
        input! {
            x_i: usize,
            c_i: usize
        }
        cix[i].push(c_i);
        cix[i].push(i);
        cix[i].push(x_i);
    }
    prinlnt!("{:?}")
    

}