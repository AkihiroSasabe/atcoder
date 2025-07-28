#![allow(dead_code, unused_imports)]
use proconio::{input, marker::{Usize1, Isize1, Chars}};
use itertools::Itertools;
use std::cmp::{max, min, Ordering, Reverse};
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::collections::{VecDeque, BinaryHeap, HashMap, BTreeMap, HashSet, BTreeSet};
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
use rand::Rng;
use num::{BigUint, ToPrimitive, Integer};
use num_bigint::ToBigUint;
fn main() {
    // 2025-07-26 21:34-21:50 (16min)
    input! {
        t: usize,
    }

    let mut ns = vec![];
    let mut ms = vec![];
    let mut a = vec![];
    let mut b = vec![];


    for i in 0..t {
        input! {
            ni: usize,
            mi: usize,
            ai: [usize; ni],
            bi: [usize; ni],
        }
        ns.push(ni);
        ms.push(mi);
        a.push(ai);
        b.push(bi);
    }

    for i in 0..t {
        let ni = ns[i];
        let mi = ms[i];
        // let ai = &a[i];
        // let bi = &b[i];
        // let mut heap = BinaryHeap::new();
        let mut btree = BTreeMap::new();
        for j in 0..ni {
            // heap.push(a[i][j]);
            *btree.entry(a[i][j]).or_insert(0) += 1;
        }
        // a[i].sort();
        b[i].sort();

        let mut ans = 0;
        for j in (0..ni).rev() {
            let bij = b[i][j];
            if let Some((aij, _)) = btree.range((mi-bij)..).next() {
                ans += (aij + bij) % mi;
                btree.subtract_value_by_1(*aij);
            }
            else {
                let (&aij, _) = btree.iter().next().unwrap();
                ans += (aij + bij) % mi;
                btree.subtract_value_by_1(aij);
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