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
    let mut a = vec![vec![];n];
    for i in 0..n {
        input! {
            a_i: Chars,
        }
        // println!("{:?}", a_i);
        for j in 0..n {
            a[i].push(a_i[j] as usize - 48);
        }
    }
    let cand: [[isize; 2]; 8] = [[-1,-1], [-1,0], [-1,1], [0, -1], [0, 1], [1, -1], [1, 0], [1, 1]];
    let mut max_sum = 0;
    for i in 0..n {
        for j in 0..n {
            for k in 0..8 {
                let cy = cand[k][0];
                let cx = cand[k][1];
                let mut sum = 0;
                let mut y = i as isize;
                let mut x = j as isize;
                for l in 0..n {
                    sum *= 10;
                    sum += a[y as usize][x as usize];
                    y = (y + cy) % n as isize;
                    x = (x + cx) % n as isize;
                    if y < 0 {y = y + n as isize;}
                    if x < 0 {x = x + n as isize;}
                }
                max_sum = max(sum, max_sum)
            }
        }
    }
    println!("{}", max_sum);
}