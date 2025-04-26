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
    // 2025-04-10 20:37-21:54 (1h17min)
    // 2025-04-11 12:26-12:45 (19min)
    // 2025-04-11 19:13-20:28 (1h15min, upsolve)
    // Total: 2h51min
    input! {
        n: usize,
        x: usize,
        a: [usize; n],
    }
    // dp[i] := i番目までの硬貨で、{支払うべき金額: 必要な硬貨の枚数の最小値}
    let mut dp: Vec<BTreeMap<usize, usize>> = vec![BTreeMap::new(); n];

    // dfs(x,d) := {a[0], a[1], ..., a[n-1-d]} の硬貨を使って、x円を支払う時に必要な硬貨の枚数の最小値を取得する関数
    let ans = dfs(x, 0, &mut dp, &a);
    println!("{}", ans);
/*
~~~ 思考実験の過程(1) ~~~

x=396円なら?
支払い: 4x100円 + 1x5円 + 1x1円
お釣り: 1x10円
で4+1+1+1=7枚 が最小。

支払い: 4x100円
お釣り: 4x1円
だと4+4=8枚

~~~ 思考実験の過程(2) ~~~
// 一番大きな硬貨から考える。
[1]過剰に払って、お釣りを、小さな硬貨で貰う
[2]ギリギリ超えない範囲で払って、残りを、小さな硬貨で払う
お釣りを貰うことと、不足金 を払うことは、必要なコインの枚数を計算する上では等価。

実は、a[i]で支払い後の、 不足金 と お釣り は、x % a[i] か a[i] - (x % a[i]) の2択しかないことが味噌。 
なぜなら問題文に、a[i+1] % a[i] == 0 と書いてあるため、
a[i]より大きな硬貨で何枚払おうが、a[i]による剰余は等しいためである。つまり、
(x - (num[i+1] * a[i+1] + num[i+2] * a[i+2] + ... + num[n-1] * a[n-1])) % a[i] == x % a[i] だからである。
なので、2N通りの状態しかないので、メモ化再帰すれば、O(N)で終わる

実際にシミュレーションしてみる。
◆a[3]=100円のとき
x=396円
4x100円 -> nx= 4円 (お釣り。後続で4円分だけ用意できればよい。)
3x100円 -> nx=96円 (不足金。後続で96円分だけ用意できればよい。)

◆a[2]=10円のとき
x=4円
1x10円 -> nx=6円 (お釣り)
0x10円 -> nx=4円 (不足金)
x=96円
10x10円 -> nx=4円 (お釣り)
9x10円 -> nx=6円 (不足金)

◆a[1]=5円のとき
x=4円
1x5円 -> nx=1円 (お釣り)
0x5円 -> nx=4円 (不足金)

x=6円
2x5円 -> nx=4円 (お釣り)
1x5円 -> nx=1円 (不足金)

◆a[0]=1円のとき
x=4円
4x1円 -> nx=0

x=1円
1x1円 -> nx=0
*/
}


// dfs(x,d) := {a[0], a[1], ..., a[n-1-d]} の硬貨を使って、x円を支払う時に必要な硬貨の枚数の最小値を取得する関数
// dp[i] := i番目までの硬貨で、{支払うべき金額: 硬貨の枚数の最小値}
fn dfs(x: usize, d: usize, dp: &mut Vec<BTreeMap<usize, usize>>, a: &Vec<usize>) -> usize {
    let n = dp.len();
    let i = n - 1 - d;
    if d == n - 1 {
        // a[0] = 1 なので、x円ならx枚払う。
        dp[0].insert(x, x);
        return x;
    }
    // メモ化再帰の容量で、既に計算済みならキャッシュを使う。
    if let Some(val) = dp[i].get(&x) {
        return *val
    }

    // お釣りが出ないような払い方で必要な枚数を求める
    let num = x / a[i]; // a[i]の必要枚数
    let nx = x - a[i] * num; // a[i+1]以降で払うべき金額
    let mut cand = num + dfs(nx, d+1, dp, a);

    // お釣りを貰うような払い方で必要な枚数を求める    
    if x % a[i] != 0 {
        let nx2 = a[i] * (num+1) - x;
        cand = min(cand, num + 1 + dfs(nx2, d+1, dp, a));
    }
    dp[i].insert(x, cand);
    return cand
}