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
use rand::Rng;
fn main() {
    // 2024-09-02 19:40-20:47 (1h7min)
    input! {
        n: usize,
        m: usize,
    }
    let INF = 1 << 60;
    let mut graph = vec![vec![INF; n]; n];
    let mut brides = vec![];
    for i in 0..m {
        input! {
            ui: usize,
            vi: usize,
            ti: isize,
        }
        graph[ui-1][vi-1] = min(graph[ui-1][vi-1], ti);
        graph[vi-1][ui-1] = min(graph[vi-1][ui-1], ti);
        brides.push((ui-1,vi-1,ti));
    }
    input! {
        q: usize,
    }
    let mut k = vec![];
    let mut b = vec![];
    for i in 0..q {
        input! {
            ki: usize,
            bi: [usize; ki],
        }
        k.push(ki);
        b.push(bi);
    }

    let dp = floyd_warshall(n, &graph);


    // println!("graph = ");
    // for i in 0..n {
    //     for j in 0..n {
    //         print!("{} ", graph[i][j]);
    //     }
    //     println!("");
    // }

    // println!("dp = ");
    // for i in 0..n {
    //     for j in 0..n {
    //         print!("{} ", dp[i][j]);
    //     }
    //     println!("");
    // }

    for i in 0..q {
        let mut ans = 1 << 60;
        for perm in (0..k[i]).permutations(k[i]) {
            // println!("perm = {:?}", perm);
            for bit in 0..1<<k[i] {
                let mut vs = 0;
                let mut cand = 0;
                let mut last_bt = 0;
                for (ind, &p) in perm.iter().enumerate() {

                    let mut bs = brides[b[i][p]-1].0;
                    let mut bt = brides[b[i][p]-1].1;
                    if bit & (1 << ind) != 0 {
                        swap(&mut bs, &mut bt);
                    }

                    let tt = brides[b[i][p]-1].2;
                    let cont1= dp[vs][bs];
                    let cont2 = tt as usize;
                    // println!("cont1 = {:?}", cont1);
                    // println!("cont2 = {:?}", cont2);
                    vs = bt;
                    last_bt = bt;
                    cand += cont1 + cont2;
                }
                let cont3= dp[last_bt][n-1];
                cand += cont3;
                ans = min(ans, cand);
            }

            // println!("cont3 = {:?}", cont3);
            // println!("cand = {:?}", cand);
        }
        println!("{}", ans);
    }

    // 橋を5本渡る。
    // 5 * 4 * 3 * 2 = 120 通り



    // 64_000_000
}

// // フロイド・ワーシャル法で、全頂点対間の距離をO(V^3)で最小化 (全点対間最短経路問題)
// fn floyd_warshall(graph: &Vec<Vec<Vec<usize>>>) -> Vec<Vec<usize>> {
//     // 頂点数
//     let n = graph.len();
//     // dp[i][j]で頂点iから頂点jに行くときの最短距離
//     let INF = std::usize::MAX;
//     let mut dp = vec![vec![INF; n]; n];

//     // dpの初期化
//     for i in 0..n {
//         dp[i][i] = 0;
//         for j in 0..graph[i].len() {
//             dp[i][graph[i][j][0]] = graph[i][j][1];
//         }
//     }
    
//     for k in 0..n {
//         for i in 0..n {
//             for j in 0..n {
//                 // k未満の頂点(0-k-1)のみを、中継点として通って良い。
//                 dp[i][j] = min(dp[i][j], dp[i][k] + dp[k][j]);
//                 // 例 k = 1の時
//                 // dp[0][0] = min(dp[0][0], dp[0][1] + dp[1][0]);
//                 // dp[0][1] = min(dp[0][1], dp[0][1] + dp[1][1]);
//                 // dp[0][2] = min(dp[0][2], dp[0][1] + dp[1][2]);
//                 // dp[0][3] = min(dp[0][3], dp[0][1] + dp[1][3]);
//                 // dp[0][4] = min(dp[0][4], dp[0][1] + dp[1][4]);
//             }
//         }
//     }
//     return dp
// }



// フロイド・ワーシャル法で、全頂点対間の距離をO(V^3)で最小化 (全点対間最短経路問題)
// この問題を解く用にアレンジされているのでabc257_Dを参考: https://github.com/AkihiroSasabe/atcoder/blob/main/src/abc257/d.rs: 
fn floyd_warshall(n: usize, graph: &Vec<Vec<isize>>) -> Vec<Vec<usize>> {
    // dp[i][j]で頂点iから頂点jに行くときの最短距離
    let mut dp = vec![vec![0; n]; n];


    // dpの初期化
    for i in 0..n {
        for j in 0..n {
            if i == j {
                dp[i][j] = 0;
            }
            else {
                dp[i][j] = graph[i][j] as usize;
            }
        }
    }
    
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                // k未満の頂点(0-k-1)のみを、中継点として通って良い。
                dp[i][j] = min(dp[i][j], dp[i][k] + dp[k][j]);
                // 例
                // dp[0][2] = min(dp[0][2], dp[0][1] + dp[1][2]);
                // dp[0][3] = min(dp[0][3], dp[0][1] + dp[1][3]);
                // dp[0][4] = min(dp[0][4], dp[0][1] + dp[1][4]);
            }
        }
    }

    return dp
}
