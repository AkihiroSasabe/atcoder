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
    // 開始時刻: 2025-03-06 20:06:00
    input! {
        s: String
    }
    let mut week = ["SUN","MON","TUE","WED","THU","FRI","SAT"];

    let mut index = 0;
    for i in 0..week.len() {
        if s == week[i] {
            let ans = 7 - i;
            println!("{}", ans);
            return;
        }
    }


}