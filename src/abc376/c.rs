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
    input! {
        n: usize,
        mut a: [usize; n], // おもちゃ
        b: [usize; n-1], // はこ
    }
    a.sort();
    a.reverse();
    // btree := 箱を格納
    let mut btree = BTreeMap::new();
    for i in 0..n-1 {
        btree.add_value_by_1(b[i]);
    }

    let mut count = 0; // 現状の箱で、収容できないおもちゃの数
    let mut ans = 0;
    for i in 0..n {
        // 貪欲に、一番大きなおもちゃを、収容可能な最小の箱を使って、順に詰めていく。
        if let Some((&k2, _)) = btree.range(a[i]..).next() {
            // 収容できるとき
            btree.subtract_value_by_1(k2);
        }
        else {
            // 収容できないとき
            count += 1;
            ans = a[i];
        }
    }

    if count == 1 {
        println!("{}", ans);
    }
    else {
        // 収容できない箱が2個以上あった場合は無理。
        println!("-1");
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