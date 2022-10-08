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
        m: usize,
        x: [usize; n],
        // mut cy: [[usize; 2]; m]
    }
    // cy.sort();
    let mut c = vec![];
    let mut y = vec![];
    for i in 0..m {
        input! {
            c_i: usize,
            y_i: usize,
        }
        c.push(c_i);
        y.push(y_i);
    }
    let mut bonus = vec![0; n+1];
    for i in 0..m {
        bonus[c[i]] = y[i];
    }

    // // 累積和
    // let mut bonus_sum = vec![0; n];
    // let mut current_sum = 0;
    // for i in 0..(m-1) {
    //     current_sum += cy[i][1];
    //     for j in cy[i][0]..cy[i+1][0] {
    //         bonus_sum[j] = current_sum;
    //     }
    // }
    // current_sum += cy[m-1][1];
    // for j in cy[m-1][0]..n {
    //     bonus_sum[j] = current_sum;
    // }
    // println!("bonus_sum: {:?}", bonus_sum);

    let mut dp = vec![vec![0; n + 1]; n];
    let mut dp_max = vec![0; n];
    dp_max[0] = x[0]  + bonus[1];
    dp[0][1] = x[0] + bonus[1];
    for i in 1..n {
        for count in 0..(n+1) {
            if i + 1 < count {continue}
            if count == 0 {
                dp[i][count] = dp_max[i-1];
                dp_max[i] = max(dp[i][count], dp_max[i]);
            }
            else {
                dp[i][count] = max(dp[i-1][count - 1] + x[i] + bonus[count], dp[i][count]);
                dp_max[i] = max(dp[i][count], dp_max[i]);
            }
        }
    }
    println!("{}", dp_max[n-1]);

    // for i in 0..n {
    //     println!("{:?}", dp[i]);
    // }

    
}