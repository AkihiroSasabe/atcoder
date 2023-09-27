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
    // 2023-09-27 12:29-12:49
    // 19:05-19:15
    // 30min
    input! {
        n: usize,
        m: usize,
        mut p: [isize; n],
        mut l: [isize; m],
        mut d: [isize; m],
    }

    // 値引き額が大きいものから、貪欲に使っていけばいいのでは?
    p.sort();

    let mut d_minusl = vec![];
    for i in 0..m {
        d_minusl.push(vec![d[i], -l[i]]);
    }
    d_minusl.sort();
    d_minusl.reverse();

    // dlは値引き率が大きくて、かつ使用できるコストが低い順に並んでいる。
    let mut dl = vec![];
    let mut d_sorted = vec![];
    let mut l_sorted = vec![];
    for i in 0..m {
        dl.push(vec![d_minusl[i][0], -d_minusl[i][1]]);
        d_sorted.push(d_minusl[i][0]);
        l_sorted.push(-d_minusl[i][1]);
    }

    let mut ans = 0;
    // btree<値段, 個数>
    let mut btree = BTreeMap::new();
    for i in 0..n {
        ans += p[i];
        *btree.entry(p[i]).or_insert(0_usize) += 1;
    }
    // let mut used = vec![false; n];
    for i in 0..m {
        let off_price = d_sorted[i];
        let condition = l_sorted[i];
        // condition以上で最小の値段の商品を探す
        if let Some((&key, _)) = btree.range(condition..).next() {
            ans -= off_price;
            reduce_from_btree(&mut btree, key);
        }
    }
    println!("{}", ans);

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