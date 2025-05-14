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
    // 2025-05-10 10:39-11:53 (1h14min, BruteForce)
    // 2025-05-10 11:53-13:16 (1h23min)
    // 2025-05-14 12:38-12:56 (18min)
    // 2025-05-14 18:49-21:25 (2h36min)
    // Total: 5h31min
    input! {
        n: usize,
        m: usize,
    }
    solve(n, m);
    // solve_bf(n, m);

}

fn solve(n: usize, m: usize) {
    let modulus = 998244353;

    // dp1[i][m1] := i番目までみたとき、最長増加部分列の長さが1で、最小の要素がm1である数列の個数
    let mut dp1 = vec![vec![0; m+1]; n];
    // dp2[i][m1][m2] := i番目までみたとき、最長増加部分列の長さが2で、長さ1の最小の要素がm1で、長さ2の最小の要素がm2である数列の個数
    let mut dp2= vec![vec![vec![0; m+1]; m+1]; n];
    // dp3[i][m1][m2][m3] := i番目までみたとき、最長増加部分列の長さが3で、長さ1の最小の要素がm1で、長さ2の最小の要素がm2で、長さ3の最小の要素がm3で、ある数列の個数
    let mut dp3= vec![vec![vec![vec![0; m+1]; m+1]; m+1]; n];

    for i in 1..m+1 {
        dp1[0][i] = 1;
    }
    for i in 0..n-1 {
        for m1 in 1..m+1 {
            for nm1 in 1..m1+1 {
                dp1[i+1][nm1] += dp1[i][m1];
                dp1[i+1][nm1] %= modulus;
            }
        }
    }
    // println!("dp1 = {:?}", dp1);

    for i in 0..n-1 {
        for m1 in 1..m+1 {
            for nm2 in m1+1..m+1 {
                dp2[i+1][m1][nm2] += dp1[i][m1];
                dp2[i+1][m1][nm2] %= modulus;
            }
            for m2 in m1+1..m+1 {
                for nm1 in 1..m1+1 {
                    dp2[i+1][nm1][m2] += dp2[i][m1][m2];
                    dp2[i+1][nm1][m2] %= modulus;
                }
                for nm2 in m1+1..m2+1{
                    dp2[i+1][m1][nm2] += dp2[i][m1][m2];
                    dp2[i+1][m1][nm2] %= modulus;
                }
            }
        }
    }
    // println!("dp2");
    // for i in 0..n {
    //     for m1 in 0..m+1 {
    //         for m2 in 0..m+1 {
    //             println!("dp2[{i}][{m1}][{m2}] = {:?}", dp2[i][m1][m2]);
    //         }
    //     }
    //     // println!("dp2[{i}] = {:?}", dp2[i]);
    // }

    for i in 0..n-1 {
        for m1 in 1..m+1 {
            for m2 in m1+1..m+1 {
                for nm3 in m2+1..m+1 {
                    dp3[i+1][m1][m2][nm3] += dp2[i][m1][m2];
                    dp3[i+1][m1][m2][nm3] %= modulus;
                }
                for m3 in m2+1..m+1 {
                    for nm1 in 1..m1+1 {
                        dp3[i+1][nm1][m2][m3] += dp3[i][m1][m2][m3];
                        dp3[i+1][nm1][m2][m3] %= modulus;
                    }
                    for nm2 in m1+1..m2+1 {
                        dp3[i+1][m1][nm2][m3] += dp3[i][m1][m2][m3];
                        dp3[i+1][m1][nm2][m3] %= modulus;
                    }
                    for nm3 in m2+1..m3+1 {
                        dp3[i+1][m1][m2][nm3] += dp3[i][m1][m2][m3];
                        dp3[i+1][m1][m2][nm3] %= modulus;   
                    }
                }
            }
        }
    }
    // println!("dp3");
    // for i in 0..n {
    //     for m1 in 1..m+1 {
    //         for m2 in m1+1..m+1 {
    //             for m3 in m2+1..m+1 {
    //                 println!("dp3[{i}][{m1}][{m2}][{m3}] = {:?}", dp3[i][m1][m2][m3]);
    //             }
    //         }
    //     }
    // }


    let mut ans = 0;
    for m1 in 1..m+1 {
        for m2 in m1+1..m+1 {
            for m3 in m2+1..m+1 {
                // println!("dp3[n-1][{m1}][{m2}][{m3}] = {:?}", dp3[n-1][m1][m2][m3]);

                ans += dp3[n-1][m1][m2][m3];
                ans %= modulus;
            }
        }
    }
    println!("{}", ans);


}


// 愚直解法 (TLE)
fn solve_bf(n: usize, m: usize) {
    let modulus = 998244353;


    fn dfs(d: usize, a: &mut Vec<usize>, ans: &mut usize, n: usize, m: usize, modulus: usize, dp: &mut Vec<usize>) {
        if d == n {

            let mut is_ok_3 = false;
            let mut min_last = 1 << 60;
            for comb in (0..n).combinations(3) {
                let mut pre = 0;
                let mut is_ok = true;
                for &i in comb.iter() {
                    if a[i] <= pre {
                        is_ok = false;
                        break
                    }
                    pre = a[i];
                }
                if is_ok {
                    is_ok_3 = true;
                    min_last = min(min_last, a[comb[2]]);
                }
            }

            let mut is_exist_over_3 = false;
            for num in 4..n+1 {
                for comb in (0..n).combinations(num) {
                    let mut pre = 0;
                    let mut is_ok = true;
                    for i in comb {
                        if a[i] <= pre {
                            is_ok = false;
                            break
                        }
                        pre = a[i];
                    }
                    if is_ok {
                        is_exist_over_3 = true;
                    }
                }
            }
            if is_ok_3 && !is_exist_over_3 {
                println!("a = {:?}", a);
                *ans += 1;
                dp[min_last] += 1;
            }
            return;
        }
        for ad in 1..m+1 {
            a.push(ad);
            dfs(d+1, a, ans, n, m, modulus, dp);
            a.pop();
        }
    }

    let mut ans = 0;
    let mut a = vec![];
    let mut dp = vec![0; m+1];
    dfs(0, &mut a, &mut ans, n, m, modulus, &mut dp);
    println!("{}", ans);
    for i in 1..m+1 {
        println!("dp[{i}] = {:?}", dp[i]);
    }

}
