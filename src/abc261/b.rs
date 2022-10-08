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
        a: [Chars; n]
    }

    let mut flag = true;
    for i in 0..n {
        for j in 0..n {
            if a[i][j] == 'W' && a[j][i] != 'L' {
                flag = false;
            }
            if a[i][j] == 'L' && a[j][i] != 'W' {
                flag = false;
            }
            if a[i][j] == 'D' && a[i][j] != a[j][i] {
                flag = false;
            }
        }
    }
    if flag {
        println!("correct");
    }
    else {
        println!("incorrect");
    }
}