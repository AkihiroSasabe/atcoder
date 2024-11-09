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
    // 2024-11-09 14:37-15:00 (23min)
    input! {
        t: usize
    }
    for i in 0..t {
        input! {
            ni: usize,
            ki: usize,
            ai: [usize; ni],
            bi: [usize; ni],
        }
        let mut ab = vec![];
        for i in 0..ni {
            ab.push((ai[i], bi[i]));
        }
        ab.sort();
        // ab.reverse();
        // println!("ab = {:?}", ab);
        let mut bsum = 0;
        let mut btree = BTreeMap::new();
        for i in 0..ki {
            btree.add_value_by_1(ab[i].1);
            bsum += ab[i].1;
        }
        let mut ans = ab[ki-1].0 * bsum;
        for i in ki..ni {
            let (&out, _) = btree.iter().rev().next().unwrap();
            let cand = ab[i].0 * (bsum + ab[i].1 - out);
            ans = min(ans, cand);
            if ab[i].1 < out {
                bsum = bsum + ab[i].1 - out;
                btree.add_value_by_1(ab[i].1);
                btree.subtract_value_by_1(out);
            }
        }
        println!("{}", ans);
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