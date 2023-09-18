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
        c: [[usize; 3]; 3]
    }
    let mut c_1d: Vec<usize> = vec![];
    for i in 0..3 {
        for j in 0..3 {
            c_1d.push(c[i][j]);
        }
    }

    // 各マスの数を知る順番は、9! = 362,880通り < O(4*10^5)
    // 1つのパターンにつき、ガッカリするかのチェックに 9マス分 x ((4列：縦・横・右斜・左斜) x 2マス) = 72回 < O(100)
    // よってO(4*10^7)なので全探索しても間に合う
    let mut count = 0; // ガッカリしないで全部チェックできた回数
    let num_whole_event = 362880.0; // 全事象の個数=9!
    for perm in (0..9).permutations(9) {
        let mut array: Vec<Vec<usize>>  = vec![vec![0; 3]; 3];
        let mut is_gakkari = false;
        for i in perm {
            is_gakkari = gakkari_check(i, &array);
            if is_gakkari {break}
            array[i/3][i%3] = c_1d[i];
        }
        if is_gakkari {continue}
        count += 1;
    }
    let ans: f64 = count as f64 / num_whole_event;
    println!("{}", ans);
}

fn gakkari_check(n: usize, array: &Vec<Vec<usize>>) -> bool {
    let is_gakkari = false;

    let y = n / 3;
    let x = n % 3;
    // 縦|のチェック
    let y1 = (y + 1) % 3;
    let y2 = (y + 2) % 3;
    if array[y1][x] != 0 && array[y1][x] == array[y2][x] {
        return true
    } 
    
    // 横-のチェック
    let x1 = (x + 1) % 3;
    let x2 = (x + 2) % 3;
    if array[y][x1] != 0 && array[y][x1] == array[y][x2] {
        return true
    }

    // 左斜め\のチェック
    if n == 0 || n == 4 || n == 8 {
        if array[y1][x1] != 0 && array[y1][x1] == array[y2][x2] {
            return true
        }
    }

    // 右斜め/のチェック
    if n == 2 || n == 4 || n == 6 {
        if array[y1][x2] != 0 && array[y1][x2] == array[y2][x1] {
            return true
        }
    }

    return is_gakkari
}