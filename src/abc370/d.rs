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
use rand::Rng;
fn main() {
    input! {
        h: usize,
        w: usize,
        q: usize,
    }
    let mut r = vec![];
    let mut c = vec![];
    for i in 0..q {
        input! {
            ri: usize,
            ci: usize,
        }
        r.push(ri-1);
        c.push(ci-1);
    }

    let mut img: Vec<Vec<usize>> = vec![vec![1; w]; h];
    let mut ys = vec![BTreeSet::new(); h];
    let mut xs = vec![BTreeSet::new(); w];

    for y in 0..h {
        for x in 0..w {
            ys[y].insert(x);
            xs[x].insert(y);
        }
    }

    // println!("ys = {:?}", ys);
    // println!("xs = {:?}", xs);


    // println!("-----");
    // img.print_2d_array();
    // println!("-----");


    for i in 0..q {
        // println!("i = {:?} =======================", i);
        let ri = r[i];
        let ci = c[i];

        if img[ri][ci] == 1 {
            img[ri][ci] = 0;
            ys[ri].remove(&ci);
            xs[ci].remove(&ri);
            // println!("-----");
            // img.print_2d_array();
            // println!("ys = {:?}", ys);
            // println!("xs = {:?}", xs);
            // println!("-----");
            continue
        }


        // 右にいるか?
        if let Some(&xi) = ys[ri].range(ci+1..).next() {
            img[ri][xi] = 0;
            ys[ri].remove(&xi);
            xs[xi].remove(&ri);
            
        }

        // 左にいるか？
        if let Some(&xi) = ys[ri].range(..ci).rev().next() {
            img[ri][xi] = 0;
            ys[ri].remove(&xi);
            xs[xi].remove(&ri);
        }

        // 下にいるか?
        if let Some(&yi) = xs[ci].range(ri+1..).next() {
            img[yi][ci] = 0;
            xs[ci].remove(&yi);
            ys[yi].remove(&ci);

        }

        // 上にいるか？
        if let Some(&yi) = xs[ci].range(..ri).rev().next() {
            img[yi][ci] = 0;
            xs[ci].remove(&yi);
            ys[yi].remove(&ci);
        }

        // println!("-----");
        // img.print_2d_array();
        // println!("ys = {:?}", ys);
        // println!("xs = {:?}", xs);
        // println!("-----");
    }

    // img.print_2d_array();

    let mut ans = 0;
    for y in 0..h {
        for x in 0..w {
            ans += img[y][x];
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