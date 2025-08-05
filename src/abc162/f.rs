#![allow(dead_code, unused_imports)]
use proconio::{input, marker::{Usize1, Isize1, Chars}};
use itertools::Itertools;
use std::cmp::{max, min, Ordering, Reverse};
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::collections::{VecDeque, BinaryHeap, HashMap, BTreeMap, HashSet, BTreeSet};
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
use rand::Rng;
use num::{BigUint, ToPrimitive, Integer};
use num_bigint::ToBigUint;
fn main() {
    // 2025-08-01 12:37-12:51 (14min)
    // 2025-08-02 15:08-16:41 (1h33min)
    // total: 1h47min
    input! {
        n: usize,
        a: [isize; n],
    }
    solve(n, a);

    // debug用
    // loop {
    //     let (n, a) = make_random_testcase();
    //     println!("{}",n);
    //     println!("{}", a.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
    //     let ans_bf = solve_bf(n, a.clone());
    //     let ans = solve(n, a);
    //     assert_eq!(ans_bf, ans);
    // }

}

fn make_random_testcase() -> (usize, Vec<isize>) {
    let mut rng = rand::thread_rng();
    let n = rng.gen_range(2..=5);
    let mut a = vec![0; n];
    for i in 0..n {
        // a[i] = rng.gen_range(-1_000_000_000..=1_000_000_000);
        a[i] = rng.gen_range(-5..=5);
    }
    (n, a)
}

fn solve_bf(n: usize, a: Vec<isize>) -> isize {
    let num = n/2;
    let mut ans: isize = - 1_000_000_000_000_000_000;
    for comb in (0..n).combinations(num) {
        let mut sum = 0;

        let mut is_ok = true;
        for i in 1..comb.len() {
            if comb[i] == comb[i-1] + 1 {
                is_ok = false;
                break;
            }
        }
        if !is_ok {
            continue;
        }

        for c in comb {
            sum += a[c];
        }
        ans = max(ans, sum);
    }
    println!("{}", ans);
    return ans
}

fn solve(n: usize, a: Vec<isize>) -> isize{
    // DPを頑張れば行ける気がする。

    // min, max
    // dp[i][0] := i番目まで選んだとき、選んだ個数の最小値
    // dp[i][1] := i番目まで選んだとき、選んだ個数の最大値
    let mut dp = vec![[0, 0]; n];    
    for i in 0..n-1 {
        if i % 2 == 0 {
            dp[i][0] = i/2;
            dp[i+1][0] = i/2;
            
            dp[i][1] = i/2 + 1;
            dp[i+1][1] = i/2 + 1;
        }
    }
    dp[n-1][0] = n/2;
    dp[n-1][1] = n/2;

    if n % 2 == 0 {
        for i in 0..n-1 {
            if i % 2 == 1 {
                dp[i][0] = i/2 + 1;
            }
        }
    }
    dp[0][0] = 0;

    // debug
    // for i in 0..n {
    //     println!("dp[{i}] = {:?}", dp[i]);
    // }

    // dp2[i][0][0] := i番目を選ばないとき、かつ、選んだ個数がminのときの最大値
    // dp2[i][0][1] := i番目を選ばないとき、かつ、選んだ個数がmaxのときの最大値
    // dp2[i][1][0] := i番目を選んだとき、かつ、選んだ個数がminのときの最大値
    // dp2[i][1][1] := i番目を選んだとき、かつ、選んだ個数がmaxのときの最大値

    let ninf: isize = - 1_000_000_000_000_000_000;
    let mut dp2 = vec![[[ninf; 2]; 2]; n];
    dp2[0][0][0] = 0;
    dp2[0][0][1] = ninf; // 存在しない
    dp2[0][1][0] = ninf; // 存在しない
    dp2[0][1][1] = a[0];

    // 配るDP
    for i in 0..n-1 {
        let num_min = dp[i][0];
        let num_max = dp[i][1];

        let n_num_min = dp[i+1][0];
        let n_num_max = dp[i+1][1];


        // i番目で選んだとき
        if dp2[i][1][0] != ninf {
            if num_min == n_num_min {
                dp2[i+1][0][0] = max(dp2[i+1][0][0], dp2[i][1][0]);
            }
            if num_min == n_num_max {
                dp2[i+1][0][1] = max(dp2[i+1][0][1], dp2[i][1][0]);
            }
        }
        if dp2[i][1][1] != ninf {
            if num_max == n_num_min {
                dp2[i+1][0][0] = max(dp2[i+1][0][0], dp2[i][1][1]);
            }
            if num_max == n_num_max {
                dp2[i+1][0][1] = max(dp2[i+1][0][1], dp2[i][1][1]);
            }
        }
        // i番目で選ばなかったとき
        if dp2[i][0][0] != ninf {
            // i+1番目でも選ばないとき
            if num_min == n_num_min {
                dp2[i+1][0][0] = max(dp2[i+1][0][0], dp2[i][0][0]);
            }
            if num_min == n_num_max {
                dp2[i+1][0][1] = max(dp2[i+1][0][1], dp2[i][0][0]);
            }

            // i+1番目では選ぶとき
            if num_min + 1 == n_num_min {
                dp2[i+1][1][0] = max(dp2[i+1][1][0], dp2[i][0][0] + a[i+1]);
            }
            if num_min + 1 == n_num_max {
                dp2[i+1][1][1] = max(dp2[i+1][1][1], dp2[i][0][0] + a[i+1]);
            }
        }
        if dp2[i][0][1] != ninf {
            // i+1番目でも選ばないとき
            if num_max == n_num_min {
                dp2[i+1][0][0] = max(dp2[i+1][0][0], dp2[i][0][1]);
            }
            if num_max == n_num_max {
                dp2[i+1][0][1] = max(dp2[i+1][0][1], dp2[i][0][1]);
            }
            // i+1番目では選ぶとき
            if num_max + 1 == n_num_min {
                dp2[i+1][1][0] = max(dp2[i+1][1][0], dp2[i][0][1] + a[i+1]);
            }
            if num_max + 1 == n_num_max {
                dp2[i+1][1][1] = max(dp2[i+1][1][1], dp2[i][0][1] + a[i+1]);
            }
        }
    }
    let mut ans = max(dp2[n-1][0][1], dp2[n-1][1][1]);
    ans = max(ans, dp2[n-1][0][0]);
    ans = max(ans, dp2[n-1][1][0]);
    println!("{}", ans);
    return ans;

}