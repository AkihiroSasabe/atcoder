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
    // 2024-08-28 20:15-21:00 (45 min)
    // 2024-08-29 12:49-12:55 (6 min)
    // 51 min
    input! {
        n: usize,
        k: usize,
    }
    let mut x = vec![];
    let mut y = vec![];
    let mut c = vec![];
    for i in 0..n {
        input! {
            xi: usize,
            yi: usize,
            ci: char,
        }
        x.push(xi);
        y.push(yi);
        c.push(ci);
    }

    // 2kで割った余りの位置を求める
    // usize だと、MLRするので、u32にする必要がある。
    let mut cum_b: Vec<Vec<u32>> = vec![vec![0; 4*k]; 4*k];
    let mut cum_w = vec![vec![0; 4*k]; 4*k];
    for i in 0..n {
        let xri = x[i] % (2*k);
        let yri = y[i] % (2*k);

        if c[i] == 'B' {
            cum_b[yri][xri] += 1;

            cum_b[yri+2*k][xri] += 1;
            cum_b[yri][xri+2*k] += 1;
            cum_b[yri+2*k][xri+2*k] += 1;

        }
        else {
            cum_w[yri][xri] += 1;

            cum_w[yri+2*k][xri] += 1;
            cum_w[yri][xri+2*k] += 1;
            cum_w[yri+2*k][xri+2*k] += 1;
        }
    }

    let cum_b = get_2d_cumulative_sum(&cum_b);
    let cum_w = get_2d_cumulative_sum(&cum_w);

    // let mut ans: usize = cum_w[2*k-1][2*k-1] + cum_w[k-1][k-1] - cum_w[2*k-1][k-1] - cum_w[k-1][2*k-1];
    let mut ans = 0;

    for yi in 0..2*k {
        for xi in 0..2*k {
            // 白の開始位置

            let w0 = get_sum_from_2d_cum(&cum_w, yi, xi, yi + k - 1, xi + k - 1);
            let w1 = get_sum_from_2d_cum(&cum_w, yi+k, xi+k,  yi+2*k-1, xi+2*k-1);

            let b0 = get_sum_from_2d_cum(&cum_b, yi + k, xi, yi + 2*k - 1, xi + k - 1);
            let b1 = get_sum_from_2d_cum(&cum_b, yi, xi + k, yi + k - 1, xi + 2*k-1);
            let cand = w0 + w1 + b0 + b1;
            ans = max(ans, cand);
        }
    }
    println!("{}", ans);

}

/// 2次元累積和から、(ys, xs) ~ (yt, xt) で囲まれた矩形領域の部分の和を求める関数
fn get_sum_from_2d_cum<T>(cum_2d: &Vec<Vec<T>>, ys: usize, xs: usize, yt: usize, xt: usize) -> T 
    where T: std::ops::Add<Output = T> + std::ops::Sub<Output = T> + Copy,
{
    if xs == 0 && ys == 0{
        let sum = cum_2d[yt][xt];
        return sum
    }
    else if xs == 0 && ys != 0 {
        let sum = cum_2d[yt][xt] - cum_2d[ys-1][xt];
        return sum
    }
    else if xs != 0 && ys == 0 {
        let sum = cum_2d[yt][xt] - cum_2d[yt][xs-1];
        return sum
    }
    else {
        // xs != 0 && ys != 0
        let sum = cum_2d[yt][xt] + cum_2d[ys-1][xs-1] - cum_2d[yt][xs-1] - cum_2d[ys-1][xt];
        return sum
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