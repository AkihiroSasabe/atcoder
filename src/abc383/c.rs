#![allow(dead_code, unused_imports)]
use proconio::{input, marker::{Usize1, Isize1}};
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
use rand::Rng;
fn main() {
    input! {
        h: usize,
        w: usize,
        d: usize,
        s: [Chars; h]
    }
    // 10^3 ** 3 = 10^9

    let mut heap: BinaryHeap<(usize, usize, usize)> = BinaryHeap::new();
    let mut res = vec![vec![0; w]; h];
    let mut hum = vec![vec![0; w]; h];
    for y in 0..h {
        for x in 0..w {
            if s[y][x] == 'H' {
                heap.push((d, y, x));
                hum[y][x] = 1;
            }
        }
    }
    // 右、上、左、下
    let dir_x: Vec<isize> = vec![1, 0, -1, 0];
    let dir_y: Vec<isize> = vec![0, -1, 0, 1];
    
    while heap.len() != 0 {
        let (d0, y, x) = heap.pop().unwrap();
        // println!("d0 = {}, y = {}, x = {:?}", d0, y, x);
        hum[y][x] = 1;
        if res[y][x] < d0 {
            res[y][x] = d0;
        }
        else {
            continue
        }
        if d0 == 0 {continue}

        for i in 0..dir_y.len() {
            let ny = dir_y[i] + y as isize;
            let nx = dir_x[i] + x as isize;
            if ny < 0 || s.len() as isize <= ny || nx < 0 || s[0].len() as isize <= nx {continue}
            let ny = ny as usize;
            let nx = nx as usize;
            if s[ny][nx] != '.' {continue}

            if res[ny][nx] <= d0 - 1 {
                heap.push((d0 -1, ny, nx));
            }
            // res[ny][nx] = max(res[ny][nx], d0-1);
        }
    }

    let mut ans = 0;
    for y in 0..h {
        for x in 0..w {{
            ans += hum[y][x];
        }}
    }
    // hum.print_2d_array();
    println!("{}", ans);

}

// デバッグ用に2次元配列をprintするトレイト
pub trait Print2DArray {
    fn print_2d_array(&self);
    fn print_2d_array_with_name(&self, name: &str);
    fn print_2d_array_transposed(&self);
    fn print_2d_array_transposed_with_name(&self, name: &str);
}
impl<T: std::fmt::Debug> Print2DArray for Vec<Vec<T>> {
    fn print_2d_array(&self) {
        for y in 0..self.len() {
            for x in 0..self[y].len() {
                print!("{:?} ", self[y][x]);
            }
            println!("");
        }
    }
    fn print_2d_array_with_name(&self, name: &str) {
        println!("-=-=-=-= {} -=-=-=-=", name);
        self.print_2d_array();
        println!("-=-=-=-=-=-=-=-=");
    }
    fn print_2d_array_transposed(&self) {
        for x in 0..self[0].len() {
            print!("{:02}: ", x);
            for y in 0..self.len() {
                print!("{:?} ", self[y][x]);
            }
            println!("");
        }
    }
    fn print_2d_array_transposed_with_name(&self, name: &str) {
        println!("-=-=-=-= transposed {} -=-=-=-=", name);
        self.print_2d_array_transposed();
        println!("-=-=-=-=-=-=-=-=");
    }
}

// fn dfs(y: usize, x: usize, &s: Vec<Vec<char>>, res: &mut Vec<isize>) {

// }