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
    // 2025-09-07 20:30-21:00 (30min)
    // 2025-09-08 12:57-13:19 (22min)
    // 2025-09-09 14:54-17:00 (2h6m)
    // 2025-09-09 23:04-23:24 (20min, upsolve. 公式解説 と @momohara_kyopro のツイートをみた)
    // https://x.com/momohara_kyopro/status/1964568329408143546
    // Total 3h18m

    // 公式解説は以下のような、要素数2のタプルで頂点を表したグラフを考えている。
    // 頂点(d,u) -> 頂点(d-1,v) となるエッジからなるグラフ。(d,u)は、頂点uからd回移動するときの状態を表す。
    // 頂点数は、(N-1) * N個で、エッジの本数は、(N-1)*M 。
    // このグラフは、DAG(Directed Acyclic Graph)であるから、
    // 閉路を持たず、トポロジカルソート順に遷移すれば、DPの遷移が可能。
    // エッジの向きは常に d -> d-1 だから、dを降順に遷移するだけでいい。
    // DAGではない場合は、閉路を持つため、状態の更新（緩和）の順序が一定じゃなくなるので、DPの遷移ができなくなる。

    input! {
        n: usize,
        m: usize,
        w: [usize; n],
    }
    let mut u = vec![];
    let mut v = vec![];
    let mut g = vec![vec![]; n]; // 隣接リスト
    for i in 0..m {
        input!{
            ui: Usize1,
            vi: Usize1,
        }
        u.push(ui);
        v.push(vi);
        g[ui].push(vi);
        g[vi].push(ui);
    }

    // 5000 * 5000 = 25_000_000 = 2.5 * 10^7
    // 燃料, 体重


    let inf: usize = 1 << 60;
    // dp[d][v] := 頂点vからd回移動するときに必要な燃料の最小値
    // もう少し詳しく説明すると、頂点0から適当な経路(0, va, vb, ..., v)で頂点vに移動し、残りd回移動するときに、必要な燃料の最小値。
    // 例えば、0から頂点vに移動するのに、L回かかったとすると、以下のようになる。
    // dp[d][v] := min(dp[d][v], w[0] * (d+L) + w[va] * (d+L-1) + ... + w[v] * d)
    let mut dp = vec![vec![inf; n]; n];

    // 初期化: 頂点0 から、d回移動する場合、燃料は d * w[0] 必要
    for d in 0..n {
        dp[d][0] = d * w[0];
    }
    // トポロジカルソート順(dの降順)に遷移
    for d in (1..n).rev() {
        // println!("dp[{}] = {:?}", d, dp[d]); // debug
        for i in 0..m {
            let ui = u[i];
            let vi = v[i];
            dp[d-1][ui] = min(dp[d-1][ui], dp[d][vi] + w[ui] * (d-1));
            dp[d-1][vi] = min(dp[d-1][vi], dp[d][ui] + w[vi] * (d-1));
        }
    }
    for v in 0..n {
        // 頂点vから0回移動するとき -> 頂点0から頂点vへの最小燃料
        println!("{}", dp[0][v]);
    }

}
