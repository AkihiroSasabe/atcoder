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
        n: usize
    }

    let mut a: Vec<Vec<usize>> = vec![vec![]; n];

    for i in 0..n {
        let mut a_i: Vec<usize> = vec![1; i+1];
        for j in 0..i+1 {
            if j == 0 || j == i {
                a_i[j] = 1;
            }
            else {
                a_i[j] = a[i-1][j-1] + a[i-1][j];
            }
            print!("{} ", a_i[j]);
        }
        a[i] = a_i;
        println!("");
    }

}