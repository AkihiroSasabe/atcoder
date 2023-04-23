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
        mut a: i128,
        mut b: i128,
    }
    // 考察:
    // ユークリッドの互除法に似た問題。
    // A' = A mod B (B<A)で、A'==0まで計算すると計算量はO(logA+logB)になる。
    // 理由: 「A' = A % B　だとすると、A' <  A/2」　となるから。
    // 証明: 
    // (前提)定義よりA' < B.
    // (1)B  <  A/2の場合: A' < BよりA' < A/2
    // (2)A/2 < Bの場合: A%B=A-B<A/2となるので、A' < A/2
    // (3)B = A/2の場合: A'=A%B=0

    let mut ans: i128 = 0;
    loop {
        // println!("a: {}, b: {}, ans: {}", a, b, ans);
        if a > b {
            if a % b == 0 {
                ans += a / b - 1;
                break
            }
            else {
                ans += a / b;
                a = a % b;
            }
        }
        else if a < b {
            if b % a == 0 {
                ans += b / a - 1;
                break
            }
            else {
                ans += b / a;
                b = b % a;
            }
        }
        else {
            break
        }
    }

    println!("{}", ans);
}