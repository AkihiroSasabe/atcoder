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
    // 23-01-01 15:47-16:02 => 15min
    input! {
        n: usize,
        q: usize,
        s: Chars,
    }
    let mut query = vec![];
    for i in 0..q {
        input! {
            t_i: usize,
            x_i: usize
        }
        query.push(vec![t_i, x_i]);
    }
    let mut head = 0;
    for i in 0..q {
        let t_i = query[i][0];
        let x_i = query[i][1];
        if t_i == 1 {
            head = (n + head - x_i) % n;
        }
        else if t_i == 2 {
            println!("{}", s[(head + x_i - 1) % n]);
        }
    }


}