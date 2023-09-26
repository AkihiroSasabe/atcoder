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

// マクロの実験 (これは作り途中)
macro_rules! custom_dbgarray {
    () => {
        eprintln!("[{}:{}]", file!(), line!());
    };
    ($val:expr $(,)?) => {
        match $val {
            tmp => {
                eprintln!("[{}:{}] {} = {:#?}", file!(), line!(), stringify!($val), &tmp);
                tmp
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($($crate::custom_dbgarray!($val)),+,)
    };
}

fn main() {
    // 2023-09-22 20:48 - 20:55
    // 2023-09-23 10:27 - 12:02 分からなかった。溶けなかった。
    // 12:03 これから解説acする。nyaanさんとkyopro_friendsさんの解法で解く
    input! {
        h: usize,
        w: usize,
        n: usize,
        ab: [(usize, usize); n]
    }
    // solve_tle(h, w, n, ab);
    solve(h, w, n, ab);

}

fn solve(h: usize, w: usize, n: usize, ab: Vec<(usize, usize)>) {
    // 穴開き部分のマスク画像を取得
    let mut yx = vec![];
    let mut xy = vec![];
    let mut hole_mask = vec![vec![0; w]; h];
    let mut hole_y = vec![];
    let mut hole_x = vec![];
    let mut min_hole_x = w;
    let mut min_hole_y = h;
    for i in 0..n {
        hole_y.push(ab[i].0 - 1);
        hole_x.push(ab[i].1 - 1);
        min_hole_y = min(min_hole_y, ab[i].0 - 1);
        min_hole_x = min(min_hole_x, ab[i].1 - 1);
        hole_mask[ab[i].0 - 1][ab[i].1 - 1] = 1;
        yx.push(vec![ab[i].0 - 1, ab[i].1 - 1]);
        xy.push(vec![ab[i].1 - 1, ab[i].0 - 1]);
    }
    hole_y.push(h);
    hole_x.push(w);
    yx.push(vec![h, w]);
    xy.push(vec![w, h]);

    hole_y.sort();
    hole_x.sort();
    yx.sort();
    xy.sort();

    // dp[y][x] := (y, x)を左上としたとき、穴の無い正方形の辺の最大長さ
    let initial_num = 1_000_000_000;
    let mut dp = vec![vec![initial_num; w]; h];
    // 一番下の行の初期化
    for y in 0..h {
        if hole_mask[y][w-1] == 1 {
            dp[y][w-1] = 0;
        }
        else {
            dp[y][w-1] = 1;
        }
    }

    // 一番右の列の初期化
    for x in 0..w {
        if hole_mask[h-1][x] == 1 {
            dp[h-1][x] = 0;
        }
        else {
            dp[h-1][x] = 1;
        }
    }
    
    // dpの遷移。右下から探索していく
    for y in (0..h-1).rev() {
        for x in (0..w-1).rev() {
            if hole_mask[y][x] == 1 {
                dp[y][x] = 0;
            }
            dp[y][x] = min(dp[y][x], dp[y+1][x] + 1);
            dp[y][x] = min(dp[y][x], dp[y][x+1] + 1);
            dp[y][x] = min(dp[y][x], dp[y+1][x+1] + 1);
        }
    }
    let mut ans: usize = 0;
    for y in 0..h {
        for x in 0..w {
            ans += dp[y][x];
        }
    }

    // let xxx = 100;
    // custom_dbgarray!(xxx);
    // dp.print_2d_array_with_name("dp");
    // hole_mask.print_2d_array_with_name("hole_mask");
    println!("{}", ans);

}

// TLEしてしまう解き方
fn solve_tle(h: usize, w: usize, n: usize, ab: Vec<(usize, usize)>) {
    // 穴開き部分のマスク画像を取得
    let mut hole_mask = vec![vec![0; w]; h];
    let mut hole_y = vec![];
    let mut hole_x = vec![];
    for i in 0..n {
        hole_y.push(ab[i].0 - 1);
        hole_x.push(ab[i].1 - 1);
        hole_mask[ab[i].0 - 1][ab[i].1 - 1] = 1;
    }
    
    // 累積和
    // x方向
    let mut x_cum = vec![vec![0; w]; h];
    for y in 0..h {
        x_cum[y][0] = hole_mask[y][0];
    }
    for y in 0..h {
        for x in 1..w {
            x_cum[y][x] = x_cum[y][x-1] + hole_mask[y][x];
        }
    }

    // xy方向の累積和
    let mut xy_cum = vec![vec![0; w]; h];
    for x in 0..w {
        xy_cum[0][x] = x_cum[0][x];
    }
    for y in 1..h {
        for x in 0..w {
            xy_cum[y][x] += xy_cum[y-1][x] + x_cum[y][x];
        }
    }    
    // println!("hole_mask");
    // print_array(&hole_mask);
    // println!("x_cum");
    // print_array(&x_cum);
    // println!("xy_cum");
    // print_array(&xy_cum);

    let mut ans = 0;
    // O(1^2 + 2^2 + 3^2 + ... + 3000^2 = 1/6* 3000 * (3000+1) * (2*3000+1) >= 10^9: TLE)
    for kernel in 1..min(h,w)+1 {
        println!("kernel = {:?}, ans = {}", kernel, ans);
        for y in 0..(1+h-kernel) {
            for x in 0..(1+w-kernel) {
                // [y:y+kernel][x:x+kernel]が0であれば足せる
                // let mut hoke_num = 0;
                let num_1 = xy_cum[y+kernel-1][x+kernel-1];
                
                let num_2= match y > 0 {
                    true => xy_cum[y-1][x+kernel-1],
                    false => 0
                };
                let num_3 = match x > 0 {
                    true => xy_cum[y+kernel-1][x-1],
                    false => 0
                };
                
                let num_4 = match y > 0 && x > 0 {
                    true => xy_cum[y-1][x-1],
                    false => 0
                };
                let hole_num = num_1 + num_4 - num_2 - num_3;
                if hole_num == 0 {
                    ans += 1;
                }
            }
        }
    }
    println!("{}", ans);    
}

// デバッグ用に2次元配列をprintするトレイト
pub trait Print2DArray {
    fn print_2d_array(&self);
    fn print_2d_array_with_name(&self, name: &str);
}
impl<T: std::fmt::Display> Print2DArray for Vec<Vec<T>> {
    fn print_2d_array(&self) {
        for i in 0..self.len() {
            for j in 0..self[i].len() {
                print!("{} ", self[i][j]);
            }
            println!("");
        }
    }
    fn print_2d_array_with_name(&self, name: &str) {
        println!("-=-=-=-= {} -=-=-=-=", name);
        self.print_2d_array();
        println!("-=-=-=-=-=-=-=-=");
    }
}