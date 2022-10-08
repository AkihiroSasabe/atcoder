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
        n: usize,
    }
    let mut p = vec![0];
    for i in 0..(n-1) {
        input! {
            p_i: usize
        }
        p.push(p_i - 1);
    }

    let mut seen = vec![false; n];
    for i in 1..n {
        seen[p[i]] = true;
    }
    // println!("{:?}", seen);
    let mut child = 0;
    for i in 0..n {
        if !seen[i] {
            child = i;
        }
    }
    // println!("max child: {}", child);


    let mut count_1 = 0;
    let mut count_n = 0;
    let mut count = 0;
    // while count < n - 1 {
    loop {
        let next = p[child];
        count += 1;

        if next == 0 {
            count_1 = count;
            break
        }
        else if next == n -1 {
            count_n = count;
        }
        child = next;
        // println!("child: {}", child);
    }

    println!("{}", count_1 - count_n);

}

