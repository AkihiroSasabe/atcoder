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
        s: [Chars; h],
    }

    let snuke = vec!['s', 'n', 'u', 'k', 'e'];
    let mut flag = false;
    let mut seen = vec![vec![false; s[0].len()]; s.len()];
    // 右、下、左、上
    let dir_y = vec![0, 1, 0, -1];
    let dir_x = vec![1, 0, -1, 0];
    if s[0][0] == 's' {
        dfs(&s, 0, 0, &mut seen, &snuke, 0, &mut flag, &dir_y, &dir_x);
    }
    if flag {
        println!("Yes");
    }
    else {
        println!("No");
    }


}

fn dfs(s: &Vec<Vec<char>>, y: usize, x: usize, seen: &mut Vec<Vec<bool>>, snuke: &Vec<char>, current_index: usize, flag: &mut bool, dir_y: &Vec<isize>, dir_x: &Vec<isize>) {
    // println!("y={}, x={}", y, x);
    seen[y][x] = true;
    if y == s.len() -1 && x == s[0].len() - 1 {
        // println!("goal");
        *flag = true;
        return
    }
    

    for i in 0..4 {
        let next_y = dir_y[i] + y as isize;
        let next_x = dir_x[i] + x as isize;
        if next_y < 0 || next_y >= s.len() as isize || next_x < 0 || next_x >= s[0].len() as isize {
            continue
        }
        let next_y = next_y as usize;
        let next_x = next_x as usize;
        // println!("next_y={}, next_x={}", next_y, next_x);
        if seen[next_y][next_x] {
            continue
        }
        // println!("next_y={}, next_x={}", next_y, next_x);
        // println!("snuke[(current_index + 1) % snuke.len()]= {}, s[next_y][next_x])= {}", snuke[(current_index + 1) % snuke.len()], s[next_y][next_x]);
        if snuke[(current_index + 1) % snuke.len()] == s[next_y][next_x] {
            // println!("next_y={}, next_x={}", next_y, next_x);
            // println!("snuke.len()={}", snuke.len());
            dfs(s, next_y, next_x, seen, snuke, (current_index + 1) % snuke.len(), flag, dir_y, dir_x);
        }
        if *flag {return}
    }
    // seen[y][x] = false;

}