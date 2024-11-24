#![allow(dead_code, unused_imports)]
use proconio::{input, marker::Usize1};
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
    // 2024-11-22 22:09-22:40 (31min)
    // 2024-11-23 15:04-15:59 (55min)
    // 2024-11-23 15:59-16:35 (36min, 問題文の意味を勘違いしているのに、気付いた。非連続な部分列でよいこと) 
    // Total: 2h02min
    input! {
        n: usize,
        q: usize,
        s: Chars,
    }
    let mut l = vec![];
    let mut r = vec![];
    let mut ans = vec![0; q];
    for i in 0..q {
        input!{
            li: Usize1,
            ri: Usize1,
        }
        l.push(li);
        r.push(ri);
    }

    // 15:59- 
    let mut cum1 = vec![0; n]; // '1' の数の累積和
    let mut cum2 = vec![0; n]; // '2' の数の累積和

    let mut btree = BTreeMap::new(); // key: '/'のindex, val:何番目の'/'か?  
    let mut vec = vec![]; // 各'/'のインデックスを格納
    for i in 0..n {
        if s[i] == '/' {
            btree.insert(i, vec.len());
            vec.push(i);
        }
        else if s[i] == '1' {
            cum1[i] = 1;
        }
        else if s[i] == '2' {
            cum2[i] = 1;
        }
    }

    cum1 = CumulativeSum::new(&cum1);
    cum2 = CumulativeSum::new(&cum2);

    // 計算量: O(Q * log(N) * log(N), NlogN)
    for i in 0..q {
        let li = l[i];
        let ri = r[i];

        if let Some((&pos_l, &ind_l)) = btree.range(li..ri+1).next() {
            if let Some((&pos_r, &ind_r)) = btree.range(li..ri+1).rev().next() {

                // 一番左の'/'
                let (num_1, num_2) = get_num(ind_l, li, ri, &vec, &cum1, &cum2);
                if num_1 >= num_2 {
                    println!("{}", num_2 * 2 + 1);
                    continue;
                }
                let mut ok = ind_l; // num_1 < num_2

                // 一番右の'/'
                let (num_1, num_2) = get_num(ind_r, li, ri, &vec, &cum1, &cum2);
                if num_1 <= num_2 {
                    println!("{}", num_1 * 2 + 1);
                    continue;
                }
                let mut ng = ind_r;
                // めぐる式二分探索
                let judge= |mid: usize| -> bool {
                    let (num_1, num_2) =get_num(mid, li, ri, &vec, &cum1, &cum2);
                    return num_1 < num_2
                };

                while (ng as i128 - ok as i128).abs() > 1 {
                    let mid = (ng + ok) / 2;
                    let is_ok = judge(mid);
                    if is_ok {
                        ok = mid;
                    }
                    else {
                        ng = mid;
                    }
                }
                // println!("{}", ok);
                let (num_1, num_2) =get_num(ok, li, ri, &vec, &cum1, &cum2);
                let mut ans = 1 + num_1 * 2;

                if ok + 1 < vec.len() && vec[ok+1] <= ri {
                    let (num_1, num_2) =get_num(ok+1, li, ri, &vec, &cum1, &cum2);
                    ans = max(ans, 1 + num_2 * 2);
                }
                println!("{}", ans);
            }
        }
        else {
            println!("0");
        }
    }
}

fn get_num(index: usize, li: usize, ri: usize, 
    vec: &Vec<usize>, 
    cum1: &Vec<usize>,
    cum2: &Vec<usize>,
) -> (usize, usize) {
    let pos = vec[index];
    let num_1 = if (li+ 1 <= pos) {
        cum1.range_sum(li, pos-1)
    } else {
        0
    };
    let num_2 = if (pos + 1 <= ri) {
        cum2.range_sum(pos+1, ri)
    } else {
        0
    };

    return (num_1, num_2)
}

/// 累積和の処理に関するトレイト
pub trait CumulativeSum<T> {
    fn new(data: &[T]) -> Self;
    fn range_sum(&self, l: usize, r: usize) -> T;
}

impl<T> CumulativeSum<T> for Vec<T>
where
    T: Copy + Default + std::ops::Sub<Output = T> + std::ops::Add<Output = T>,
{
    /// 配列の累積和の配列を求める関数
    fn new(array: &[T]) -> Self {
        let mut cum = Vec::with_capacity(array.len());
        let mut sum = T::default();
        for &value in array {
            sum = sum + value;
            cum.push(sum);
        }
        cum
    }
    // [L, R] の和を求める関数
    fn range_sum(&self, l: usize, r: usize) -> T {
        if r < l {
            return T::default(); // Tのデフォルト値 (通常は0) を返す
        }
        if l == 0 {
            return self[r]
        } else {
            return self[r] - self[l - 1]
        }
    }
}

