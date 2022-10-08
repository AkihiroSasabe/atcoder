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
    let mut s = vec![];
    for i in 0..10 {
        input! {
            s_i: Chars
        }
        s.push(s_i);
    }

    let mut a: usize = 10;
    let mut b: usize = 1;
    let mut c: usize = 10;
    let mut d: usize = 1;
    for i in 0..10 {
        let mut flag = false;
        for j in 0..10 {
            if s[i][j] == '#' {
                flag = true;
                if flag {
                    c = min(c, j+1);
                    d = max(d, j+1);
                }
            }
        }
        if flag {
            a = min(a, i+1);
            b = max(b, i+1);
        }
    }
    println!("{} {}", a, b);
    println!("{} {}", c, d);

}