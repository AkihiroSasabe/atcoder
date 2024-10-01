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
    // 2024-09-30 20:16-21:09 (53min)
    input! {
        n: usize,
        a: [isize; 3*n],
    }
    

    // N個の要素を取り除く

    // 貪欲に考えるなら、
    // 前半で小さい奴を削り、
    // 後半で大きい奴を削る

    // 切れ目を設定する n-1の後ろ ~ 2*n-1の後ろ の好きな位置

    // 尺取法でいけそうな気がしてきた。
    // 前半
    let mut sum0 = 0;
    let mut btree0_in = BTreeMap::new();
    // let mut btree0_out = BTreeMap::new();
    for i in 0..n {
        sum0 += a[i];
        *btree0_in.entry(a[i]).or_insert(0) += 1;
    }


    // 後半
    let mut sum1 = 0;
    let mut btree1_in = BTreeMap::new();
    let mut btree1_out = BTreeMap::new();
    for i in n..3*n {
        *btree1_out.entry(a[i]).or_insert(0) += 1;
    }
    for _ in 0..n {
        // デカい奴を削っていく
        let (&k, &num) = btree1_out.iter().next().unwrap();
        btree1_out.subtract_value_by_1(k);
        btree1_in.add_value_by_1(k);
        sum1 += k;
    }

    let mut ans = sum0 - sum1;

    for i in n..2*n {
        btree0_in.add_value_by_1(a[i]);
        
        // 0から、捨てる奴
        let (&k, &num) = btree0_in.iter().next().unwrap();
        btree0_in.subtract_value_by_1(k);

        sum0 += a[i] - k;

        // 1 から a[i] を抜かないといけない
        let (in_max, _) = btree1_in.iter().rev().next().unwrap();
        let (out_min, _) = btree1_out.iter().next().unwrap();

        if btree1_in.contains_key(&a[i]) {
            // 1の上位N個に、a[i]が入っている?
            btree1_in.subtract_value_by_1(a[i]);
            let (&out_min, _) = btree1_out.iter().next().unwrap();
            btree1_out.subtract_value_by_1(out_min);
            btree1_in.add_value_by_1(out_min);
            sum1 += out_min - a[i]

        }
        else {
            // 元から入っていない?
            btree1_out.subtract_value_by_1(a[i]);
        }

        ans = max(ans, sum0 - sum1);
    }
    println!("{}", ans);





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