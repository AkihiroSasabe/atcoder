use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use proconio::marker::Chars;
fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h]
    }
    let mut yx = vec![];
    for i in 0..h {
        for j in 0..w {
            // print!("{:?}", s[i][j]);
            if s[i][j] == 'o' {
                yx.push([i as isize,j as isize]);
            }
        }
    }
    let ans = (yx[0][0] - yx[1][0]).abs() + (yx[0][1] - yx[1][1]).abs();
    println!("{}", ans);
}