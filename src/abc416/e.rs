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
    // 2025-07-26 21:50-22:40 (50min)
    // 2025-07-27 15:10-16:11 (1h1)
    // 1h51min
    input! {
        n: usize,
        m: usize,
    }
    let mut a = vec![];
    let mut b = vec![];
    let mut c = vec![];
    for i in 0..m {
        input!{
            ai: Usize1,
            bi: Usize1,
            ci: usize,
        }
        a.push(ai);
        b.push(bi);
        c.push(2*ci);
    }
    input! {
        k: usize,
        t: usize,
        mut d: [Usize1; k],
        q: usize
    }
    let mut kinds = vec![];
    let mut xx = vec![];
    let mut yy = vec![];
    let mut tt = vec![];
    for i in 0..q {
        input!{
            kindsi: usize,
        }
        kinds.push(kindsi);
        if kindsi == 1 {
            input!{
                xxi: Usize1,
                yyi: Usize1,
                tti: usize,
            }
            xx.push(xxi);
            yy.push(yyi);
            tt.push(2*tti);
        }
        else if kindsi == 2 {
            input!{
                xxi: Usize1,
            }
            xx.push(xxi);
            yy.push(0);
            tt.push(0);
        }
        else if kindsi == 3 {
            xx.push(0);
            yy.push(0);
            tt.push(0);
        }
    }
    let mut g = vec![vec![]; n+1];
    for i in 0..m {
        g[a[i]].push((b[i], c[i]));
        g[b[i]].push((a[i], c[i]));
    }

    for i in 0..k {
        g[d[i]].push((n, t));
        g[n].push((d[i], t));
    }
    let mut dp = floyd_warshall(&g);

    let inf = usize::MAX / 10;
    let mut ans =  0;
    for i in 0..n {
        for j in 0..n {
            if dp[i][j] == inf {
                continue;
            }
            ans += dp[i][j];
        }
    }

    // debug
    // for i in 0..n+1 {
    //     println!("dp[{i}] = {:?}", dp[i]);
    // }

    for i in 0..q {
        let kind = kinds[i];
        let xi = xx[i];
        let mut yi = yy[i];
        let mut ti = tt[i];
        if kind == 1 || kind == 2{
            if kind == 2 {
                yi = n;
                ti = t;
            }
            if dp[xi][yi] <= ti {
                continue
            }
            let pre = dp[xi][yi];

            if kind == 1 {
                if pre == inf {
                    ans += 2*ti;
                }
                else {
                    ans -= 2*(pre - ti);
                }
            }

            dp[xi][yi] = ti;
            dp[yi][xi] = ti;

            for jj in 0..(n+1) {
                for kk in jj+1..(n+1) {
                    let cand0 = dp[jj][xi] + ti + dp[yi][kk];
                    let cand1 = dp[jj][yi] + ti + dp[xi][kk];
                    let cand = min(cand0, cand1);
                    if cand >= inf {continue}
                    if dp[jj][kk] == inf {
                        if kk != n {
                            ans += 2 * cand;
                        }
                        dp[jj][kk] = cand;
                        dp[kk][jj] = cand;
                    }
                    else if dp[jj][kk] > cand {
                        if kk != n {
                            ans -= 2 * (dp[jj][kk] - cand);
                        }                            
                        dp[jj][kk] = cand;
                        dp[kk][jj] = cand;
                    }
                }
            }
        }
        else if kind == 3 {
            println!("{}", ans /2);
        }
    }

}


// 類題: abc73_D: https://atcoder.jp/contests/abc073/tasks/abc073_d
// 類題: abc257_D: https://github.com/AkihiroSasabe/atcoder/blob/main/src/abc257/d.rs
// フロイド・ワーシャル法で、全頂点対間の距離をO(V^3)で最小化 (全点対間最短経路問題)
fn floyd_warshall<T>(graph: &Vec<Vec<(usize, T)>>) -> Vec<Vec<T>> 
    where T: 
        Copy + 
        Ord +
        std::cmp::PartialEq + 
        std::ops::Div<Output = T> +
        num::Zero +
        num::One +
        num::Bounded // max_value() で要る
{
    // 頂点数
    let n = graph.len();

    // 初期化のために、任意の型に対応した、 0 と max / 2 が必要。
    let zero: T     = T::zero();
    let one: T      = T::one();
    let two: T      = one + one;
    let ten: T      = two + two + two + two + two;
    let inf: T      = T::max_value() / ten;
    // let INF: usize = usize::MAX / 10;


    // dp[i][j]で頂点iから頂点jに行くときの最短距離
    let mut dp: Vec<Vec<T>> = vec![vec![inf; n]; n];

    // dpの初期化
    for v in 0..n {
        // 同一頂点への移動は0
        dp[v][v] = zero;
        for i in 0..graph[v].len() {
            // 直接遷移可能な頂点への移動を格納
            let nv = graph[v][i].0;
            let dist = graph[v][i].1;
            dp[v][nv] = dp[v][nv].min(dist);
        }
    }
    
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                // dp[i][j] := i -> j へ、k未満の頂点(0 ~ k-1)のみを、中継点として通って良い。
                dp[i][j] = dp[i][j].min(dp[i][k] + dp[k][j]);
                // 例 k = 1の時
                // dp[0][0] = min(dp[0][0], dp[0][1] + dp[1][0]);
                // dp[0][1] = min(dp[0][1], dp[0][1] + dp[1][1]);
                // dp[0][2] = min(dp[0][2], dp[0][1] + dp[1][2]);
                // dp[0][3] = min(dp[0][3], dp[0][1] + dp[1][3]);
                // dp[0][4] = min(dp[0][4], dp[0][1] + dp[1][4]);
            }
        }
    }
    return dp
}

