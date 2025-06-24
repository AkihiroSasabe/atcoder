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
    // solve();
    solve_2nd();

}

fn solve_2nd() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        b: [usize; m],
    }
    
    // 最終的な長さで考えようか.
    let min_len = min(n, m);
    let inf = 1 << 60;

    // dp[i][j] := i と j がマッチしているとき、消した個数の最小は?
    let mut dp = vec![vec![inf; m]; n];

    for j in 0..m {
        
    }





}

fn solve() {
    // 2024-03-16 15:24-16:14 (50min, 答えここからみた)
    // 2024-03-20 15:41-
    // 2024-04-09 12:15-12:54 (39min, LCS: 最長共通部分列問題の勉強をしてから実装。)
    // 2024-04-09 19:33-20:08 (25min, debug: x=0の初期化と、y=0の初期化で、ミスっていた)
    // 50 + 74 = 124min
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        b: [usize; m],
    }

    // 2次元DP
    let INF = 1 << 60;
    // dp[i][j] := a[i]とb[j]まで使っていいときの、x+yの最小値
    // LCSのように、dp[i][j] -> dp[i+1][j+1] の遷移だけ考えればよい。
    let mut dp = vec![vec![INF; m]; n];

    // 原点
    if a[0] == b[0] {
        let x = n+m-2;  // 取り除く合計数
        let y = 0;      // 不一致なiの個数
        dp[0][0] = min(dp[0][0], x+y);
    }
    else {
        let x = n+m-2;
        let y = 1;
        dp[0][0] = min(dp[0][0], x+y);
    }
    // x=0(y軸)の初期化
    for i in 1..n {
        dp[i][0] = min(dp[i][0], dp[i-1][0]);
        if a[i] == b[0] {
            let x = n+m-2;
            let y = 0;
            dp[i][0] = min(dp[i][0], x+y);
        }
    }
    // y=0(x軸)の初期化
    for i in 1..m {
        dp[0][i] = min(dp[0][i], dp[0][i-1]);
        if a[0] == b[i] {
            let y = n+m-2;
            let x = 0;
            dp[0][i] = min(dp[0][i], x+y);
        }
    }
    
    for i in 0..n {
        for j in 0..m {
            if i+1 < n {
                dp[i+1][j] = min(dp[i+1][j], dp[i][j]);
            }
            if j+1 < m {
                dp[i][j+1] = min(dp[i][j+1], dp[i][j]);
            }
            if i+1 < n && j+1 < m {
                // i+1とj+1を、末尾に追加
                // 取り除く文字数xが2個減って、不一致なiの個数yが1個増えるので、-1
                dp[i+1][j+1] = min(dp[i+1][j+1], dp[i][j] - 1);
                if a[i+1] == b[j+1] {
                    // 取り除く文字数xが2個減って、不一致なiの個数は変わらないので、-2
                    dp[i+1][j+1] = min(dp[i+1][j+1], dp[i][j] - 2);
                }
            }
        }
    }
    // dp.print_2d_array();
    println!("{}", dp[n-1][m-1]);


    // // dp[i][j] := お尻がA[i]、B[j] となるときの、x+yの最小値
    // let INF = 1 << 60;
    // let mut dp = vec![vec![INF; m]; n];
    // if a[0] == b[0] {
    //     dp[0][0] = 0;
    // }
    // else {
    //     dp[0][0] = 1;
    // }

    // for i in 0..n {
    //     for j in 0..m {
            
    //     }
    // }

    // // [1]: A[i]とB[j]の位置を揃える
    // // [2]: A[i]を消す
    // // [3]: B[j]を消す

    // // [1]のケース
    // // dp[i][j] = dp[i-1][j] + 1

    // // [2]のケース
    // // dp[i][j] = dp[i][j-1] + 1

    // // [3]のケース
    // // dp[i][j] = dp[i-1][j-1] + 0 or 1 if a[i] == b[j] else

    // for i in 0..n {
    //     for j in 0..m {
    //         if i == 0 && j == 0 {continue}
    //         // [1]のケース
    //         dp[i][j] = min(dp[i][j], dp[i+1][j] + 1);

    //         // [2]のケース
    //         // dp[i][j] = dp[i][j-1] + 1
    //         dp[i][j] = min(dp[i][j], dp[i-1][j] + 1);

    //         // [3]のケース
    //         if a[i] == b[j] {
    //             dp[i][j] = min(dp[i][j], dp[i-1][j-1]);
    //         }
    //         else {
    //             dp[i][j] = min(dp[i][j], dp[i-1][j-1] + 1);
    //         }


    //         // // [1]のケース
    //         // dp[i][j] = min(dp[i][j], dp[i-1][j] + 1);

    //         // // [2]のケース
    //         // // dp[i][j] = dp[i][j-1] + 1
    //         // dp[i][j] = min(dp[i][j], dp[i-1][j] + 1);

    //         // // [3]のケース
    //         // if a[i] == b[j] {
    //         //     dp[i][j] = min(dp[i][j], dp[i-1][j-1]);
    //         // }
    //         // else {
    //         //     dp[i][j] = min(dp[i][j], dp[i-1][j-1] + 1);
    //         // }
    //     }
    // }
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