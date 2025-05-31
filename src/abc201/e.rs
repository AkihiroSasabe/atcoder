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
use proconio::marker::{Chars, Usize1};
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    // 2025-05-30 21:06-21:33 (27min)
    // 2025-05-31 10:25-13:00 (2h25min, 木DPで解こうとしたが、うまくデバッグできず。。。)
    // 2025-05-31 16:17-16:22 (5min, 回答みた。)
    // 参考: @kyopro_friends のツイートでLCA。
    // bit毎の計算のイメージ。辺をビット毎に整理すると0,1になる。: https://kanpurin.hatenablog.com/entry/2021/05/15/224226
    // Total 2h57min
    input! {
        n: usize,
    }
    let mut graph: Vec<Vec<(usize, usize)>> = vec![vec![]; n];
    for i in 0..n-1 {
        input!{
            ui: Usize1,
            vi: Usize1,
            wi: usize,
        }
        graph[ui].push((vi, wi));
        graph[vi].push((ui, wi));
    }

    // solve_botu(n, &graph);
    solve(n, &graph);
}

fn solve(n:usize, graph: &Vec<Vec<(usize, usize)>>) {

    let modulus = 1_000_000_007;

    let mut dp = vec![0; n];
    fn dfs(v: usize, p: usize, graph: &Vec<Vec<(usize, usize)>>, dp: &mut Vec<usize>) {
        for i in 0..graph[v].len() {
            let (nv, nw) = graph[v][i];
            if nv == p {continue}
            dp[nv] = dp[v] ^ nw;
            dfs(nv, v, graph, dp);
        }
    }
    dfs(0,0, &graph, &mut dp);
    let mut cum = vec![vec![vec![0; 2]; 64]; n];
    for i in 0..n {
        for b in 0..64 {
            if (1 << b) & dp[i] != 0 {
                cum[i][b][1] = 1;
            }
            else {
                cum[i][b][0] = 1;
            }
        }

        if i == 0 {continue}
        for b in 0..64 {
            cum[i][b][0] += cum[i-1][b][0];
            cum[i][b][1] += cum[i-1][b][1];
        }
    }
    
    let mut ans = 0;
    for i in 1..n {
        for b in 0..64 {
            if dp[i] & (1 << b) != 0 {
                ans += cum[i-1][b][0] * ((1_usize << b) % modulus);
                ans %= modulus;
            }
            else {
                ans += cum[i-1][b][1] * ((1_usize << b) % modulus);
                ans %= modulus;
            }
        }
    }
    println!("{}", ans);

}

// fn solve_botu(n: usize, graph: &Vec<Vec<(usize, usize)>>) {
//     let modulus = 1_000_000_007;
//     // エッジを何回通るか、という問題に帰着

//     // oooox-xooooo
//     let dp = get_subtree_sizes(0, &graph);

//     // let mut dp2 = vec![vec![vec![0; 2]; 64]; n];
//     // for b in 0..64 {
//     //     dp2[0][b][0] = dp[0][b][0];
//     //     dp2[0][b][1] = dp[0][b][1];
//     // }

//     let mut ans: usize = 0;
//     let mut nums = vec![vec![0; 2]; 64];
//     dfs(0,  0,  &graph, &dp, &mut ans, nums);
//     println!("{}", ans);

// }

// fn dfs(v: usize, parent: usize, graph: &Vec<Vec<(usize, usize)>>, dp: &Vec<Vec<Vec<usize>>>, ans: &mut usize, mut nums: Vec<Vec<usize>>) {
//     // 問題によって、枝の本数なども数えた方がいい。
//     let modulus = 1_000_000_007;
//     let n = graph.len();

//     for i in 0..graph[v].len() {
//         let (nv, nw) = graph[v][i];
//         if nv == parent {continue}
//         for b in 0..64 {
//             let mut num0 = 0;
//             let mut num1 = 0;
//             if nw & (1 << b) != 0 {
//                 num0 = dp[nv][b][1];
//                 num1 = dp[nv][b][0]
//             }
//             else {
//                 num0 = dp[nv][b][0];
//                 num1 = dp[nv][b][1];
//             }
//             nums[b][0] += num0;
//             nums[b][1] += num1;

//             nums[b][0] % modulus;
//             nums[b][1] % modulus;
//         }
//     }

//     // println!("v = {:?}", v);
//     println!("nums = {:?}", nums);
//     // for i in 0..n {
//     //     println!("nums[{i}] = {:?}", nums[i]);
//     // }


//     for i in 0..graph[v].len() {
//         let (nv, nw) = graph[v][i];
//         if nv == parent {continue}

//         let mut n_nums = vec![vec![0; 2]; 64];
//         for b in 0..64 {

//             let mut num0 = 0;
//             let mut num1 = 0;
//             if nw & (1 << b) != 0 {
//                 num0 = dp[nv][b][1];
//                 num1 = dp[nv][b][0];

//                 n_nums[b][0] = nums[b][1] - num1;
//                 n_nums[b][1] = nums[b][0] - num0;
//                 n_nums[b][1] += 1;
//             }
//             else {
//                 num0 = dp[nv][b][0];
//                 num1 = dp[nv][b][1];

//                 n_nums[b][0] = nums[b][0] - num0;
//                 n_nums[b][1] = nums[b][1] - num1;
//                 n_nums[b][0] += 1;
//             }

//             let count = (nums[b][0] - num0) * num1;
//             let count2 = (nums[b][1] - num1) * num0;

//             let diff = count * ((1 << b) % modulus) % modulus;
//             let diff2 = count2 * ((1 << b) % modulus) % modulus;

//             if b <= 1 {
//                 println!("v = {:?}", v);
//                 println!("diff = {}, diff2 = {:?} diff + diff2 = {}", diff, diff2, diff + diff2);
//             }
//             *ans += diff + diff2;
//             *ans %= modulus;

//         }
//         dfs(nv, v, graph, dp, ans, n_nums);
//     }
// }


// /// 各頂点の「部分木の大きさのリストを返す関数
// /// root_v := 木の根となる頂点
// /// 木DP は、葉から根に向かって計算するDP。問題によっては、根の値 dp[root] だけじゃなく、全頂点で、ans = max(ans, dp[v]) をチェックする必要がある。
// fn get_subtree_sizes(root_v: usize, graph: &Vec<Vec<(usize, usize)>>) -> Vec<Vec<Vec<usize>>> {
//     // subtree_sizes[v] := 頂点vを根とする、部分木の大きさ(頂点数)
//     let n = graph.len();
//     let mut dp = vec![vec![vec![0;2]; 64]; n];
//     dfs_1(root_v, root_v, &graph, &mut dp);
//     return dp
// }
// fn dfs_1(v: usize, parent: usize, graph: &Vec<Vec<(usize, usize)>>, dp: &mut Vec<Vec<Vec<usize>>>) {
//     // 問題によって、枝の本数なども数えた方がいい。
//     let modulus = 1_000_000_007;
//     let n = graph.len();
//     for i in 0..graph[v].len() {
//         let (nv, nw) = graph[v][i];
//         if nv == parent {continue}
//         dfs_1(nv, v, graph, dp);
//         for b in 0..64 {

//             let ind0 = if (0 ^ (nw & 1 << b) != 0) {1} else {0};
//             let ind1 = if (1 ^ (nw & 1 << b) != 0) {1} else {0};
//             dp[v][b][ind0] += dp[nv][b][0];
//             dp[v][b][ind1] += dp[nv][b][1];
//             dp[v][b][0] %= modulus;
//             dp[v][b][1] %= modulus;
//         }
//     }
//     // 葉であった場合
//     if graph[v].len() == 1 && v != 0 {
//         for b in 0..64 {
//             dp[v][b][0] = 1;
//         }
//     }
// }

// // xxo-oxxx
// // [0,0,0,0]

// // dp[b][0] := b桁目が0のやつは何個か?
// // dp[b][1] := b桁目が1のやつは何個か?
