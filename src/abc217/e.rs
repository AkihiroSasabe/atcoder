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
    // 2024-02-03 13:07-13:34 (27min)
    input! {
        q: usize
    }
    let mut xs = vec![];
    let mut ks = vec![];
    for i in 0..q {
        input!{
            ki: usize,
        }
        ks.push(ki);
        if ki == 1 {
            input!{
                xi: usize
            }
            xs.push(xi);
        }
    }
    xs.reverse();
    // 1 x : 
    // A の最後尾に 
    // x を追加する。

    // 2 : 
    // A の最初の要素を出力する。その後、その要素を削除する。このクエリが与えられるとき、
    // A は空でないことが保証される。

    // 3 : 
    // A を昇順にソートする。

    // let mut heap = BinaryHeap::new();
    let mut btree: BTreeMap<usize, usize> = BTreeMap::new();
    let mut deque = VecDeque::new();
    for i in 0..q {
        if ks[i] == 1 {
            deque.push_back(xs.pop().unwrap());
        }
        else if ks[i] == 2 {
            if btree.len() == 0 {
                let v = deque.pop_front().unwrap();
                println!("{}", v);
            }
            else {
                let (&v, &num) = btree.iter().next().unwrap();
                btree.subtract_value_by_1(v);
                println!("{}", v);
            }
        }
        else if ks[i] == 3 {
            while deque.len() != 0 {
                let v = deque.pop_front().unwrap();
                btree.add_value_by_1(v);
            }
        }
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