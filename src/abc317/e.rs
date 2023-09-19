#![allow(dead_code, unused_imports)]
use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::BTreeMap;
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::collections::{HashSet, BTreeSet};
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    // 2023-09-19 tue. 19:34-20:36 58min
    input! {
        h: usize,
        w: usize,
        mut a: [Chars; h]
    }

    let mut b = vec![vec![0; w]; h]; // 0は自由に侵入可能なマス。1は侵入不可なマス

    let mut start_y = 0;
    let mut start_x = 0;
    let mut goal_y = 0;
    let mut goal_x = 0;

    for i in 0..h {
        let mut ban_mode = false;
        for j in 0..w {
            if a[i][j] == '>' {
                b[i][j] = 1;
                ban_mode = true;
            }
            else if a[i][j] == '.' {
                if ban_mode {
                    b[i][j] = 1;
                }
            }
            else if a[i][j] == 'S' {
                start_y = i;
                start_x = j;
            }
            else if a[i][j] == 'G' {
                goal_y = i;
                goal_x = j;
            }
            else {
                b[i][j] = 1;
                ban_mode =false;
            }
        }

        let mut ban_mode = false;
        for j in (0..w).rev() {
            if a[i][j] == '<' {
                b[i][j] = 1;
                ban_mode = true;
            }
            else if a[i][j] == '.' {
                if ban_mode {
                    b[i][j] = 1;
                }
            }
            else if a[i][j] == 'S' || a[i][j] == 'G' {
                continue
            }
            else {
                b[i][j] = 1;
                ban_mode =false;
            }
        }
    }

    for x in 0..w {
        let mut ban_mode = false;
        for y in 0..h {
            if a[y][x] == 'v' {
                b[y][x] = 1;
                ban_mode = true;
            }
            else if a[y][x] == '.' {
                if ban_mode {
                    b[y][x] = 1;
                }
            }
            else if a[y][x] == 'S' || a[y][x] == 'G' {
                continue
            }
            else {
                b[y][x] = 1;
                ban_mode =false;
            }
        }
        let mut ban_mode = false;
        for y in (0..h).rev() {
            if a[y][x] == '^' {
                b[y][x] = 1;
                ban_mode = true;
            }
            else if a[y][x] == '.' {
                if ban_mode {
                    b[y][x] = 1;
                }
            }
            else if a[y][x] == 'S' || a[y][x] == 'G' {
                continue
            }
            else {
                b[y][x] = 1;
                ban_mode =false;
            }
        }
    }

    let initial_distance = 1_000_000_000;
    let distance = bfs(start_y, start_x, &b, goal_y, goal_x, initial_distance);
    if distance[goal_y][goal_x] == initial_distance {
        println!("-1");
    }
    else {
        println!("{}", distance[goal_y][goal_x]);
    }

}

fn bfs(start_y: usize, start_x: usize, b: &Vec<Vec<usize>>, goal_y: usize, goal_x: usize, initial_distance: usize) -> Vec<Vec<usize>> {
    let h = b.len();
    let w = b[0].len();
    let mut distance = vec![vec![initial_distance; w]; h];
    distance[start_y][start_x] = 0;

    // 右、上、左、下
    let dir_x: Vec<isize> = vec![1, 0, -1, 0];
    let dir_y: Vec<isize> = vec![0, -1, 0, 1];

    let mut queue = VecDeque::new();
    queue.push_back((start_y, start_x));
    while queue.len() != 0 {
        let (y, x) = queue.pop_front().unwrap();
        for i in 0..dir_y.len() {
            let next_y = dir_y[i] + y as isize;
            let next_x = dir_x[i] + x as isize;
            // 場外ならスキップ
            if next_y < 0 || b.len() as isize <= next_y || next_x < 0 || b[0].len() as isize <= next_x {continue}
            // 訪問済みならスキップ
            if distance[next_y as usize][next_x as usize] != initial_distance {continue}
            // 侵入不可ならスキップ
            if b[next_y as usize][next_x as usize] == 1 {continue}
            distance[next_y as usize][next_x as usize] = distance[y][x] + 1;
            queue.push_back((next_y as usize, next_x as usize));
        }

    }

    return distance

}
