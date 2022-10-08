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
    }
    // let mut l = vec![];
    // let mut r = vec![];
    // let mut rl = vec![vec![]; n];
    let mut lr = vec![vec![]; n];

    for i in 0..n {
        input! {
            l_i: usize,
            r_i: usize
        }
        // l.push(l_i);
        // r.push(r_i);
        // rl[i].push(r_i);
        // rl[i].push(l_i);
        lr[i].push(l_i);
        lr[i].push(r_i);
    }
    // println!("{:?}", rl);
    // rl.sort();
    lr.sort();
    // println!("{:?}", rl);

    // let mut ans = vec![];
    let mut current_start_time = lr[0][0];
    let mut current_end_time = lr[0][1];
    // let mut update_flag = false;
    for i in 0..n {
        if lr[i][0] <= current_end_time {
            current_end_time = max(current_end_time, lr[i][1]);
            // update_flag = false;
        } 
        else {
            println!("{} {}", current_start_time, current_end_time);
            current_start_time = lr[i][0];
            current_end_time = lr[i][1];
            // update_flag = true;
        }
    }
    println!("{} {}", current_start_time, current_end_time);
}