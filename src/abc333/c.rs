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
        mut n: usize
    }

    // 112222222233
    // 1133223

    // 112222222233
    // 111111111111

    // 111111111111

    // 112_222_222_233
    // 111_111_111_111
    // 001_111_111_111
    // 000_000_000_011

    // 十進法ですべての桁の数字が 1 である整数の集合。repunits[i] := 1がi+1桁ある
    let mut repunits: Vec<usize> = vec![];
    let mut temp: usize = 1;
    for i in 0..12 {
        repunits.push(temp);
        temp *= 10;
        temp += 1;
    }
    // println!("repunits = {:?}", repunits);

    let mut btree = BTreeSet::new();
    for i in 0..12 {
        for j in 0..i+1 {
            for k in 0..j+1 {
                let x = repunits[i] + repunits[j] + repunits[k];
                btree.insert(x);
            }
        }
    }

    let mut count = 0;
    for x in btree.iter() {
        count += 1;
        if count == n {
            println!("{}", x);
        }
    }

}