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
        m: usize
    }

    for i in 1..(m+1) {
        let mut now_list = vec![i];
        saiki(0, i, now_list, n, m);
    }

}

fn saiki(digit: usize, mut val: usize, mut now_list: Vec<usize>, n: usize, m: usize) {
    if digit == n - 1 {
        for i in 0..n {
            print!("{} ", now_list[i]);
        }
        println!("");
        return;
    }
    loop {
        // println!("n:{n} digit:{digit} m:{m} val:{val}");
        val += 1;
        if m < val {
            // println!("skip!!! digit:{} val:{}", digit + 1, val);
            break;
        }
        // if digit + 1 == 1 {break}
        let mut next_list = now_list.clone();
        next_list.push(val);
        // now_list[digit + 1] = val;
        saiki(digit + 1, val, next_list, n, m);
    }

}