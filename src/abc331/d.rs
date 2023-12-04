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
    // 2023-12-02 21:26-22:40 (74min)
    // 2023-12-03 08:43-09:00 (17min)
    // total: 91min
    input! {
        n: usize,
        q: usize,
        p: [Chars; n],
        abcd: [[usize; 4]; q]
    }
    let mut img = vec![vec![0; n]; n];
    let mut sum = 0;
    for i in 0..n {
        for j in 0..n {
            if p[i][j] == 'B' {
                img[i][j] = 1;
                sum += 1;
            }
        }
    }

    // cum[i][j] := 原点から、(i,j)までの黒点の数
    let cum = get_2d_cumulative_sum(&img); 
    
    for i in 0..q {
        let a = abcd[i][0];
        let b = abcd[i][1];
        let c = abcd[i][2];
        let d = abcd[i][3];

        let area_big = get_global_box(&cum, c, d);

        let area_small = if a != 0 && b != 0 {
            get_global_box(&cum, a - 1, b - 1)
        }
        else {
            0
        };
        let area_right =  if a != 0 {
            get_global_box(&cum, a - 1, d)
        }
        else {
            0
        };
        let area_left = if b != 0 {
            get_global_box(&cum, c, b-1)
        }
        else {
            0
        };

        // println!("area_big = {:?}", area_big);
        // println!("area_small = {:?}", area_small);
        // println!("area_right = {:?}", area_right);
        // println!("area_left = {:?}", area_left);

        let ans = area_big + area_small - area_right - area_left;

        println!("{}", ans);
    }
}


/// 2次元累積和 cum を計算する関数
/// cum[y][x] := 原点(0,0)から右下(y,x)までの画素値の累積和
fn get_2d_cumulative_sum<T>(img: &Vec<Vec<T>>) -> Vec<Vec<T>> 
    where T: std::ops::Add<Output = T> + std::ops::Sub<Output = T> + Copy + Default
{
    let h = img.len();
    let w = img[0].len();
    let zero: T = Default::default();

    // 累積和の初期化
    let mut cum: Vec<Vec<T>> = vec![vec![zero; w]; h];
    cum[0][0] = img[0][0];

    // 1行目(y=0)の計算
    for x in 1..w {
        cum[0][x] = img[0][x] + cum[0][x-1];
    }
    // 2行目以降(y>0)の計算
    for y in 1..h {
        // 1列目(x=0)の計算
        cum[y][0] = img[y][0] + cum[y-1][0];
        // 2列目以降(x>0)の計算
        for x in 1..w {
            cum[y][x] = img[y][x] + cum[y-1][x] + cum[y][x-1] - cum[y-1][x-1];
        }
    }

    return cum
}


fn get_global_box(cum: &Vec<Vec<usize>>, y: usize, x: usize) -> usize {
    let n = cum.len();
    let y_num = y / n;
    let y_r = y % n;
    let x_num = x / n;
    let x_r = x % n;

    let one_box_sum = get_one_box(cum, 0, 0, n-1, n-1);
    let mut ans = one_box_sum * x_num * y_num;
    ans = ans + y_num * get_one_box(cum, 0, 0, n-1, x_r);
    ans = ans + x_num * get_one_box(cum, 0, 0, y_r, n-1);
    ans = ans + get_one_box(cum, 0, 0, y_r, x_r);

    return ans
}

fn get_one_box(cum: &Vec<Vec<usize>>, a: usize, b: usize, c: usize, d: usize) -> usize {
    let mut ans = cum[c][d];
    let diff0;
    let diff1;
    let diff2;
    if a == 0 && b == 0 {
        diff0 = 0;
        diff1 = 0;
        diff2 = 0;
    }
    else if a == 0 && b != 0 {
        diff0 = 0;
        diff1 = 0;
        diff2 = cum[0][b-1];
    }
    else if a != 0 && b == 0 {
        diff0 = 0;
        diff1 = cum[a-1][0];
        diff2 = 0;
    }
    else {
        diff0 = cum[a-1][b-1];
        diff1 = cum[a-1][d];
        diff2 = cum[c][b-1];
    }
    ans = ans + diff0 - diff1 - diff2;
    return ans
}