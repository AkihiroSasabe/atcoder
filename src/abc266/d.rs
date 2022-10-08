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
    }
    let mut t = vec![];
    let mut x = vec![];
    let mut a = vec![];
    for i in 0..n {
        input! {
            t_i: usize,
            x_i: usize,
            a_i: isize,
        }
        t.push(t_i);
        x.push(x_i);
        a.push(a_i);
    }

    let N_INF = -1 * (1 << 60); 
    let T_MAX = 100_001;
    let mut dp = vec![vec![N_INF; 5]; T_MAX+1];
    dp[0][0] = 0;
    for i in 0..n {
        if t[i] >= x[i] {
            dp[t[i]][x[i]] = a[i];
        }
    }
    let mut current_n = 0;
    for i in 0..T_MAX {
        let mut flag = false;
        if current_n < n && t[current_n] == i+1 {
            flag = true;
            current_n += 1;
        }
        for j in 0..5 {
            let mut left_j = j;
            let mut right_j = j;
            if j > 0 {
                left_j = j - 1;
            }
            if j < 4 {
                right_j = j + 1;
            }
            
            if flag {
                if x[current_n - 1] == left_j {
                    dp[i+1][left_j] = max(dp[i][j] + a[current_n - 1], dp[i+1][left_j]);                    
                }
                else if x[current_n - 1] == j {
                    dp[i+1][j] = max(dp[i][j] + a[current_n - 1], dp[i+1][j]);                
                }
                else if x[current_n - 1] == right_j {
                    dp[i+1][right_j] = max(dp[i][j] + a[current_n - 1], dp[i+1][right_j]);            
                }
            }
            dp[i+1][left_j] = max(dp[i][j], dp[i+1][left_j]); 
            dp[i+1][j] = max(dp[i][j], dp[i+1][j]);
            dp[i+1][right_j] = max(dp[i][j], dp[i+1][right_j]);
        }
    }

    let mut ans = 0;
    for i in 0..5 {
        ans = max(ans, dp[T_MAX-1][i]);
    }

    println!("{}", ans);


}