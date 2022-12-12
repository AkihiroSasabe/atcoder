#![allow(dead_code, unused_imports)]
use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    input! {
        n: usize,
        k: usize,
        d: usize,
        a: [usize; n]
    }

    let INF: usize = 1_000_000_000_000;

    let mut dp = vec![vec![vec![INF; d]; k+1]; n];

    dp[0][0][0] = 0;
    dp[0][1][a[0] % d] = a[0];
    for nn in 0..n {
        for dd in 0..d {
            dp[nn][0][0] = 0;
        }
    }

    for nn in 1..n {
        for kk in 1..(k+1) {
            for dd in 0..d {
                // 上から下への遷移
                if dp[nn-1][kk][dd] != INF {
                    if dp[nn][kk][dd] == INF {
                        dp[nn][kk][dd] = dp[nn-1][kk][dd];
                        // if nn == 1 && kk == 1 && dd == 1 {
                        //     println!("1 dao {}", dp[nn][kk][dd]);
                        // }
                    }
                    else {
                        dp[nn][kk][dd] = max(dp[nn][kk][dd], dp[nn-1][kk][dd]);
                        // if nn == 1 && kk == 1 && dd == 1 {
                        //     println!("2 dao");
                        // }
                    }
                }
                // 左上からの遷移
                let mut moto = 0;
                if dd >= a[nn] % d {
                    moto = dd - a[nn] % d;
                }
                else {
                    moto = d + dd - a[nn] % d;
                }
                if dp[nn-1][kk-1][moto] != INF {
                    if dp[nn][kk][dd] == INF {
                        // a[nn] % d == dd && 
                        dp[nn][kk][dd] = dp[nn-1][kk-1][moto] + a[nn];
                        // if nn == 1 && kk == 1 && dd == 1 {
                        //     println!("3 dao");
                        // }
                    }
                    else {
                        // if nn == 1 && kk == 1 && dd == 1 {
                        //     println!("4 dao {}", dp[nn][kk][dd]);
                        //     // println!("moto: {moto}");
                        //     dbg!(moto, dd, a[nn] % d, a[nn], dp[nn][kk][dd], dp[nn-1][kk-1][moto]);
                        // }
                        dp[nn][kk][dd] = max(dp[nn][kk][dd], dp[nn-1][kk-1][moto] + a[nn]);
                        // if nn == 1 && kk == 1 && dd == 1 {
                        //     println!("4 dao {}", dp[nn][kk][dd]);
                        //     // println!("moto: {moto}");
                        //     dbg!(moto, dd, a[nn] % d, a[nn], dp[nn][kk][dd], dp[nn-1][kk-1][moto]);
                        // }
                        
                    }
                }
            }
        }
    }

    // for nn in 0..n {
    //     for kk in 0..(k+1) {
    //         print!("{:?}", dp[nn][kk]);
    //     }
    //     println!("");
    // }

    if dp[n-1][k][0] != INF {
        println!("{}", dp[n-1][k][0]);
    }
    else {
        println!("-1");
    }

}