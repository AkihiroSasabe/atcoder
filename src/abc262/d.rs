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
    // 選択数:      1個選択                                     2個選択                                      3個選択
    // 割る数:      mod1        mod2          mod3             mod1          mod2          mod3            mod1          mod2          mod3
    // 余り:      0  1  2       0  1  2      0   1  2          0  1  2       0  1  2      0   1  2         0  1  2       0  1  2      0   1  2
    // a1   m1: [1, 0, 0] m2: [1, 0, 0] m3: [0, 0, 1]    m1: [0, 0, 0] m2: [0, 0, 0] m3: [0, 0, 0]   m1: [0, 0, 0] m2: [0, 0, 0] m3: [0, 0, 0] 
    // a2   m1: [2, 0, 0] m2: [2, 0, 0] m3: [1, 0, 1]    m1: [1, 0, 0] m2: [1, 0, 0] m3: [0, 0, 1]   m1: [0, 0, 0] m2: [0, 0, 0] m3: [0, 0, 0] 
    // a3   m1: [3, 0, 0] m2: [3, 0, 0] m3: [1, 0, 2]    m1: [3, 0, 0] m2: [3, 0, 0] m3: [0, 1, 2]   m1: [1, 0, 0] m2: [1, 0, 0] m3: [0, 1, 0] 


    for i in 0..n {
        for m in 1..n+1 {
            dp[i][1][m][a[i]%m] += 1;
            dp[i][1][m][a[i]%m] %= modulo;
        }
    }

    // println!("初期化");
    // for i in 0..n {
    //     for j in 1..n+1 {
    //         for m in 1..n+1 {
    //             print!("m{}: {:?} ", m, dp[i][j][m]);
    //         }
    //     }
    //     println!("");
    // }


    // 配る型の遷移が楽。余りが負になることを考えなくていいので。
    for i in 0..n-1 {
        for j in 1..n+1 {
            for m in 1..n+1 {
                for k in 0..n {
                    if k >= m {continue}
                    // 上からの遷移
                    dp[i+1][j][m][k] += dp[i][j][m][k];
                    dp[i+1][j][m][k] %= modulo;
                    // 左上からの遷移
                    if j + 1 > n {continue}
                    dp[i+1][j+1][m][(k + a[i+1]) % m] += dp[i][j][m][k];
                    dp[i+1][j+1][m][(k + a[i+1]) % m] %= modulo;
                }
            }
        }
    }

    let mut ans = 0;
    for j in 1..n+1 {
        ans += dp[n-1][j][j][0];
        ans %= modulo;
    }
    // println!("収束後");
    // for i in 0..n {
    //     for j in 1..n+1 {
    //         for m in 1..n+1 {
    //             print!("mod{}: {:?} ", m, dp[i][j][m]);
    //         }
    //     }
    //     println!("");
    // }
    println!("{}", ans);


}