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
    // 2024-11-29 12:38-12:49 (11min)
    input! {
        s: Chars
    }
    // 26文字で何とかする
    //尺取の匂い

    // 過半数ってことは、2回連続で続けば確定
    // 部分文字列|T|が偶数のときは、どこかで絶対2個連続で続かないといけない。*_*_**_
    // 部分文字列|T|が奇数のときは、3個見て2個以上同じやつが入っていればok。 *_*
    let n = s.len();
    for i in 1..n {
        if s[i-1] == s[i] {
            println!("{} {}", i, i+1);
            return;
        }
    }

    for i in 2..n {
        let mut set = BTreeSet::new();
        for j in 0..3 {
            set.insert(s[i-j]);
        }
        if set.len() < 3 {
            println!("{} {}",i - 1, i + 1);
            return;
        }
    }

    println!("-1 -1");

}