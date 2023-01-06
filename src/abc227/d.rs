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
use std::ops::Index;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    // 2023-01-06 15:55-
    // 16:53 (NG)  TLEした。方針がよくない。58min
    // 19:54 (AC)  解説AC (4h, もっと早く諦めて解説見ればよかった。今度から時間を決めよう)
    input! {
        n: usize,
        k: usize,
        mut a: [usize; n]
    }

    // let debug = judge(2, &a, k);
    // dbg!(debug);

    // 解を決め打ち二分探索
    let sum: usize = a.iter().sum();
    let mut ng = sum / k + 1;
    let mut ok = 1;
    
    while ng - ok > 1 {
        let mid = (ng + ok) / 2;
        if judge(mid, &a, k) {
            ok = mid;
        }
        else {
            ng = mid;
        }
    }
    println!("{}", ok);
    

    // TLE解(1)
    // a.sort();
    // loop {
    //     // 第k位の人数
    //     let num_k = a[a.len()-k];
    //     // println!("{:?}", a);
    //     // input! {
    //     //     tekito: Chars,
    //     // }
    //     // 第k位の人数が0になればこれ以上作れないので終了
    //     if num_k == 0 {break}
    //     // 第k+1位の人数
    //     let mut num_kp1 = 0;
    //     if a.len() != k {
    //         num_kp1 = a[a.len()-(k+1)];
    //     }
    //     let mut diff = num_k - num_kp1;
    //     // k+1位が0でなければさらに-1
    //     if num_kp1 != 0 {
    //         diff += 1;
    //     }
    //     // println!("diff :{diff}");
    //     for i in 0..k {
    //         a[n-k + i] -= diff;
    //     }
    //     ans += diff;
    //     a.sort();
    // }

    // うまくかけない
    // // key: 部署に属する人数, val: key人属する部署の数
    // let mut btree = BTreeMap::new();
    // for i in 0..n {
    //     if btree.contains_key(&a[i]) {
    //         *btree.get_mut(&a[i]).unwrap() += 1;
    //     }
    //     else {
    //         btree.insert(a[i], 1);
    //     }
    // }
    // loop {
    //     let mut count = k;

    //     // 第k位の人数
    //     let mut num_k = 0;
    //     // 第k+1位の人数
    //     let mut num_kp1 = 0;

    //     let mut flag = false;
    //     for (&key, &val) in btree.iter().rev() {
    //         if flag {
    //             num_kp1 = key;
    //             break
    //         }
    //         if val < count {
    //             count -= val;
    //         }
    //         else if val == count {
    //             num_k = key;
    //             flag = true;
    //         }
    //         else if val > count {
    //             num_k = key;
    //             num_kp1 = key;
    //             flag = true;
    //         }
    //     }
    //     let mut diff = num_k - num_kp1;
    //     // k+1位が0でなければさらに-1
    //     if num_kp1 != 0 {
    //         diff += 1;
    //     }
        
    //     // let range = btree.range(num_k..);
    //     for (&key, &mut _) in btree.range_mut(num_k..) {
    //         if btree.contains_key(&(key - diff)) {
    //             btree.remove(&key);
    //         }
    //     }

    //     // println!("diff :{diff}");
    //     for i in 0..k {
    //         // 第k位置の人数
    //         a[n-k + i] -= diff;
    //     }
    //     ans += diff;
    // }    
}


// N K
// A1, A2, ..., An

// 1<=K<=N<=2*10^5
// 1<=Ai<=10^12

// 1プロジェクト: K個の相異なる部署から1人ずつK人
// 1人が複数のプロジェクトには、はいれない

// プロジェクトが最大になるように。最大何個?

// 例1)
// 3 3
// 2 3 4

// 3人必要
// 2個

// 4 2
// 1 1 3 4
// 4個

// 人数の多いプロジェクトから鳴らしていくのが良さそう
// 1 1 2 3
// 1 1 1 2
// 1 1 0 1
// 0 0 0 1

// 16:04 方針思いつき
// 第K位の人数がk+1位の人数-1になるまで作業する

// 4 3
// 1 1 3 4
// > 1個作る
// 1 0 2 3
// > 1個作る
// 0 0 1 2

// 終了条件:  keyの数がK未満になったとき


// p_num個のプロジェクトは作成可能かを判定
fn judge(p_num: usize, a: &Vec<usize>, k: usize) -> bool {
    let mut index = 0;
    let mut count = 0;
    for i in 0..a.len() {
        // dbg!(count);
        let mut next_index = index + a[i];
        if a[i] >= p_num {
            next_index = index + p_num;
        }

        if next_index > p_num - 1 {
            count += 1;
            // 各プロジェクトにk人を割り当てられればok
            if count >= k {
                return true
            }
            next_index = next_index % p_num;
        }
        index = next_index;
    }
    return false
}

// 下記の入力のとき、p_num == 3 は可能か? 
// 3 3
// 2 3 4
// 下のシミュレーションで無理と判明
// P0	0	1	2	
// P1	0	1	2
// P2	1	2	x
