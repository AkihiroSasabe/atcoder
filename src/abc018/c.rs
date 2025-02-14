#![allow(dead_code, unused_imports)]
use proconio::{input, marker::{Usize1, Isize1}};
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
use num::{BigUint, ToPrimitive};
use num_bigint::ToBigUint;
fn main() {
    // 2025-02-07 20:38-21:00 (22min)
    // 2025-02-13 19:18-22:47 (3h29min)
    // 3h51min
    input! {
        h: usize, // 3<= h <= 500   
        w: usize, // 3<= w <= 500
        k: usize, // 2<= k <= 500
        s: [Chars; h],
    }
    // 500^4 = 62_500_000_000 = 6.25 * 10^10
    // 累積和でいける

    let nh = 1 + h + w;
    let nw = 1 + h + w;
    let mut cum = vec![vec![0; nw]; nh];
    for y in 0..h {
        for x in 0..w {
            let (ny, nx) = shazo(y, x, w);
            if s[y][x] == 'x' {
                // 黒
                cum[ny][nx] = 1;
            }
        }
    }
    let cum = get_2d_cumulative_sum(&cum);

    let mut ans = 0;
    for y in 0..h {
        for x in 0..w {
            if k-1 <= x && x <= w - k && k-1 <= y && y <= h - k {
                let (nys, nxs) = shazo(y+1-k, x, w);
                let (nyt, nxt) = shazo(y+k-1, x, w);
                let sum = get_sum_from_2d_cum(&cum, nys, nxs, nyt, nxt);
                if sum == 0 {
                    ans += 1;
                }
            }
        }
    }
    println!("{}", ans);

}

/*
4 5 2
xoooo
oooox
ooooo
oxxoo

0 0 0 0 0 0 0 0 0 0 
0 0 0 0 0 1 0 0 0 0 
0 0 0 0 0 0 0 0 0 0 
0 0 0 0 0 0 0 0 0 0 
1 0 0 0 0 0 0 0 0 0 
0 0 0 0 0 1 0 0 0 0 
0 0 0 0 1 0 0 0 0 0 
0 0 0 0 0 0 0 0 0 0 
0 0 0 0 0 0 0 0 0 0 
0 0 0 0 0 0 0 0 0 0 

4 5 2
xoooo
ooooo
ooooo
ooooo

0 0 0 0 0 0 0 0 0 0 
0 0 0 0 0 0 0 0 0 0 
0 0 0 0 0 0 0 0 0 0 
0 0 0 0 0 0 0 0 0 0 
1 0 0 0 0 0 0 0 0 0 
0 0 0 0 0 0 0 0 0 0 
0 0 0 0 0 0 0 0 0 0 
0 0 0 0 0 0 0 0 0 0 
0 0 0 0 0 0 0 0 0 0 
0 0 0 0 0 0 0 0 0 0 

*/


fn shazo(y: usize, x: usize, w: usize) -> (usize, usize) {
    // 2次元map上の(y,x)を原点(z軸の正方向)周りに45度右回転させた座標系(ny,nx)に射影する関数
    // ただし、(ny,nx)のnyが負になるので、ny方向への射影はmapのwidth-1だけオフセットする。
    let nx = x + y;
    let ny = w - 1 - x + y;
    return (ny, nx)
/*
以下のように射影する行列Aを考える。
A: (x,y) -> (nx,ny)
A: (0,0) -> (0,0)
A: (1,1) -> (1,0)
A=[
    [1 1],
    [-1 1]
]
となる行列が良さそう。
つまり、
NX =  X+Y
NY = -X+Y

◆写像先の大きさ(NH, NW)を考える。
nx_min = 0 + 0 = 0
nx_max = xmax + ymax
ny_min = (-1)*xmax + 0 = -xmax
ny_max = (-1)*0 + ymax = ymax

よって、
NH = 1 + ny_max - ny_min = 1 + ymax + xmax = H + W - 1
NW = 1 + nx_max - nx_min = 1 + xmax + ymax = H + W - 1
となる。写像先は、写像元より大きい正方形となる。

◆最終的な写像
NYが負になる可能性があって嫌なので、常にオフセットをつけておきたい。
NX =  X+Y
NY = xmax-X+Y

// 実際に写像してみる
    println!("==x==");
    for y in 0..h {
        for x in 0..w {
            print!("{}", x);
        }
        println!("");
    }
    println!("==y==");
    for y in 0..h {
        for x in 0..w {
            print!("{}", y);
        }
        println!("");
    }
    println!("==nx==");
    for y in 0..h {
        for x in 0..w {
            let (ny, nx) = shazo(y, x, w);
            print!("{}", nx);
        }
        println!("");
    }
    println!("==ny==");
    for y in 0..h {
        for x in 0..w {
            let (ny, nx) = shazo(y, x, w);
            print!("{}", ny);
        }
        println!("");
    }
==x==
01234
01234
01234
01234

==y==
00000
11111
22222
33333

==nx==
01234
12345
23456
34567

==ny==
43210
54321
65432
76543
*/
}

// デバッグ用に2次元配列をprintするトレイト
pub trait Print2DArray {
    fn print_2d_array(&self);
    fn print_2d_array_with_name(&self, name: &str);
    fn print_2d_array_transposed(&self);
    fn print_2d_array_transposed_with_name(&self, name: &str);
}
impl<T: std::fmt::Debug> Print2DArray for Vec<Vec<T>> {
    fn print_2d_array(&self) {
        for y in 0..self.len() {
            for x in 0..self[y].len() {
                print!("{:?} ", self[y][x]);
            }
            println!("");
        }
    }
    fn print_2d_array_with_name(&self, name: &str) {
        println!("-=-=-=-= {} -=-=-=-=", name);
        self.print_2d_array();
        println!("-=-=-=-=-=-=-=-=");
    }
    fn print_2d_array_transposed(&self) {
        for x in 0..self[0].len() {
            print!("{:02}: ", x);
            for y in 0..self.len() {
                print!("{:?} ", self[y][x]);
            }
            println!("");
        }
    }
    fn print_2d_array_transposed_with_name(&self, name: &str) {
        println!("-=-=-=-= transposed {} -=-=-=-=", name);
        self.print_2d_array_transposed();
        println!("-=-=-=-=-=-=-=-=");
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

/// 2次元累積和から、(ys, xs) ~ (yt, xt) で囲まれた矩形領域の和を求める関数
fn get_sum_from_2d_cum<T>(cum_2d: &Vec<Vec<T>>, ys: usize, xs: usize, yt: usize, xt: usize) -> T 
    where T: std::ops::Add<Output = T> + std::ops::Sub<Output = T> + Copy,
{
    if xs == 0 && ys == 0 {
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

// ●○●○●○●○●○●○
// ○●○●○●○●○●○●
// ●○●○●○●○●○●○
// ○●○●○●○●○●○●
// ●○●○●○●○●○●○
// ○●○●○●○●○●○●
// ●○●○●○●○●○●○
// ○●○●○●○●○●○●

// ◆◇◆◇◆◇◆◇◆◇◆◇◆◇
// ◇◆◇◆◇◆◇◆◇◆◇◆◇◆
// ◆◇◆◇◆◇◆◇◆◇◆◇◆◇
// ◇◆◇◆◇◆◇◆◇◆◇◆◇◆
// ◆◇◆◇◆◇◆◇◆◇◆◇◆◇
// ◇◆◇◆◇◆◇◆◇◆◇◆◇◆
// ◆◇◆◇◆◇◆◇◆◇◆◇◆◇
// ◇◆◇◆◇◆◇◆◇◆◇◆◇◆
// ◆◇◆◇◆◇◆◇◆◇◆◇◆◇
// ◇◆◇◆◇◆◇◆◇◆◇◆◇◆
// ◆◇◆◇◆◇◆◇◆◇◆◇◆◇
// ◇◆◇◆◇◆◇◆◇◆◇◆◇◆

/*
以下のような射影する行列を考える。
A: (x,y) -> (nx,ny)
A: (0,0) -> (0,0)
A: (1,1) -> (1,0)
A=[
    [1 1],
    [-1 1]
]
となる行列が良さそう。
つまり、
NX =  X+Y
NY = -X+Y

◆写像先の大きさ(NH, NW)を考える。
nx_min = 0 + 0 = 0
nx_max = xmax + ymax

ny_min = (-1)*xmax + 0 = -xmax
ny_max = (-1)*0 + ymax = ymax

よって、
NH = 1 + ny_max - ny_min = 1 + ymax + xmax 
NW = 1 + nx_max - nx_min = 1 + xmax + ymax

◆最終的な写像
NYが負になる可能性があって嫌なので、常にオフセットをつけておきたい。
NX =  X+Y
NY = xmax-X+Y
*/


    

