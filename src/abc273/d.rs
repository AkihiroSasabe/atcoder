use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    input! {
        h: usize,
        w: usize,
        mut rs: usize,
        mut cs: usize,
        n: usize,
    }
    rs -= 1;
    cs -= 1;
    // Vec<Vec<usize>> h, w <= 10^9なので、メモリ制限の1,024[MB]を超えてしまう可能性がある。
    // let mut row: Vec<Vec<usize>> = vec![vec![]; h];
    // let mut column: Vec<Vec<usize>> = vec![vec![]; w];
    // メモリ節約のために、HashMapを使う。
    let mut row: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut column: HashMap<usize, Vec<usize>> = HashMap::new();

    for i in 0..n {
        input! {
            r_i: usize,
            c_i: usize,
        }
        row.entry(r_i - 1).or_insert(vec![]).push(c_i - 1);
        column.entry(c_i - 1).or_insert(vec![]).push(r_i - 1);
        // row[(r_i - 1)].push(c_i - 1);
        // column[c_i - 1].push(r_i - 1);
    }
    input! {
        q: usize
    }
    let mut d = vec![];
    let mut l = vec![];
    for i in 0..q {
        input! {
            d_i: char,
            l_i: isize,
        }
        let mut dd_i = 0;
        if d_i == 'L' {
            dd_i = 0;
        }
        else if d_i == 'R' {
            dd_i = 1;
        }
        else if d_i == 'U' {
            dd_i = 2;
        }
        else {
            dd_i = 3;
        }
        
        d.push(dd_i);
        l.push(l_i);
    }


    // let mut unk: Vec<usize> = vec![];
    // println!("{:?}", unk);
    // unk.sort();
    // println!("{:?}", unk);


    // let mut tekito: Vec<usize> = vec![2];
    // let ind = tekito.upper_bound(&5);
    // println!("index; {}", ind); // 1

    for v in row.values_mut() {
        v.sort();
    }
    for v in column.values_mut() {
        v.sort();
    }

    let mut y = rs as isize;
    let mut x = cs as isize;
    let dir = vec![vec![0, -1], vec![0, 1], vec![-1, 0], vec![1, 0]];
    for i in 0..q {
        let dir_y = dir[d[i]][0];
        let dir_x = dir[d[i]][1];
        // dbg!(dir_y, dir_x);

        let mut next_y = y + dir_y * l[i];
        let mut next_x = x + dir_x * l[i];

        next_y = kabe_check(next_y, h);
        next_x = kabe_check(next_x, w);

        // 下に行く
        if dir_y > 0 {
            // 一番近い壁
            if let Some(v) = column.get(&(x as usize)) {
                let index = v.upper_bound(&(y as usize));
                if index != column[&(x as usize)].len() {
                    next_y = min(column[&(x as usize)][index] as isize - 1, next_y);
                }
            }
        }
        // 上に行く
        else if dir_y < 0 {
            // println!("column[x]: {:?}", column[x as usize]);
            if let Some(v) = column.get(&(x as usize)) {
                let mut index: usize = v.lower_bound(&(y as usize));
                if index != 0 {
                    index -= 1;
                    next_y = max(column[&(x as usize)][index] as isize + 1, next_y);
                }
            }
        }

        // 右に行く
        if dir_x > 0 {
            // 最寄りの壁
            if let Some(v) = row.get(&(y as usize)) {
                let index: usize = v.upper_bound(&(x as usize));
                if index != row[&(y as usize)].len() {
                    next_x = min(row[&(y as usize)][index] as isize - 1, next_x);
                }
            }
        }
        // 左に行く
        else if dir_x < 0 {
            if let Some(v) = row.get(&(y as usize)) {
                let mut index = v.lower_bound(&(x as usize));
                if index != 0 {
                    index -= 1;
                    next_x = max(row[&(y as usize)][index] as isize + 1, next_x);
                }
            }
        }
        println!("{} {}", next_y + 1, next_x + 1);
        y = next_y;
        x = next_x;
    }

}

fn kabe_check(mut x: isize, length: usize) -> isize {
    if x < 0 {
        x = 0;
    }
    else if x >= length as isize {
        x = length as isize - 1;
    }
    return x
}
