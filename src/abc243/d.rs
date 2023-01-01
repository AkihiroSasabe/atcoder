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
    // 23-01-01 17:25-18:00
    // 23-01-01 19:07-
    input! {
        n: usize,
        x: usize,
        mut s: Chars,
    }

    // 【ケース1】下ってから上がる => 省略できる
    // ex1. LLLLUUU = L(LLLUUU) = L
    // ex2. LLULLU = L(LULLU) = LL
    // 【ケース2】上がってから下る => 省略できない
    // ex3. URR
    // ケース1で省略すれば、最後にいる頂点よりも下か、最初にいる頂点よりも下にいくことはない。

    // down > 0 で下がっている途中であることを表現
    let mut down: isize = 0;
    // d_indexに下がっているときに引いたdのインデックスを格納。Uを引いたら、相殺させる
    let mut d_index = vec![];
    for i in 0..s.len() {
        if s[i] == 'U' {
            if down > 0 {
                down -= 1;
                if let Some(d_i) = d_index.pop() {
                    s[d_i] = 'N';
                    s[i] = 'N';
                }
            }
        }
        else {
            down += 1;
            d_index.push(i);
        }
        if down < 0 {
            d_index = vec![];
            down = 0;
        }
    }

    // 省略された文字列でシミュレーション
    let mut now = x;
    for i in 0..s.len() {
        if s[i] == 'U' {
            if now == 1 {continue}
            now /= 2;
        }
        else if s[i] == 'R' {
            now  = now * 2 + 1;
        }
        else if s[i] == 'L' {
            now = now * 2;
        }
    }
    println!("{}", now);

}

// log_base(x)を求める
fn log(base: usize, mut x: usize) -> usize {
    let mut exp = 0;
    while base <= x {
        x = x / base;
        exp += 1;
    }
    return exp
}

