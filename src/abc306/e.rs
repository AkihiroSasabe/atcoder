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
    input! {
        n: usize,
        k: usize,
        q: usize,
    }
    let mut x = vec![];
    let mut y = vec![];
    for i in 0..q {
        input! {
            x_i: usize,
            y_i: isize,
        }
        x.push(x_i - 1);
        y.push(y_i);
    }
    // Aの初期化
    let mut a = vec![0; n];

    // BをB木に格納
    let mut b_tree = BTreeMap::new();
    b_tree.insert(0, k);

    // B以外にいるAをB木に格納
    let mut not_b_tree = BTreeMap::new();
    not_b_tree.insert(0, n-k);

    // 現在のf(A)
    let mut fa = 0;
    // println!("a={:?}", a);
    // println!("b_tree={:?}", b_tree);
    // println!("not_b_tree={:?}", not_b_tree);
    // println!("{}", fa);

    // 逐次シミュレーションしていく
    for i in 0..q {
        // println!("---- ---- i={} ---- ----", i);
        let pre = a[x[i]];
        a[x[i]] = y[i];
        let after = y[i];
        // afterをnot_b_treeに追加しておく
        add_to_btree(&mut not_b_tree, after);

        // preがBの中にいる場合
        if b_tree.contains_key(&pre) {
            // Bからpreを削除
            reduce_from_btree(&mut b_tree, pre);

            // not Bの中で最大のものをBに移動
            // let (max_of_not_b, _) = not_b_tree.iter().max().unwrap(); // TLEする
            let (max_of_not_b, _) = not_b_tree.iter().rev().next().unwrap();
            let max_of_not_b = *max_of_not_b;

            reduce_from_btree(&mut not_b_tree, max_of_not_b);
            add_to_btree(&mut b_tree, max_of_not_b);

            // faの更新
            fa = fa - pre + max_of_not_b;
        }
        // preがBの中にいない場合
        else {
            // println!("pre={} does not exist in B", pre);
            // B以外からpreを削除
            reduce_from_btree(&mut not_b_tree, pre);

            // Bの中で最小のもの
            // let (min_of_b, _) = b_tree.iter().min().unwrap(); // TLEする
            let (min_of_b, _) = b_tree.iter().next().unwrap();
            let min_of_b = *min_of_b;

            // not Bの中で最大のもの
            // let (max_of_not_b, _) = not_b_tree.iter().max().unwrap(); // TLEする
            let (max_of_not_b, _) = not_b_tree.iter().rev().next().unwrap();
            let max_of_not_b = *max_of_not_b;

            // B と B以外 間で移動が発生するケース
            if min_of_b < max_of_not_b {
                // max_of_not_b を NotB -> B に移動
                reduce_from_btree(&mut not_b_tree, max_of_not_b);
                add_to_btree(&mut b_tree, max_of_not_b);

                // min_of_b を B -> NotB に移動
                reduce_from_btree(&mut b_tree, min_of_b);
                add_to_btree(&mut not_b_tree, min_of_b);

                // faの更新
                fa = fa - min_of_b + max_of_not_b;
            }
        }
        // println!("pre={}, after={}, x={}, y={}", pre, after, x[i], y[i]);
        // println!("a={:?}", a);
        // println!("b_tree={:?}", b_tree);
        // println!("not_b_tree={:?}", not_b_tree);
        println!("{}", fa);
        
    }
}


// B木に要素xを1個追加する関数
fn add_to_btree<T: std::cmp::Ord>(b_tree: &mut BTreeMap<T, usize>, x: T) {
    // b_tree := {key: 要素, value: 格納した要素の個数}
    if b_tree.contains_key(&x) {
        *b_tree.get_mut(&x).unwrap() += 1;
    }
    else {
        b_tree.insert(x, 1);
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