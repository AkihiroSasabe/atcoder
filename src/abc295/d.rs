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
    input! {
        s: Chars
    }
    // 考察
    // 入力例1: 20230322
    // (L,R) = (1,6),(1,8),(2,7),(7,8) で 合計4つ

    // (L, R)が条件を満たす時、下の表のindexがL-1とRの0-9の列で、各数字の累積登場回数の偶奇のパターンが同じ場合であることに気づく
    // よって、各数字の累積登場回数の偶奇のパターンをbitで表現し、
    // 同一パターンの列の個数をnとすると、nC2 がそのパターンのときに条件を満たす(L,R)の組み合わせの個数となる
    // 後は、全パターンについての組み合わせ個数の和が答えとなる。
    // index->   init  1   2   3   4   5   6   7   8
    // s[index]->      2   0   2   3   0   3   2   2
    // V[0-9]      V[Cumulative frequency of appearance]
    // 0           0   0   1   1   1   2   2   2   2
    // 1           0   0   0   0   0   0   0   0   0   
    // 2           0   1   1   2   2   2   2   3   4
    // 3           0   0   0   0   1   1   2   2   2
    // 4           0   0   0   0   0   0   0   0   0
    // 5           0   0   0   0   0   0   0   0   0
    // 6           0   0   0   0   0   0   0   0   0
    // 7           0   0   0   0   0   0   0   0   0
    // 8           0   0   0   0   0   0   0   0   0
    // 9           0   0   0   0   0   0   0   0   0

    // 偶数個ずつ

    // 現在の状態 (0-9の偶奇で10ビット)
    // 例えば
    //(0123456789)
    // 0000000000: 全部偶数個 (初期状態は全部0個なのでこれ。)
    // 0000000001: 0が奇数個で, 他が全部偶数個
    // 0000000010: 1が奇数個で, 他が全部偶数個
    // 0000000101: 0と2が奇数個で、他が全部偶数個
    let mut bit = 0;

    // 各桁の状態
    let mut hash = HashMap::new();

    // key: 状態, value: その状態の登場数
    hash.insert(0_usize, 1_usize);

    // println!("bit:{}", bit);

    for i in 0..s.len() {
        let num: usize = s[i] as usize - 48;
        // ^は排他的論理和XOR
        bit = bit ^ (1 << num);
        // println!("bit:{}", bit);
        if hash.contains_key(&bit) {
            *hash.get_mut(&bit).unwrap() += 1;
        }
        else {
            hash.insert(bit, 1);
        }
    }

    let mut ans = 0;
    for (k, v) in hash.iter() {
        // nC2
        ans += (*v * (*v-1)) / 2;
    }
    println!("{}", ans);



}