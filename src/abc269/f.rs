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
    // input! {
        
    // }

    let n = 10;
    println!("{}", n);
    let mut x_seen = vec![false; n];
    let mut y_seen = vec![false; n];
    let mut yx = vec![vec!['x'; n]; n];
    for y in 0..n {
        for x in 0..n {
            yx[y][x] = '_';
            dfs(x, y,&mut x_seen, &mut y_seen, &mut yx, 0, n);
            yx[y][x] = 'x';
            println!("{} {}", y, x);
        }
    }
}

fn dfs(x: usize, y: usize, x_seen: &mut Vec<bool>, y_seen: &mut Vec<bool>, yx: &mut Vec<Vec<char>>, depth: usize, n: usize) {
    if depth == n - 1 {
        for i in 0..n {
            // println!("{:?}", yx[i]);
            for j in 0..n {
                print!("{} ", yx[i][j]);
            }
            println!("");
        }
        // println!();
        return;
    }
    x_seen[x] = true;
    y_seen[y] = true;
    for y_next in 0..n {
        if y_seen[y_next] {continue}
        for x_next in 0..n {
            if x_seen[x_next] {continue}
            yx[y_next][x_next] = 'o';
            dfs(x_next, y_next, x_seen, y_seen, yx, depth + 1, n);
            x_seen[x_next] = false;
            y_seen[y_next] = false;
            yx[y_next][x_next] = 'x';
        }
    }
    x_seen[x] = false;
    y_seen[y] = false;

}