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
    input! {
        h: usize,
        w: usize,
        s: [Chars; h]
    }
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == 's' {
                if search(i, j, &s) {
                    return;
                }
            }
        }
    }
}

fn search(y: usize, x: usize, s: &Vec<Vec<char>>) -> bool {

    // 右、右上、上、左上、左、左下、下、右下
    let dy_list: Vec<isize> = vec![0, -1, -1, -1, 0, 1, 1, 1];
    let dx_list: Vec<isize> = vec![1, 1, 0, -1, -1, -1, 0, 1];
    let snuke = vec!['s', 'n', 'u', 'k', 'e'];

    for direct in 0..dy_list.len() {
        let mut dir_flag = true;
        for progress in 1..snuke.len() {
            let next_y = y as isize + dy_list[direct] * progress as isize;
            let next_x = x as isize + dx_list[direct] * progress as isize;
            if next_y < 0 || next_x < 0 || next_y >= s.len() as isize || next_x >= s[0].len() as isize {
                dir_flag = false;
                break
            }
            if snuke[progress as usize] != s[next_y as usize][next_x as usize] {
                dir_flag = false;
                break
            }
        }
        if dir_flag {
            for progress in 0..snuke.len() {
                let next_y = y as isize + dy_list[direct] * progress as isize + 1;
                let next_x = x as isize + dx_list[direct] * progress as isize + 1;
                println!("{} {}", next_y, next_x);
            }
            return true;
        }
    }

    return false
}