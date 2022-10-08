use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }
    let modulo: usize = 998244353;

    // dp[i][j][m][k]: 下記i,j,kを満たすうち、平均値が整数である
    // i: 数列aの内、インデックスが0-iまでのものを使える。
    // j: 実際に使う整数の数
    // m: mで割る
    // k: mで割った余りがkのもの
    let mut dp = vec![vec![vec![vec![0; n]; n+1]; n+1]; n];

    // 例
    // N = 3, A = [2,6,2]
    // 3
    // 2 6 2
    // 使: 0   0   0   1   1   1   2   2   2   3   3   3  
    // 余:[0   1   2] [0   1   2] [0   1   2] [0   1   2] 
    // 0  [0   0   0] [1   0   0] [0   0   0] [0   0   0] 
    // 1  [0   0   0] [2   0   0] [1   0   0] [0   0   0] 
    // 2  [0   0   0] [3   0   0] [3   0   0] [0   1   0] 

    for i in 0..n {
        dp[i][0][0][0] = i + 1;
    }

    for m in 1..(n+1) {
        for i in 1..n {
            for j in 1..(n+1) {
                for k in 0..n {
                    // 追加しない場合                        
                    dp[i][j][m][k] += dp[i-1][j][m][k];
                    // 追加する場合
                    dp[i][j][m][k] += dp[i-1][j-1][m][(k +  m - (a[i]) % m) % m];
                }
            }
        }
    }
    for i in 0..n {
        println!("{:?}", dp[i]);
    }


}