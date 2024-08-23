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
    // 2024-08-23 20:10-20:52 (42min)
    input! {
        a: usize, // 白
        b: usize, // 黒
    }
    // true: 白
    // false; 黒
    let mut na = 0;
    let mut nb = 0;
    // 全部白で初期化
    let mut ans = vec![vec![true; 100]; 100];
    na += 1;

    // 上半分は黒にする
    for y in 0..50 {
        for x in 0..100 {
            ans[y][x] = false;
        }
    }
    nb += 1;


    // 基本は、上に白ポチを埋め込み、下に黒ポチを埋め込む
    
    // 上に白ポチを埋め込む
    for y in 0..50 {
        if na == a {break}
        if y % 2 != 0 {continue}
        for x in 0..100 {
            if na == a {break}
            if x % 2 != 0 {continue}
            ans[y][x] = !ans[y][x];
            na += 1;
        }
    }

    for y in 50..100 {
        if nb == b {break}
        if y % 2 == 0 {continue}
        for x in 0..100 {
            if nb == b {break}
            if x % 2 != 0 {continue}
            ans[y][x] = !ans[y][x];
            nb += 1;
        }
    }

    println!("100 100");
    for y in 0..100 {
        for x in 0..100 {
            if ans[y][x] {
                print!(".");
            }
            else {
                print!("#");
            }
        }
        println!("");
    }

}