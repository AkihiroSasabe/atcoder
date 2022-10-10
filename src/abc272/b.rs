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
        m: usize,
    }

    let mut x = vec![];
    for i in 0..m {
        input! {
            k_i: usize,
            x_i: [usize; k_i]
        }
        x.push(x_i);
    }
    for i in 0..m {
        for j in 0..x[i].len() {
            x[i][j] -= 1;
        }
    }

    let mut graph = vec![vec![false; n]; n];
    for i in 0..n {
        graph[i][i] = true;
    }
    for i in 0..m {
        for j in 0..x[i].len() {
            for k in (j+1)..x[i].len() {
                graph[x[i][j]][x[i][k]] = true;
                graph[x[i][k]][x[i][j]] = true;
            }
        }
    }

    let mut flag = true;
    for i in 0..n {
        for j in 0..n {
            if !graph[i][j] {
                flag = false;
            }
        }
    }
    if flag {
        println!("Yes");
    }
    else {
        println!("No");
    }

}