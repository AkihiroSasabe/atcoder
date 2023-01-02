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
    // 23-01-02
    // 17:00-17:24 => 24min
    input! {
        n: usize,
        s: [Chars; n]
    }
    let mut flag = false;
    for y in 0..n {
        for x in 0..n {
            if horizontal_check(&s, y, x) || vertical_check(&s, y, x) || down_cross_check(&s, y, x) || upper_cross_check(&s, y, x) {
                flag = true;
            }
        }
    }
    if flag {
        println!("Yes");
    }
    else {
        println!("No");
    }
}

fn upper_cross_check(s: &Vec<Vec<char>>, y: usize, x: usize) -> bool {
    let mut count = 0;
    for i in 0..6 {
        if y < i {
            return false
        }
        if does_index_exceed(s, y-i, x+i) {
            return false
        }
        if s[y-i][x+i] == '#' {
            count += 1;
        }
    }
    return count >= 4
}

fn down_cross_check(s: &Vec<Vec<char>>, y: usize, x: usize) -> bool {
    let mut count = 0;
    for i in 0..6 {
        if does_index_exceed(s, y+i, x+i) {
            return false
        }
        if s[y+i][x+i] == '#' {
            count += 1;
        }
    }
    return count >= 4
}

fn horizontal_check(s: &Vec<Vec<char>>, y: usize, x: usize) -> bool {
    let mut count = 0;
    for i in x..x+6 {
        if does_index_exceed(s, y, i) {
            return false
        }
        if s[y][i] == '#' {
            count += 1;
        }
    }
    return count >= 4
}

fn vertical_check(s: &Vec<Vec<char>>, y: usize, x: usize) -> bool {
    let mut count = 0;
    for i in y..y+6 {
        if does_index_exceed(s, i, x) {
            return false
        }
        if s[i][x] == '#' {
            count += 1;
        }
    }
    return count >= 4
}

fn does_index_exceed(s: &Vec<Vec<char>>, y: usize, x: usize) -> bool {
    let n = s.len();
    return n <= x || n <= y
}