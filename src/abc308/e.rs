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
use std::collections::HashSet;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    // 2023-09-26 20:44-22:12 (1h28m)
    input! {
        n: usize,
        a: [usize; n],
        s: Chars
    }
    // print!("     ");
    // for i in 0..n {
    //     print!("{} ", a[i]);
    // }
    // println!("");
    // print!("     ");
    // for i in 0..n {
    //     print!("{} ", s[i]);
    // }
    // println!("");


    // x_cum[i][j] := s[i,n]がXでかつ、a[i,n]がjである個数
    let mut x_cum = vec![vec![0; 3]; n];

    if s[n-1] == 'X' {
        match a[n-1] {
            0 => x_cum[n-1][0] = 1,
            1 => x_cum[n-1][1] = 1,
            2 => x_cum[n-1][2] = 1,
            _ => ()
        }
    }
    for i in (0..n-1).rev() {
        x_cum[i][0] = x_cum[i+1][0];
        x_cum[i][1] = x_cum[i+1][1];
        x_cum[i][2] = x_cum[i+1][2];
        if s[i] != 'X' {continue}
        match a[i] {
            0 => x_cum[i][0] += 1,
            1 => x_cum[i][1] += 1,
            2 => x_cum[i][2] += 1,
            _ => ()
        }
    }

    // ex_cum[i][bit] := 区間[i,n]で'EX'で、そのmexがbitとなる個数
    let mut ex_cum = vec![vec![0; 1 << 3]; n];
    for i in (0..n-1).rev() {
        for ex_bit in 0..1<<3 {
            ex_cum[i][ex_bit] = ex_cum[i+1][ex_bit];
        } 

        if s[i] != 'E' {continue}
        for j in 0..3 {
            let bit = (1 << j) | (1 << a[i]);
            // println!("bit={:03b}, i={}, j={}, a[i]={}", bit, i, j, a[i]);
            ex_cum[i][bit] += x_cum[i+1][j];
        }
    }

    // mex_cum[i][bit] := 区間[i,n]で'MEX'で、そのmexがbitとなる個数
    let mut mex_cum = vec![vec![0; 1 << 3]; n];
    for i in (0..n-2).rev() {
        for mex_bit in 0..1<<3 {
            mex_cum[i][mex_bit] = mex_cum[i+1][mex_bit];
        }
        if s[i] != 'M' {continue}
        for ex_bit in 0..(1<<3) {
            let mex_bit = ex_bit | (1 << a[i]);
            mex_cum[i][mex_bit] += ex_cum[i+1][ex_bit];
        }
    }

    let mut ans = 0;
    for mex_bit in 0..(1<<3) {
        let mex = get_least_0_bit(mex_bit);
        ans += mex * mex_cum[0][mex_bit];
    }
    // x_cum.print_2d_array_transposed_with_name("x_cum");
    // ex_cum.print_2d_array_transposed_with_name("ex_cum");
    // mex_cum.print_2d_array_transposed_with_name("mex_cum");
    println!("{}", ans);


}


fn get_least_0_bit(bit: usize) -> usize {
    let mut ans = 0;
    loop {
        if bit & (1 << ans) == 0 {
            return ans
        }
        else {
            ans += 1;
        }
    }
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