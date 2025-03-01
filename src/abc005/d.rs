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
    // 2025-03-01 13:38-14:07 (29min)
    input! {
        n: usize,
        d: [[usize; n]; n],
        q: usize,
        p: [usize; q]
    }
    // n^4<=50^4=   6_250_000
    // n^5<=50^5=   312_500_000
    // n^6<=50^6<=  15_625_000_000

    // solve_o_n5(n, d, q, p); // 自己流O(N^5)解法
    solve_o_n4(n, d, q, p); // 公式O(N^4)解法
}

fn solve_o_n4(n: usize, d: Vec<Vec<usize>>, q: usize, p: Vec<usize>) {
    // O(N^4)で解く (公式解説)

    // 2次元累積和 O(N^2)
    let cum = get_2d_cumulative_sum(&d);

    let mut s = vec![0; n*n+1];

    // 2次元マップ内の直方体の全探索 O(N^4)
    for ys in 0..n {
        for xs in 0..n {
            for yt in ys..n {
                for xt in xs..n {
                    let sum = get_sum_from_2d_cum(&cum, ys, xs, yt, xt);
                    let area = (1+yt-ys)*(1+xt-xs);
                    s[area] = max(s[area], sum);
                }
            }
        }
    }
    for area in 1..n*n+1 {
        s[area] = max(s[area], s[area-1]);
    }

    // O(N^2 * N^3) = O(N^5) の解法
    for i in 0..q {
        // let devisors = enumerate_devisors(p[i]);
        // println!("devisors = {:?}", devisors);
        let mut ans = s[p[i]];
        println!("{}", ans);
    }

}


fn solve_o_n5(n: usize, d: Vec<Vec<usize>>, q: usize, p: Vec<usize>) {
    // O(N^5)で解く (自分の初提出。一応AC)
    let cum = get_2d_cumulative_sum(&d);
    // println!("------");
    // cum.print_2d_array();
    // println!("------");
    // q <= n^2
    // 始点は、n^2

    // O(N^2 * N^3) = O(N^5) の解法
    for i in 0..q {
        // let devisors = enumerate_devisors(p[i]);
        // println!("devisors = {:?}", devisors);
        let mut ans = 0;
        for ys in 0..n {
            for xs in 0..n {
                // for h in devisors.iter() {
                for h in 1..n+1 {
                    let w = p[i] / h;
                    if w == 0 {continue}

                    let yt = min(ys+h-1, n-1);
                    let xt = min(xs+w-1, n-1);
                    let sum = get_sum_from_2d_cum(&cum, ys, xs, yt, xt);
                    ans = max(ans, sum);
                }
            }
        }
        println!("{}", ans);
    }
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

/// 自然数Nの約数をO(√N)で昇順に全列挙する関数。
/// 約数の個数は意外と少ない https://github.com/E869120/kyopro_educational_90/blob/main/editorial/085-01.jpg
/// N <= 10^6 なら、N=720,720 が最大で、240個
/// N <= 10^9 なら、N=735,134,400 が最大で、1,344個
/// N <= 10^12 なら、N=963,761,198,400 が最大で、6,720個
/// N <= 10^18 なら、N=897,612,484,786,617,600 が最大で、103,680個
fn enumerate_devisors<T>(n: T) -> Vec<T> 
    where T: 
        Default + 
        Copy + 
        std::fmt::Display + 
        std::cmp::PartialEq + 
        std::ops::Not<Output = T> + 
        std::ops::Add<Output = T> +
        std::ops::Div<Output = T> +
        std::ops::Rem<Output = T> + 
        std::ops::Mul<Output = T> +
        std::cmp::PartialOrd

{
    // 自然数Nの約数を列挙する関数O(√N)
    let mut devisors = vec![];
    let mut devisors_over_root_n = vec![]; // √N より大きい約数を一時的に格納するリスト

    let zero: T = Default::default();
    let not_zero: T = !zero;
    let one: T = not_zero / not_zero;

    let mut devisor = one;
    while (devisor * devisor) <= n {
        if n % devisor == zero {
            devisors.push(devisor);
            if n / devisor != devisor {
                devisors_over_root_n.push(n / devisor);                
            }
        }
        devisor = devisor + one;
    }
    // println!("devisors_over_root_n = {:?}", devisors_over_root_n);

    for &devisor_over_root_n in devisors_over_root_n.iter().rev() {
        devisors.push(devisor_over_root_n);
    }
    return devisors
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