#![allow(dead_code, unused_imports)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::Ordering;
use std::cmp::{max, min};
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    input! {
        n: usize,
        mut x: isize,
        mut y: isize,
        a: [usize; n]
    }
    // Aを移動方向ごとに分離
    let mut x_vec = vec![];
    let mut y_vec = vec![];
    for i in 1..n {
        // indexが偶数のときはx方向に移動
        if i % 2 == 0 {
            x_vec.push(a[i]);
        // indexが奇数のときはy方向に移動
        } else {
            y_vec.push(a[i]);
        }
    }

    // dp[i][x'] := i番目までの配列を使うとき、位置x=x'-10_000に到達可否 (bool)
    // 定義域を正で取り扱えるように座標変換: (x,y)->(x',y')
    // -10_000 <= x <= +10_000
    // <=> 0 <= x' <= +20_001
    let mut dp_x = vec![vec![false; 20_001]; x_vec.len()];
    let mut dp_y = vec![vec![false; 20_001]; y_vec.len()];
    // 初期化
    dp_x[0][10_000 + a[0] + x_vec[0]] = true;
    dp_x[0][10_000 + a[0] - x_vec[0]] = true;
    dp_y[0][10_000 + y_vec[0]] = true;
    dp_y[0][10_000 - y_vec[0]] = true;

    for i in 1..x_vec.len() {
        for p in 0..20_001 {
            if x_vec[i] <= p {
                dp_x[i][p - x_vec[i]] = (dp_x[i][p - x_vec[i]] || dp_x[i - 1][p]);
            }
            if p + x_vec[i] < 20_001 {
                dp_x[i][p + x_vec[i]] = (dp_x[i][p + x_vec[i]] || dp_x[i - 1][p]);
            }
        }
    }
    for i in 1..y_vec.len() {
        for p in 0..20_001 {
            if y_vec[i] <= p {
                dp_y[i][p - y_vec[i]] = (dp_y[i][p - y_vec[i]] || dp_y[i - 1][p]);
            }
            if p + y_vec[i] < 20_001 {
                dp_y[i][p + y_vec[i]] = (dp_y[i][p + y_vec[i]] || dp_y[i - 1][p]);
            }
        }
    }
    if dp_x[x_vec.len() - 1][(x + 10_000) as usize] && dp_y[y_vec.len() - 1][(y + 10_000) as usize]
    {
        println!("Yes");
    } else {
        println!("No");
    }
}
