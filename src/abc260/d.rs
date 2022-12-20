use proconio::input;
use itertools::Itertools;
use superslice::Ext;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::{VecDeque, btree_map};
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::BTreeMap;
use std::ops::Bound::{Excluded, Included, Unbounded};
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
fn main() {
    input! {
        n: usize,
        k: usize,
        p: [usize; n]
    }
    // 各カードが何ターン目に食べられるかを格納
    let mut eat_time = vec![-1; n];

    // BTreeMapの使い方が参考になる: 黄色コーダー: https://atcoder.jp/contests/abc260/submissions/33412680
    // key: 場にいるカードで、一番上にいる数字. value: その山に属するカード
    let mut btree_map: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    // 裏向きの山札を上からドロー！
    for i in 0..n {
        let x = p[i];
        // btree_mapのキーで、x以上のキーの内、最小のものを取り出し（もし存在すれば）
        if let Some((&key, _)) = btree_map.range(x..).next() {
            // その山の一番上のキーを破壊。removeはそのキーのバリューをついでに取り出してくれる
            let mut value = btree_map.remove(&key).unwrap();
            value.push(x);
            btree_map.insert(x, value);
        }
        // btree_mapのキーで、x以上のキーがいない場合
        else {
            btree_map.insert(x, vec![x]);
        }
        // K枚重ねられたら、その山のカードを食べる
        if btree_map[&x].len() == k {
            let yama = btree_map.remove(&x).unwrap();
            for card in yama {
                eat_time[card - 1] = i as isize + 1;
            }
        }
    }
    for i in eat_time {
        println!("{}", i);
    }   
}