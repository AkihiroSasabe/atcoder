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
    input! {
        n: usize,
        s: [Chars; n]
    }
    let mut img = vec![vec![0; n]; n];
    for y in 0..n {
        for x in 0..n {
            if s[y][x] == 'o' {
                img[y][x] = 1;
            }
        }
    }

    let mut sum_y = vec![0; n];
    let mut sum_x = vec![0; n];
    for y in 0..n {
        for x in 0..n {
            sum_y[y] += img[y][x];
        }
    }
    for x in 0..n {
        for y in 0..n {
            sum_x[x] += img[y][x];
        }
    }
    // println!("sum_x = {:?}", sum_x);
    // println!("sum_y = {:?}", sum_y);


    let mut ans: usize = 0;
    for y in 0..n {
        for x in 0..n {
            if img[y][x] == 1 {
                ans += (sum_y[y] - 1) * (sum_x[x] - 1);
            }
        }
    }
    println!("{}", ans);


}

// デバッグ用に2次元配列をprintするトレイト
pub trait Print2DArray {
    fn print_2d_array(&self);
    fn print_2d_array_with_name(&self, name: &str);
    fn print_2d_array_transposed(&self);
    fn print_2d_array_transposed_with_name(&self, name: &str);
}
impl<T: std::fmt::Display> Print2DArray for Vec<Vec<T>> {
    fn print_2d_array(&self) {
        for y in 0..self.len() {
            for x in 0..self[y].len() {
                print!("{} ", self[y][x]);
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
                print!("{} ", self[y][x]);
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