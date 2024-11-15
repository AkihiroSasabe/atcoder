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
    // 2024-11-10 11:27-12:04 (37min)
    // 12:04-12:50 (46min)
    // Total: 83min
    input! {
        n: usize,
        m: usize,
        mut x: [usize; m], // basho
        a: [usize; m], // kosu
    }
    for i in 0..m {
        x[i] -= 1;
    }

    let mut xa = vec![];
    for i in 0..m {
        xa.push((x[i], a[i]));
    }
    xa.sort();

    if xa[0].0 != 0 {
        println!("-1");
        // println!("aaa");
        return
    }
    let mut pos = n - 1;
    let mut ans = 0;
    for i in (0..m).rev() {
        // println!("i = {:?} ******", i);
        if pos < xa[i].0 {
            if xa[i].1 != 1 {
                println!("-1");
                // println!("bbb");
                return;
            }
            else {
                continue
            }
        }
        else if pos == xa[i].0 {
            if xa[i].1 != 1 {
                println!("-1");
                // println!("ccc");
                return;
            }
            else {
                if pos == 0 {break}
                pos = pos - 1;
                continue
            }
        }
        // くばる
        let diff = pos - xa[i].0;
        // let num = *btree.get(&xa[i].0).unwrap();
        let num = xa[i].1;
        let num_kubaru = (diff + 1) * diff / 2;
        // println!("num_kubaru = {:?}", num_kubaru);
        // println!("pos = {:?}", pos);
        if num == diff + 1 {
            ans += num_kubaru;
            if xa[i].0 == 0 {break}
            pos = xa[i].0 - 1;
        }
        else if num > diff + 1 {
            println!("-1");
            // println!("ddd");
            return;
        }
        else {
            ans += (diff + diff + 1 - num) * num / 2;
            if xa[i].0 == 0 {
                println!("-1");
                // println!("eee");
                return;
            }
            pos = pos - num;
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