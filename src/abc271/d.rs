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
use superslice::*;
fn main() {
    input! {
        n: usize,
        s: usize,
    }

    let mut a = vec![];
    let mut b = vec![];
    for i in 0..n {
        input! {
            a_i: usize,
            b_i: usize
        }
        a.push(a_i);
        b.push(b_i);
    }

    // Head(表), Tail(裏)
    let s_max = 10_000; // s < a[i] or b[i] の時があるので、dpの列数をsではなく、s_maxにしないといけない
    let mut dp = vec![vec![0; s_max+1]; n];
    dp[0][a[0]] = 1;
    dp[0][b[0]] = 1;

    if n != 1 {
        for i in 0..n-1 {
            for j in 1..s+1 {
                if j+a[i+1] <= s {
                    dp[i+1][j+a[i+1]] |= dp[i][j];
                }
                if j+b[i+1] <= s {
                    dp[i+1][j+b[i+1]] |= dp[i][j];
                }            
            }
        }
    }

    // for i in 0..n {
    //     println!("{:?}", dp[i]);
    // }

    if dp[n-1][s] == 1 {
        println!("Yes");
        let mut i = n-1;
        let mut sum = s;
        // println!("aaa {}", i);
        let mut ans = vec![];
        while i > 0 {
            // println!("bbb {}", i);
            if sum >= a[i] && dp[i-1][sum-a[i]] == 1 {
                // print!("H");
                ans.push("H");
                sum -= a[i];
            }
            else {
                ans.push("T");
                // print!("T");
                sum -= b[i];
            }
            i -= 1;
        }
        if sum == a[0] {
            ans.push("H");
            // print!("H");
        }
        else {
            ans.push("T");
            // print!("T");
        }
        for j in 0..n {
            print!("{}", ans[n-1-j]);
        }
    }
    else {
        println!("No");
    }

}