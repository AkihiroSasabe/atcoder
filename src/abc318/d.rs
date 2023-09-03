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

    input! {
        n: usize,
    }
    let mut graph = vec![vec![0; n]; n];
    for i in 0..n-1 {
        for j in i+1..n {
            input!{
                d_i_j: isize
            }
            graph[i][j] = d_i_j;
            graph[j][i] = d_i_j;
        }
    }
    let mut remain = vec![];
    for i in 0..n {
        remain.push(i);
    }

    let mut ans = 0;
    // dp[bits] := バイナリ bits のうち、1である bit が選択済みとなっているときの、選んだ辺の重みの総和の最大値
    let mut dp = vec![0; 1 << n];
    let over_n = n + 3;

    // 選べる辺の組み合わせの総数は、15 * 13 * 11 * 9 * 7 * 5 * 3 * 1 = 2,027,025通り <- この全探索のコードは書くのが大変過ぎた...
    // bitDPの計算量は、O(n * 2^n) = O(16 * 2^16) = O(1,048,576)
    for bits in 0..(1 << n) {
        // println!(" --- dp[{:3b}] = {} --- ", bits, dp[bits]);
        // 新たに追加するエッジ(min_v, max_v)を選択する。
        // つまり、まだ立っていないビットの中から、2つのビット(min_v, max_v)を選ぶ。

        // 追加するビットのうち、小さい方のビット min_v を選ぶ
        let mut min_v = over_n; // 存在しないビットで初期化
        for i in 0..n {
            // 右からi番目のビットが立っていないとき、
            if ((bits >> i) & 1) == 0 {
                min_v = i;
                break
            }
        }
        // min_vが初期化時のまま
        if min_v == over_n {continue}

        // 追加するビットのうち、大きい方のビット max_v を選ぶ
        for max_v in min_v+1..n {
            // 右から max_v 番目のビットが立っていないとき
            if ((bits >> max_v) & 1) == 0 {
                let next_bits = bits | (1 << min_v) | (1 << max_v);
                // println!("next_bits = {:3b}", next_bits);
                dp[next_bits] = max(dp[next_bits], dp[bits] + graph[min_v][max_v]);
            } 
        }
    }


    println!("{}", dp[dp.len()-1]); // dp[2^n-1]

}