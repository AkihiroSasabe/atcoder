#![allow(dead_code, unused_imports)]
use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h]
    }
    let mut seen = vec![vec![false; w]; h];
    let mut sx = 0;
    let mut sy = 0;
    for i in 0..h {
        for j in 0..w {
            if c[i][j] == 'S' {
                sy = i;
                sx = j;
            }
        }
    }

    // 左、上、右、下
    let dir = vec![vec![0, -1], vec![-1, 0], vec![0, 1], vec![1, 0]];
    let mut flag = false;
    dfs(sy, sx, &c, &mut seen, &dir, h, w, 0, sy, sx, &mut flag);
    if flag {
        println!("Yes");
    }
    else {
        println!("No");
    }

}

fn dfs(y: usize, x: usize, c: &Vec<Vec<char>>, seen: &mut Vec<Vec<bool>>, dir: &Vec<Vec<isize>>, h: usize, w: usize, depth: usize, sy: usize, sx: usize, flag: &mut bool) {
    seen[y][x] = true;
    // println!("y, x: {}, {}", y, x);
    for i in 0..dir.len() {
        let next_y = y as isize + dir[i][0];
        let next_x = x as isize + dir[i][1];
        if next_y < 0 || next_y as usize >= h || next_x < 0 || next_x as usize >= w {
            continue
        }
        let next_y = next_y as usize;
        let next_x = next_x as usize;

        if c[next_y][next_x] == '#' {continue}
        if seen[next_y][next_x] && !(sy == next_y && sx == next_x) {continue}
        if (sy == next_y && sx == next_x) && depth == 1 {continue}
        if (sy == next_y && sx == next_x) && depth != 1 {
            *flag = true;
            break
        }
        dfs(next_y, next_x, c, seen, dir, h, w, depth+1, sy, sx, flag);
    }
}