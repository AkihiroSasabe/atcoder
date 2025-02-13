#![allow(dead_code, unused_imports)]
use proconio::{input, marker::{Usize1, Isize1}};
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
use num::{BigUint, ToPrimitive};
use num_bigint::ToBigUint;
fn main() {
    // 2025-02-11 15:19-15:30 (11min)
    // 2025-02-13 17:33-18:04 (31min, 競プロフレンズのheapを使えば良いというヒントを得てAC。半分解説AC)
    // Total: 42min
    input! {
        n: usize,
        x: usize,
        mut a: [usize; n],
        mut b: [usize; n],
        mut c: [usize; n],
    }
    // 解法: 3分木
    // a,b,cを大きい順に並べ替え
    // 候補をsetに格納していき、ある程度の要素数が溜まったら、x番目に大きい要素を出力すればok。
    // ある程度とはどれくらいか、を見積もる必要がある。
    // indexが(0,0,0) が1番大きいのは自明。
    // 2番目に大きいのは、(1,0,0) or (0,1,0) or (0,0,1) の3択。
    // 3番目に大きいのは、2番目に選ばれなかった2個か、2番目に選ばれた奴から派生された3個の5択。
    // 4番目に大きいのは、3番目に選ばれなかった4個か、3番目に選ばれた奴から派生された3個の7択。
    // x番目に大きいのは、x-1番目に選ばれなかったf(x-1)-1個か、x-1番目に選ばれた奴から派生された3個のf(x-1)+2択。
    // 
    // x番目に大きいやつの候補数をf(x)とすると、f(x) = f(x-1) + 2 = ... f(1) + 2*(x-1) = 2*x-1 となるから、
    // 最終的なsetには、"それまでに選ばれたx-1個の要素" と "x番目に大きいやつの候補数" が格納される。つまり,
    // set.len() = x-1 + f(x) = x-1 + 2*x-1 = 3x-2個となる。
    // 実装時の注意として、同じ要素(i,j,k)をheapやsetに入れないように注意したい。
    a.sort();
    b.sort();
    c.sort();
    a.reverse();
    b.reverse();
    c.reverse();

    let mut heap = BinaryHeap::new();
    let mut set = BTreeSet::new();

    let sum = get_sum(0, 0, 0, &a, &b, &c);
    let key = (sum, 0, 0, 0);
    heap.push(key);
    set.insert(key);

    while set.len() < 3*x {
        // println!("heap = {:?}", heap);
        if let Some((s, i, j, k)) = heap.pop() {
            if i != n-1 {
                let sum = get_sum(i+1, j, k, &a, &b, &c);
                let key = (sum, i+1, j, k);
                if !set.contains(&key) {
                    set.insert(key);
                    heap.push(key);
                }
                
            }
            if j != n-1 {
                let sum = get_sum(i, j+1, k, &a, &b, &c);
                let key = (sum, i, j+1, k);
                if !set.contains(&key) {
                    set.insert(key);
                    heap.push(key);
                }
                
            }
            if k != n-1 {
                let sum = get_sum(i, j, k+1, &a, &b, &c);
                let key = (sum, i, j, k+1);
                if !set.contains(&key) {
                    set.insert(key);
                    heap.push(key);
                }
            }
        }
        else {break}
    }
    for (index, (sum, _, _, _)) in set.iter().rev().enumerate() {
        if index == x - 1 {
            println!("{}", sum);
            return
        }
    }
}
fn get_sum(i: usize, j: usize, k: usize, a: &Vec<usize>, b: &Vec<usize>, c: &Vec<usize>) -> usize {
    let sum = a[i] * b[j] + b[j] * c[k] + c[k] * a[i];
    return sum
}



/*

◆以下、無駄な途中考察
x番目...。
順当にシミュレーションするだけで良いのでは。 -> 無理。(i,j,k)は連続に変化しない

------------------------------------------
大きい方からx番目
s(i,j,k) = a[i] * b[j] + b[j] * c[k] + c[k] * a[i]
(i,j,k) は N^3 の選び方
大きい方からx番目のs(i,j,k)は?
------------------------------------------

答えを決め打ち二ブタンでは?
3,9
3,9
大きい数字同士をかける方が大きくなれそう

aを全探索してみる。
s(0,j,k) = a[0] * b[j] + b[j] * c[k] + c[k] * a[0] は、1 ~ N^2位である。
s(1,j,k) = a[1] * b[j] + b[j] * c[k] + c[k] * a[1] は、N + 1 ~ N^2位である。
...
s(1,j,k) = a[1] * b[j] + b[j] * c[k] + c[k] * a[1] は、N + 1 ~ N^2位である。

3 10
1 3 5
2 4 7
6 8 9
*/

