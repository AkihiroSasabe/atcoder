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
    // 2024-02-03 21:22-22:37 (1h15min)
    input! {
        n: usize,
        mut s: [Chars; n]
    }


    let mut persons = vec![];
    for y in 0..n {
        for x in 0..n {
            if s[y][x] == 'P' {
                persons.push(vec![y, x]);
                s[y][x] = '.';
            }
        }
    }
    let mut y0 = persons[0][0];
    let mut x0 = persons[0][1];

    let mut y1 = persons[1][0];
    let mut x1 = persons[1][1];


    let max_act: usize = 4000 * 4000;

    let mut dist = vec![vec![vec![vec![max_act; n]; n]; n]; n];
    dist[y0][x0][y1][x1] = 0;

    let dir_y = vec![-1,0,1,0];
    let dir_x = vec![0,1,0,-1];

    let mut queue = VecDeque::new();
    let v0 = vec![y0 as isize, x0 as isize, y1 as isize, x1 as isize];
    queue.push_back(v0);
    while queue.len() > 0 {
        let v = queue.pop_front().unwrap();
        let y0 = v[0];
        let x0 = v[1];
        let y1 = v[2];
        let x1 = v[3];

        for d in 0..4 {
            let dy = dir_y[d];
            let dx = dir_x[d];

            let ny0 = y0 + dy;
            let nx0 = x0 + dx;

            let ny1 = y1 + dy;
            let nx1 = x1 + dx;

            let mut ny0 = convert(ny0, &s);
            let mut nx0 = convert(nx0, &s);
            let mut ny1 = convert(ny1, &s);
            let mut nx1 = convert(nx1, &s);

            if s[ny0 as usize][nx0 as usize] == '#' {
                ny0 = y0;
                nx0 = x0;
            }
            if s[ny1 as usize][nx1 as usize] == '#' {
                ny1 = y1;
                nx1 = x1;
            }
            if dist[ny0 as usize][nx0 as usize][ny1 as usize][nx1 as usize] != max_act {continue}
            dist[ny0 as usize][nx0 as usize][ny1 as usize][nx1 as usize] = dist[y0 as usize][x0 as usize][y1 as usize][x1 as usize] + 1;
            queue.push_back(vec!{ny0, nx0, ny1, nx1});
        }
    }

    let mut ans = max_act;
    for y in 0..n {
        for x in 0..n {
            ans = min(ans, dist[y][x][y][x]);
        }
    }

    if ans == max_act {
        println!("-1");
        return
    }
    else {
        println!("{}", ans);
    }

}

fn convert(x: isize, s: &Vec<Vec<char>>) -> isize {
    let n = s.len();
    if x < 0 {
        return 0
    }
    else if x >= n as isize {
        return n as isize -1
    }
    else {
        return x
    }
}