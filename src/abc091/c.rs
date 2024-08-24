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
    // 2024-08-23 21:04-21:15 (11min)
    // 2024-08-24 15:33-16:08 (35min)
    // 46min
    input! {
        n: usize
    }
    let mut a = vec![];
    let mut b = vec![];
    // let mut ab: Vec<Vec<usize>> = vec![vec![0; 2*n]; 2*n];
    let mut ab = vec![vec![]; 2*n];
    for i in 0..n {
        input! {
            ai: usize,
            bi: usize,
        }
        a.push(ai);
        b.push(bi);
        ab[ai].push(bi);
        // ab[ai][bi] += 1;
    }

    let mut c = vec![];
    let mut d = vec![];
    let mut cd = vec![vec![]; 2*n];
    for i in 0..n {
        input! {
            ci: usize,
            di: usize,
        }
        c.push(ci);
        d.push(di);
        cd[ci].push(di);
    }

    for i in 0..2*n {
        ab[i].sort();
        cd[i].sort();
    }

    let mut ans: usize = 0;
    let mut btree = BTreeMap::new();    
    for i in 0..2*n {
        for &bi in ab[i].iter() {
            *btree.entry(bi).or_insert(0) += 1;
        }
        for &di in cd[i].iter() {
            if let Some((&ki, &vi)) = btree.range(..di).rev().next() {
                btree.subtract_value_by_1(ki);
                ans += 1;
            }
        }
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