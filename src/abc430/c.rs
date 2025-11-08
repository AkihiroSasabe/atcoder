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
        a: usize,
        b: usize,
        ss: Chars
    }
    let mut a_nums = vec![0; n];
    let mut b_nums = vec![0; n];

    for i in 0..n {
        if ss[i] == 'a' {
            a_nums[i] = 1;
        }
        else {
            b_nums[i] = 1;
        }
    }
    let mut cum_a: Vec<usize> = CumulativeSum::new(&a_nums);
    let mut cum_b: Vec<usize> = CumulativeSum::new(&b_nums);

    // let mut set_b = BTreeSet::new();
    // for i in 0..n {
    //     set_b.insert((i, cum_b[i]));
    // }

    let mut ans = 0;
    for l in 0..n {
        // println!("L = {} ----", l);
        let mut pre = 0;
        if l != 0 {
            pre = cum_a[l - 1];
        }
        let ind = cum_a.lower_bound(&(pre + a));
        if ind >= n {
            continue;
        }

        if cum_b.range_sum(l, ind) >= b {
            continue
        }

        // if b_nums[ind] >= b {
        //     continue
        // }

        // let mut pre_b = 0;
        // if ind != 0 {
        //     pre_b = cum_b[ind - 1];
        // }

        // めぐる式二分探索
        // 関数じゃなくて、クロージャーを使うと、引数を少なく出来る。
        let judge = |mid: usize| -> bool {
            return cum_b.range_sum(l, mid) < b
        };
        // fn judge(mid: usize) -> bool {
        //    return true
        // }
        
        let mut ng = n-1;
        let mut ok = ind;
        if judge(ng) {
            ok = ng;
        }
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
        // println!("ind = {:?}", ind);
        // println!("ok={}", ok);

        ans += 1 + ok - ind;

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
        let mut cum: Vec<T> = Vec::with_capacity(array.len());
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