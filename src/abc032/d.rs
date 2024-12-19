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
fn main() {
    // 2024-12-18 19:34-20:55 (1h21min)
    input! {
        n: usize, // 1<= n <= 200
        w: usize, // 1 <= w <= 10^9
    }
    let mut vs = vec![];
    let mut ws = vec![];
    for i in 0..n {
        input! {
            ui: usize, // 1 <= ui <= 10^9
            wi: usize, // 1 <= wi <= 10^9
        }
        vs.push(ui);
        ws.push(wi);
    }

    // 貪欲に、重さに対して、コスパのいい奴を選んでいく?
    // 決め打ち二分探索では?

    // 価値X以上を出すことは可能か?
    // 重さの組合せ

    // [1] N <= 30
    // [2] 1<=wi<=1_000
    // [3] 1<=vi<=1_000

    if n <= 30 {
    // if n <= 0 {
        // 2^30=1_073_741_824
        // 半分全列挙で行けるのでは?
        let n0 = n / 2;
        let n1 = n - n0;
        // 2^15 = 32_768
        let mut wvs0 = vec![];
        for mask in 0..1<<n0 {
            let mut vi= 0;
            let mut wi= 0;
            for i in 0..n0 {
                if mask & (1 << i) != 0 {
                    vi += vs[i];
                    wi += ws[i];
                }
            }
            wvs0.push((wi, vi));
        }
        wvs0.sort();

        let mut wvs1 = vec![];
        for mask in 0..1<<n1 {
            let mut vi= 0;
            let mut wi= 0;
            for i in 0..n1 {
                if mask & (1 << i) != 0 {
                    vi += vs[n0+i];
                    wi += ws[n0+i];
                }
            }
            wvs1.push((wi, vi));
        }
        wvs1.sort();
        
        // aaa0[i] := 前半の集合S0について、i番目に重たい組合せの中で、最大価値
        let mut aaa0 = vec![0; wvs0.len()];
        for i in 1..wvs0.len() {
            aaa0[i] = max(aaa0[i-1], wvs0[i].1);
        }

        // aaa1[i] := 後半の集合S1について、i番目に重たい組合せの中で、最大価値
        let mut aaa1 = vec![0; wvs1.len()];
        for i in 1..wvs1.len() {
            aaa1[i] = max(aaa1[i-1], wvs1[i].1);
        }

        // ws1[i] := 後半の集合S1について、i番目に重たい組合せの重さ
        let mut ws1 = vec![];
        for i in 0..wvs1.len() {
            ws1.push(wvs1[i].0);
        }

        let mut ans = 0;
        for i in 0..wvs0.len() {
            if w < wvs0[i].0 {continue}
            // rw := iの相方の最大重さ
            let rw = w - wvs0[i].0;
            // ind := rwより重い最小インデックス
            let ind = ws1.lower_bound(&(rw+1));
            if ind == 0 {continue}
            // 前半で i 番目の組合せを選んだときの、最大価値の和
            let val = aaa0[i] + aaa1[ind-1];
            ans = max(ans, val);
        }
        println!("{}", ans);
    }
    else {
        let vmax = *vs.iter().max().unwrap();
        let vsum: usize = vs.iter().sum::<usize>();
        let INF: usize = 1 << 60;
        if vmax <= 1000 {
        // if vmax <= 0 {
            // dp[i][v] := 価値vを維持したときの重さの最小
            let mut dp = vec![vec![INF; vsum + 1]; n + 1];
            dp[0][0]=0;
            for i in 0..n {
                dp[i+1] = dp[i].clone();
                for vi in 0..vsum+1 {
                    if vi + vs[i] > vsum {continue}
                    dp[i+1][vi + vs[i]] = min(dp[i+1][vi + vs[i]], dp[i][vi] + ws[i]);
                }
            }
            // println!("dp = {:?}", dp);
            // dp.print_2d_array();

            let mut ans = 0;
            for vi in 0..vsum+1 {
                if dp[n][vi] <= w {
                    ans = vi;
                }
            }
            println!("{}", ans);
        }
        else {
            let wmax = *ws.iter().max().unwrap();
            let wsum = ws.iter().sum::<usize>();
            // dp[i][重さ] := 重さのときの最大価値
            let mut dp = vec![vec![0; wsum + 1]; n + 1];
            dp[0][0]=0;
            for i in 0..n {
                dp[i+1] = dp[i].clone();
                for wi in 0..wsum+1 {
                    if wi + ws[i] > wsum {continue}
                    dp[i+1][wi + ws[i]] = max(dp[i+1][wi + ws[i]], dp[i][wi] + vs[i]);
                }
            }

            let mut ans = 0;
            for wi in 0..wsum+1 {
                if wi <= w {
                    ans = max(ans, dp[n][wi]);
                }
            }
            println!("{}", ans);
        }
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