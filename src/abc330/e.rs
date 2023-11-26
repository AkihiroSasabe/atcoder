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
    // 2023-11-25 22:24-22:40 (16min)
    // 2023-11-26 09:39-10:01 (22min)
    // total (38min)
    input! {
        n: usize,
        q: usize,
        mut a: [usize; n]
    }
    let mut indices = vec!{};
    let mut x = vec!{};
    for i in 0..q {
        input!{
            i_i: usize,
            x_i: usize,
        }
        indices.push(i_i-1);
        x.push(x_i);
    }
    
    // 数列Aに格納されている数字と、その個数
    let mut btree = BTreeMap::new();
    for i in 0..n {
        btree.add_value_by_1(a[i]);
        // *btree.entry(a[i]).or_insert(0_usize) += 1;
    }

    // 数列Aに格納されていない数字で、2*10^5 以下の数字を全て格納
    let mut mex_candidates = BTreeSet::new();
    for i in 0..200_003 {
        if !btree.contains_key(&i) {
            // *mex_candidates.entry(i).or_insert(0_usize) += 1;
            mex_candidates.insert(i);
        }
    }
    // 各クエリの実行
    for i in 0..q {
        // 数列Aの中身の置換 a[ind] = pre -> a[ind] = x
        let pre = a[indices[i]];
        a[indices[i]] = x[i];

        // btreeの更新
        // reduce_from_btree(&mut btree, pre);
        btree.subtract_value_by_1(pre);
        *btree.entry(x[i]).or_insert(0_usize) += 1;

        // mexの集合も更新
        if !btree.contains_key(&pre) {
            // aからpreが全て消滅したら、mexの候補集合に pre を追加
            // *mex_candidates.entry(pre).or_insert(0_usize) += 1;
            mex_candidates.insert(pre);
        }
        // reduce_from_btree(&mut mex_candidates, x[i]);
        mex_candidates.remove(&x[i]);

        let mex = mex_candidates.iter().next().unwrap();
        println!("{}", mex);
    }
}


// B木から要素xを1個減らす関数
fn reduce_from_btree<T: std::cmp::Ord>(b_tree: &mut BTreeMap<T, usize>, x: T) {
    // b_tree := {key: 要素, value: 格納した要素の個数}
    // B木にxが存在しないなら何もしない
    if !b_tree.contains_key(&x) {return}
    if b_tree[&x] == 1 {
        // B木のxの個数が1個ならxのkeyごと消去する
        b_tree.remove(&x);
    }
    else {
        // B木のxの個数を1個減らす
        *b_tree.get_mut(&x).unwrap() -= 1;
    }
}

/// BTreeMapの value を操作するトレイト
/// b_tree := {key: 要素, value: 格納した要素の個数} をイメージ
pub trait ValueManipulator {
    fn subtract_value(&mut self, key: Self::Key, value: usize);
    fn subtract_value_by_1(&mut self, key: Self::Key);
    fn add_value(&mut self, key: Self::Key, value: usize);
    fn add_value_by_1(&mut self, key: Self::Key);

    type Key; // 関連型（ Associated Type ）を宣言
}
impl<T: std::cmp::Ord> ValueManipulator for BTreeMap<T, usize> {
    fn subtract_value(&mut self, key: T, value: usize) {
        // B木からkeyに対応する値を減算する

        // B木に key が存在しないなら何もしない
        if !self.contains_key(&key) {return}
        if self[&key] <= value {
            // B木のkeyの個数がvalue個以下なら key ごと消去する
            self.remove(&key);
        }
        else {
            // B木のkeyの個数をvalue個減らす
            *self.get_mut(&key).unwrap() -= value;
        }
    }
    fn subtract_value_by_1(&mut self, key: T) {
        // B木からkeyに対応する値を1だけ減算する
        self.subtract_value(key, 1);
    }
    fn add_value(&mut self, key: T, value: usize) {
        // keyに対応する値を加算する
        *self.entry(key).or_insert(0_usize) += value;
    }
    fn add_value_by_1(&mut self, key: T) {
        // keyに対応する値を1だけ加算する
        self.add_value(key, 1);
    }
    // たぶん型エイリアスを定義 https://rs.nkmk.me/rust-type-alias/
    type Key = T; // トレイト ValueManipulator 内での Key という関連型を具体的な型 T に紐付けるための宣言
}