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
        m: usize,
    }
    let mut l = vec![];
    let mut r = vec![];
    let mut rl_set = BTreeMap::new();
    let mut l_set = BTreeMap::new();
    for i in 0..n {
        input!{
            li: usize,
            ri: usize,
        }
        l.push(li);
        r.push(ri);
        rl_set.add_value_by_1((ri, li));
        l_set.entry(li).or_insert(vec![]).push(ri);
    }
    let mut ans = 0;
    for x in 1..m+1 {
        // println!("x = {:?} ----", x);
        let mut diff: usize = 0;
        let mut max_right = m;
        if let Some((rli, num)) = rl_set.iter().next() {
            let ri = rli.0;
            // println!("ri = {:?}", ri);
            max_right = ri - 1;
        }
        // else {
        //     diff = 1 + m - x;
        // }
        diff = 1 + max_right - x;
        // println!("diff = {:?}", diff);
        ans += diff;

        if l_set.contains_key(&x) {
            let vec = l_set.get(&x).unwrap();
            for &ri in vec {
                rl_set.subtract_value_by_1((ri, x));
            }
            l_set.remove(&x);
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