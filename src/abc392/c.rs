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
    input! {
        n: usize,
        p: [Usize1; n], // 注目先
        q: [Usize1; n], // ゼッケン
    }

    let mut zekken = vec![];
    for i in 0..n {
        zekken.push((q[i], i));
    }
    zekken.sort();

    for i in 0..n {
        let ind = p[zekken[i].1];
        print!("{} ",q[ind] + 1);
    }


}