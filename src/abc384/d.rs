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
fn main() {
    input! {
        n: usize,
        mut s: usize,
        a: [usize; n]
    }

    // let mut set = BTreeSet::new();
    let cum: Vec<usize> = CumulativeSum::new(&a);
    s %= cum[n-1];
    if s == 0 {
        println!("Yes");
        return
    }

    let mut a2 = a.clone();
    for i in 0..n {
        a2.push(a[i]);
    }

    let cum2: Vec<usize> = CumulativeSum::new(&a2);
    let mut set = BTreeSet::new();
    for i in 0..cum2.len() {
        set.insert(cum2[i]);
    }
    for i in 0..n {
        if i == 0 {
            if set.contains(&s) {
                println!("Yes");
                return
            }
        }
        else {
            if set.contains(&(cum2[i-1] +s)) {
                println!("Yes");
                return
            }
        }
    }
    println!("No");

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
    // 配列の累積和の配列を求める関数 (型の注釈は必須なので注意)
    // let cum: Vec<usize> = CumulativeSum::new(&array); みたいにして使う 
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
