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
    // 2023-01-23 12:07-12:34 (27m)
    // 2023-01-23 21:10-22:19 (1h10m)
    // 2023-01-24 12:00-12:26 (26m)
    // 2h3m
    input! {
        n: usize,
        s: [Chars; n],
        t_0: [Chars; n],
    }
    let mut t_1 = rotate(&t_0);
    let mut t_2 = rotate(&t_1);
    let mut t_3 = rotate(&t_2);

    let hash_s = get_cut_length(&s);
    let hash_t0 = get_cut_length(&t_0);
    let hash_t1 = get_cut_length(&t_1);
    let hash_t2 = get_cut_length(&t_2);
    let hash_t3 = get_cut_length(&t_3);

    // println!("{:?}", hash_s);
    // println!("{:?}", hash_t0);
    // println!("{:?}", hash_t1);
    // println!("{:?}", hash_t2);
    // println!("{:?}", hash_t3);

    // print_grid(&s);
    // print_grid(&t_0);
    // print_grid(&t_1);
    // print_grid(&t_2);
    // print_grid(&t_3);

    if judge(&s, &t_0, &hash_s, &hash_t0) || judge(&s, &t_1, &hash_s, &hash_t1) || judge(&s, &t_2, &hash_s, &hash_t2) || judge(&s, &t_3, &hash_s, &hash_t3){
        println!("Yes");
    }
    else {
        println!("No");
    }
    
}

fn print_grid(grid: &Vec<Vec<char>>) {
    println!("");
    let n = grid.len();
    for i in 0..n {
        for j in 0..n {
            print!("{}", grid[i][j]);
        }
        println!("");
    }
}

fn judge(grid: &Vec<Vec<char>>, grid2: &Vec<Vec<char>>, hash: &HashMap<char, usize>, hash2: &HashMap<char, usize>) -> bool {
    let n = grid.len();

    let d = *hash.get(&'d').unwrap();
    let u = *hash.get(&'u').unwrap();
    let r = *hash.get(&'r').unwrap();
    let l = *hash.get(&'l').unwrap();

    let d2= *hash2.get(&'d').unwrap();
    let u2= *hash2.get(&'u').unwrap();
    let r2= *hash2.get(&'r').unwrap();
    let l2= *hash2.get(&'l').unwrap();

    if !(d+u == d2+u2 && r+l == r2+l2) {
        // println!("Not match cut length");
        return false
    }
    for i in 0..n-(d+u) {
        for j in 0..n-(l+r) {
            let y = i + d;
            let x = j + r;
            
            let y2= i + d2;
            let x2 = j + r2;

            if grid[y][x] != grid2[y2][x2] {
                // println!("Not match pixel value");
                return false
            }
        }
    }

    return true

}

// 上下左右の空白辺を何マス分切り取れるか?
fn get_cut_length(grid: &Vec<Vec<char>>) -> HashMap<char, usize> {
    let n = grid.len();
    let mut down = 0;
    let mut up = 0;
    let mut left = 0;
    let mut right = 0;

    // 上辺を何マス切れるか?
    for y in 0..n {
        let mut flag = true; 
        for x in 0..n {
            if grid[y][x] == '#' {
                flag = false;
                break;
            }
        }
        if !flag {break}
        down = y+1;
    }

    // 下辺を何マス切れるか?
    for yy in 0..n {
        let y = n-1-yy;
        let mut flag = true; 
        for x in 0..n {
            if grid[y][x] == '#' {
                flag = false;
                break;
            }
        }
        if !flag {break}
        up = yy+1;
    }

    // 左辺を何マス切れるか?
    for x in 0..n {
        let mut flag = true; 
        for y in 0..n {
            if grid[y][x] == '#' {
                flag = false;
                break;
            }
        }
        if !flag {break}
        right = x+1;
    }
    
    // 右辺を何マス切れるか?
    for xx in 0..n {
        let x = n-1-xx;
        let mut flag = true; 
        for y in 0..n {
            if grid[y][x] == '#' {
                flag = false;
                break;
            }
        }
        if !flag {break}
        left = xx+1;
    }
    let mut hash = HashMap::new();
    hash.insert('d', down);
    hash.insert('u', up);
    hash.insert('r', right);
    hash.insert('l', left);

    return hash
}



fn rotate(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let n = grid.len();
    let mut grid2 = vec![vec!['.';n];n];
    for y in 0..n {
        for x in 0..n {
            grid2[x][n-1-y] = grid[y][x];
        }
    }
    return grid2
}