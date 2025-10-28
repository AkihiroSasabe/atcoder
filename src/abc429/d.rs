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
    input! {
        n: usize,
        m: usize,
        c: usize,
        a: [usize; n],
    }
    let mut a2 = a.clone();
    a2.sort();

    // btree := [場所, 人の数]
    let mut btree = BTreeMap::new();
    for i in 0..n {
        *btree.entry(a[i]).or_insert(0) += 1;
        *btree.entry(a[i] + m).or_insert(0) += 1;
    }

    let mut counts = vec![];
    let mut poss = vec![];
    for (ind, (&pos, &count)) in btree.iter().enumerate() {
        counts.push(count);
        poss.push(pos);
    }

    let cum_counts: Vec<usize> = CumulativeSum::new(&counts);
    // println!("poss = {:?}", poss);
    // println!("counts = {:?}", counts);
    // println!("cum_counts = {:?}", cum_counts);
    // let cum_poss: Vec<usize> = CumulativeSum::new(&poss);

    let mut ans = 0;
    let mut pre_pos = a2[a2.len()-1]; // チェック済み
    for (ind, &pos) in poss.iter().enumerate() {
        // println!("(ind, pos, pre_pos) = {:?} ----", (ind, pos, pre_pos));

        if pos >= m {break}

        let mut num_pre = 0;
        if ind > 0 {
            num_pre = cum_counts[ind - 1];
        }
        let stop_ind = cum_counts.lower_bound(&(c + num_pre));
        let num_sweep_hito: usize = cum_counts[stop_ind] - num_pre;

        let diff = if pos > pre_pos {
            pos - pre_pos
        } else {
            pos + m  - pre_pos
        };
        // println!("num_sweep_hito = {:?}", num_sweep_hito);
        // println!("diff = {:?}", diff);
        ans += num_sweep_hito * diff;
        pre_pos = pos;
    }
    println!("{}", ans);
}


fn solve_brute(n: usize, m: usize, c: usize, a: &Vec<usize>) {
    // counts[p] := 場所pにいる人の数
    let mut counts: Vec<usize> = vec![0; 2*(m+1)];
    for i in 0..n {
        counts[a[i]] += 1;
        counts[m + a[i]] += 1;
    }

    let cum: Vec<usize> = CumulativeSum::new(&counts);

    let mut ans = 0;
    for p in 0..m {
        let num_pre = cum[p];
        let ind = cum.lower_bound(&(c + num_pre));
        let cont = cum[ind] - num_pre;
        ans += cont;
    }
    println!("{}", ans);

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
    // let cum: Vec<usize> = CumulativeSum::new(&array); // のように使う 
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