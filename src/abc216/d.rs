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
    // 2023-01-25 22:06-22:29 (23min)
    // 2023-01-26 0:46-1:24 (38min)
    // 2023-01-26 12:16-12:30 (14min)
    // 2023-01-27 12:30-12:34 (4min)
    // 1h19m
    input! {
        n: usize,
        m: usize,
    }
    let mut a = vec![];
    for i in 0..m {
        input! {
            k_i: usize,
            mut a_i: [usize; k_i]
        }
        a_i.reverse();
        a.push(a_i);
    }

    // 筒の一番上にあるボールを登録した辞書 k: ボール番号, v: 筒のindex
    let mut hash = HashMap::new();
    for i in 0..m {
        let k = a[i].pop().unwrap();
        if !hash.contains_key(&k) {
            hash.insert(k, vec![i]);
        }
        else {
            (*hash.get_mut(&k).unwrap()).push(i);
        }
    }

    // 一番上にいて2個あるボールの番号が格納されたリストを作成
    let mut two = vec![];
    for (k, v) in &hash {
        if v.len() == 2 {
            two.push(*k);
        }
    }

    let mut flag = true;
    while two.len() != 0 {
        let ball = two.pop().unwrap();
        // 筒aのindex
        let v1= (*hash.get(&ball).unwrap())[0];
        let v2= (*hash.get(&ball).unwrap())[1];

        // 取り出したボールを削除
        hash.remove(&ball);

        // 新しく取り出すボールの番号
        // unwrapできなければスキップ
        if let Some(k1) = a[v1].pop() {
            // 筒の一番上にあるボールを更新
            if !hash.contains_key(&k1) {
                hash.insert(k1, vec![v1]);
            }
            else {
                (*hash.get_mut(&k1).unwrap()).push(v1);
                two.push(k1);
            }
        }

        // もう一個のボールについても同じように更新
        if let Some(k2) = a[v2].pop() {
            if !hash.contains_key(&k2) {
                hash.insert(k2, vec![v2]);
            }
            else {
                (*hash.get_mut(&k2).unwrap()).push(v2);
                two.push(k2);
            }
        }
    }

    if hash.len() == 0 {
        println!("Yes");
    }
    else {
        println!("No");
    }

}