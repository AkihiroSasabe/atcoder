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
fn main() {
    // 2023-09-27 19:42-20:32 (50min)
    input! {
        n: usize,
        m: usize
    }

    // 考察
    // 仮説: 最初の1人はm通り、次の人は前の人を除外したm-1通り、最後は最初と前の人を除いたm-2通り
    // ans = m * (m-1) * (m-1) * .. * (m-2) = m * (m-1)^(n-2) * (m-2) でいけるのでは?

    // 仮説の検証
    // 入力例1は仮説通り
    //       A   B   C   
    // ans = 3 * 2 * 1 = 6 ?
    // (N, M) = (3, 3)
    // 満たす組み合わせを列挙
    // 0,1,2
    // 0,2,1
    // 1,2,0
    // 1,0,2
    // 2,0,1
    // 2,1,0

    // 入力例2は仮説が通用しない
    //       A   B   C   D
    // ans = 2 * 1 * 1 * 0  // D君は、m-2=2-2=0通りではなく、1通りいける。
    // (N, M) = (4, 2)
    // 満たす組み合わせを列挙
    // 0,1,0,1
    // 1,0,1,0
    // CとAが同じときは、最後はm-2じゃなくてm-1通りの選択肢がある。

    // 以下の入力例は仮説通り
    //       A * B * C
    // ans = 2 * 1 * 0 = 0を満たす
    // (N,M) = (3,2)
    // 0,1,_

    // 仮説を満たすときと、満たさないときの違いはなんだろう。
    // (n,m)と(n-1,m)で考えればいいか?
    // N個が輪じゃない場合は、
    // m * (m-1) * (m-1) * ... * (m-1) * (m-1) = m * (m-1)^(n-1)
    // N個が輪で、最初と最後が同じとき
    // m * (m-1) * (m-1) * ... * (m-1) * 1 = m * (m-1)^(n-2) * 1 ? 

    // 最初と最後が同じかどうかは、N-2番目の人に依存しそう。。。
    // -> i番目の人が、最初の人と同じかどうかで考えると良さそう。
    // -> 解法の通り。

    let modulus = 998244353;
    // dp[i][0] := i番目が、最初と同じ個数
    // dp[i][1] := i番目が、最初と異なる個数
    let mut dp = vec![vec![0; 2]; n];
    // 1人目と2人目は手動で初期化
    dp[0][0] = 0;
    dp[0][1] = m;

    dp[1][0] = 0; // 最初と同じにはなれない
    dp[1][1] = (dp[0][1] * (m-1)) % modulus;
    dp[1][0] %= modulus;
    dp[1][1] %= modulus;

    // 3人目以降は、自ずと決まる。
    for i in 2..n {
        // 最初と同じ
        dp[i][0] = dp[i-1][0] * 0 + dp[i-1][1] * 1;
        // 最初と異なる
        dp[i][1] = (dp[i-1][0] * (m-1)) % modulus + (dp[i-1][1] * (m-2)) % modulus;
        dp[i][0] %= modulus;
        dp[i][1] %= modulus;
    }
    // dp.print_2d_array_transposed_with_name("dp");
    // dp.print_2d_array_with_name("dp");
    println!("{}", dp[n-1][1]);

}

// デバッグ用に2次元配列をprintするトレイト
pub trait Print2DArray {
    fn print_2d_array(&self);
    fn print_2d_array_with_name(&self, name: &str);
    fn print_2d_array_transposed(&self);
    fn print_2d_array_transposed_with_name(&self, name: &str);
}
impl<T: std::fmt::Display> Print2DArray for Vec<Vec<T>> {
    fn print_2d_array(&self) {
        for y in 0..self.len() {
            for x in 0..self[y].len() {
                print!("{} ", self[y][x]);
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
                print!("{} ", self[y][x]);
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