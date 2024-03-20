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
    // 2024-03-20 16:00-16:42 (42min)
    input! {
        h: usize,
        w: usize,
        a: [Chars; h],
    }
    let modulus: usize = 1_000_000_007;

    let mut dp = vec![vec![0; w]; h];
    dp[0][0] = 1;

    let mut cum_y = vec![0; w]; // cum_y[x] := x列目の縦の累積和
    let mut cum_x = vec![0; h]; // cum_x[y] := y行目の横の累積和
    let mut cum_s = vec![0; h + w - 1]; // cum_s[w - 1 + y - x] := 斜めの累積和

    cum_y[0] = 1;
    cum_x[0] = 1;
    cum_s[w-1] = 1;

    for y in 0..h {
        for x in 0..w {
            if y == 0 && x == 0 {continue}
            if a[y][x] == '#' {
                dp[y][x] = 0;
                cum_y[x] = 0;
                cum_x[y] = 0;
                cum_s[w-1+y-x] = 0;
            }
            else {
                dp[y][x] = cum_y[x] + cum_x[y] + cum_s[w - 1 + y - x];
                dp[y][x] %= modulus;
    
                cum_y[x] += dp[y][x];
                cum_x[y] += dp[y][x];
                cum_s[w-1+y-x] += dp[y][x];
    
                cum_y[x] %= modulus;
                cum_x[y] %= modulus;
                cum_s[w-1+y-x] %= modulus;
            }


            // let nx = x + 1;
            // let ny = y + 1;

            // if judge_move(ny, x, &a) {
            //     dp[ny][x] += dp[y][x];
            //     dp[ny][x] %= modulus;

            // }
            // if judge_move(y, nx, &a) {
            //     dp[y][nx] += dp[y][x];
            //     dp[y][nx] %= modulus;
            // }
            // if judge_move(ny, nx, &a) {
            //     dp[ny][nx] += dp[y][x];
            //     dp[ny][nx] %= modulus;
            // }
        }
    }
    // dp.print_2d_array();

    println!("{}", dp[h-1][w-1]);
}

fn judge_move(y: usize, x: usize, a: &Vec<Vec<char>>) -> bool {
    if y >= a.len() || x >= a[0].len() {
        return false
    }
    if a[y][x] == '#' {
        return false
    }
    return true

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