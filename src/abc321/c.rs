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
        k :usize,
    }
    let mut candidates = vec![];

    // 9,876,543,210 <= 10^10: u32だと厳しい。u64である必要がある。
    // 0,1,2,3,4,5,6,7,8,9

    for bit in 1..(1 << 10) {
        // println!("bit={:010b}", bit);
        let mut temp = vec![];
        for i in 0..10 as usize {
            if bit & (1 << i) != 0 {
                temp.push(i);
            }
        }
        let mut power_of_10 = 1;
        let mut number_321 = 0;
        for i in 0..temp.len() {
            number_321 += temp[i] * power_of_10;
            power_of_10 *= 10;
        }
        if number_321 == 0 {
        }
        candidates.push(number_321);
    }
    candidates.sort();
    println!("{}", candidates[k]);
}

