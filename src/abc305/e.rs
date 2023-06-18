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
use std::collections::HashSet;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    // 公式解説の方針通りに実装 https://atcoder.jp/contests/abc305/editorial/6539
    // @kyopro_friendsの解法も分かりやすいとは思う。(本実装とは異なる) https://twitter.com/kyopro_friends/status/1667530762693021705/photo/1
    // ダイクストラ法とN回のループで収束
    input! {
        n: usize,
        m: usize,
        k: usize,
    }
    let mut graph = vec![vec![]; n];
    for i in 0..m {
        input! {
            a_i: usize,
            b_i: usize,
        }
        graph[a_i - 1].push(b_i - 1);
        graph[b_i - 1].push(a_i - 1);
    }
    let mut position = vec![];
    let mut hp = vec![];
    for i in 0..k {
        input! {
            p_i: usize,
            h_i: isize,
        }
        position.push(p_i-1);
        hp.push(h_i);
    }

    // potential[i] := 頂点iに辿り着いた警備兵の体力の最大値 (ただし警備員がたどり着けない場合は −1)。
    let mut potential = vec![-1; n]; // 初め-1で初期化。確定したら-1以外の数字が割り当てられる。

    // 暫定的なpotentialを格納するheap。
    // <=> potentialが未確定な頂点i (<=> potential[i] == -1)について、その頂点にたどり着ける警備兵の暫定的な[体力, 頂点番号]を要素に持つheap
    let mut heap: BinaryHeap<_> = BinaryHeap::new();
    // 警備兵が駐在する場所における体力で、heapを初期化。
    for i in 0..k {
        let v = position[i];
        heap.push(vec![hp[i], v as isize]);
    }

    // ダイクストラ法のように頂点の個数(n個)と同じ数だけループ(n回)を回せば収束する。
    for iteration in 0..n {
        // println!("---- iteration={} ----", iteration);
        // println!("heap={:?}", heap);

        // potentialが未確定な頂点の中で、暫定的なHPが最大のものを選んで、確定させる
        while heap.len() != 0 {
            let max_element = heap.pop().unwrap();
            let max_hp = max_element[0];
            let max_v = max_element[1] as usize;
            // 未確定な頂点である場合のみ、その頂点を確定させる
            if potential[max_v] == -1 {
                // println!("confirm v={}, hp={}", max_v, max_hp);
                potential[max_v] = max_hp;
                // 確定した頂点に隣接する頂点について、ポテンシャルが未確定であれば、仮のポテンシャルを更新する
                for i in 0..graph[max_v].len() {
                    let next_v = graph[max_v][i];
                    if potential[next_v] == -1 {
                        if max_hp - 1 < 0 {continue} // 警備兵の体力がもうない場合は追加せずスキップ
                        heap.push(vec![max_hp - 1, next_v as isize]);
                        // println!("add v={}, hp={}", next_v, max_hp-1);
                    }
                }
                break
            }
        }
    }
    
    let mut g = 0;
    let mut ans = vec![];
    for i in 0..potential.len() {
        if potential[i] != -1 {
            g += 1;
            ans.push(i+1);
        }
    }
    println!("{}", g);
    for v in ans {
        print!("{} ", v);
    }
}
