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

    let mut yx = vec![];

    for x in 0..1_001 {
        for y in 0..1_001 {
            if x * x + y * y == m {
                let yy = y as isize;
                let xx = x as isize;
                yx.push(vec![yy, xx]);
                yx.push(vec![-yy, xx]);
                yx.push(vec![yy, -xx]);
                yx.push(vec![-yy, -xx]);
            }
        }
    }
    
    let mut dist = vec![vec![-1; n]; n];
    dist[0][0] = 0;
    let mut todo: VecDeque<Vec<isize>> = VecDeque::new();
    todo.push_back(vec![0, 0]);
    while todo.len() != 0 {
        let v = todo.pop_front().unwrap();
        let y = v[0];
        let x = v[1];
        for i in 0..yx.len() {
            let next_y = y + yx[i][0];
            let next_x = x + yx[i][1];

            if next_y < 0 || next_y >= n as isize || next_x < 0 || next_x >= n as isize {continue}
            if dist[next_y as usize][next_x as usize] != -1 {continue}

            dist[next_y as usize][next_x as usize] = dist[y as usize][x as usize] + 1;
            todo.push_back(vec![next_y, next_x]);
        }
    }
    for i in 0..n {
        for j in 0..n {
            print!("{} ", dist[i][j]);
        }
        println!("");
    }



}