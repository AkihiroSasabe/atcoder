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
        mut a: Usize1,  // 調べる対象
        k: usize,       // ステップ数
        b: [Usize1; n],
    }
    for _ in 0..k {
        // println!("{}",a + 1);
        a = b[a];
    }
    println!("{}", a+1);
}