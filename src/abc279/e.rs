#![allow(dead_code, unused_imports)]
use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; m]
    }

    // 例
    // N, M = 5, 4
    // A = [1, 2, 3, 2]
    // k = [1, 2, 3, 4]

    // [_, 2, 3, 4] <- i = 1のとき
    // [1, _, 3, 4] <- i = 2のとき
    // [1, 2, _, 4] <- i = 3のとき
    // [1, 2, 3, _] <- i = 4のとき

    // 愚直にやると、O(M*M)かかってTLE
    // iの前後で射影を分け、射影の結果を共有することでO(M)で計算できる
    // (1) まずはiの前について、b[0]の移動先を求める
    // (2) 次にiの後の射影先を求め、(1)の結果を代入していく
    // https://atcoder.jp/contests/abc279/editorial/5311


    a = a.iter().map(|x| x-1).collect();

    // b_index[x]: 値xがb内の何番目に格納されているか、インデックスで示す
    let mut b_index: Vec<usize> = (0..n).collect();
    let mut b: Vec<usize> = (0..n).collect();
    // println!("a: {:?}", a);
    // println!("b: {:?}", b);

    // k=iより前の射影を考える
    let mut pre = vec![];
    // pre: Bj == 1となるindexを格納
    pre.push(b_index[0]);
    for k in 0..m-1 {
        let ii_val = b[a[k]];
        let jj_val = b[a[k]+1];
        let ii = b_index[ii_val];
        let jj = b_index[jj_val];
        b.swap(ii, jj);
        b_index.swap(ii_val, jj_val);
        pre.push(b_index[0]);
    }
    // println!("pre: {:?}", pre);

    // k=i+1以降の射影を考える
    let mut ans = vec![];
    let mut b: Vec<usize> = (0..n).collect();
    let mut b_index: Vec<usize> = (0..n).collect();
    ans.push(pre.pop().unwrap());
    for kk in 0..m-1 {
        let k = a.len() - 1 - kk;
        let ii_val = b[a[k]];
        let jj_val = b[a[k]+1];

        let ii = b_index[ii_val];
        let jj = b_index[jj_val];

        b.swap(ii, jj);
        b_index.swap(ii_val, jj_val);
        b.swap(ii, jj);
        ans.push(b_index[pre.pop().unwrap()]);
    }

    for i in 0..ans.len() {
        println!("{}", ans[ans.len()-1-i] + 1);
    }


}