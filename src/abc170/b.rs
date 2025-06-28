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
use std::collections::{HashSet, BTreeSet};
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    input! {
        x: isize, // 動物
        y: isize, // 足の本数
    }

    for num_kame in 0..x+1 {
        let num_turu = x - num_kame;

        let num_foot = num_turu * 2 + num_kame * 4;
        if y == num_foot {
            println!("Yes");
            return;
        }
    }
    println!("No");

}