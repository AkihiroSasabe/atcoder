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
    let mut v1 = vec![];
    let mut v2 = vec![];
    let mut l1 = vec![];
    let mut l2 = vec![];
    input! {
        l: usize,
        n1: usize,
        n2: usize,
    }
    for i in 0..n1 {
        input! {
            v_i: usize,
            l_i: usize
        }
        v1.push(v_i);
        l1.push(l_i);
    }
    for i in 0..n2 {
        input! {
            v_i: usize,
            l_i: usize
        }
        v2.push(v_i);
        l2.push(l_i);
    }

    let mut ans = 0;
    let mut index_g = 0; // 次の調べたいとこ
    let mut index1 = 0;
    let mut index2 = 0;
    let mut vv1 = v1[index1];
    let mut vv2 = v2[index2];
    let mut res1 = l1[index1];
    let mut res2 = l2[index2];

    while index1 < n1 && index2 < n2 {
        if vv1 == vv2 {
            ans += min(res1, res2);
        }

        if res1 < res2 {
            index1 += 1;
            res2 -= res1;
            if index1 >= n1 {break}
            res1 = l1[index1];
        }
        else if res1 == res2 {
            index1 += 1;
            index2 += 1;
            if index1 >= n1 {break}
            if index2 >= n2 {break}
            res1 = l1[index1];
            res2 = l2[index2];
        }
        else {
            index2 += 1;
            res1 -= res2;
            if index2 >= n2 {break}
            res2 = l2[index2];
        }
        if index1 >= n1 {break}
        if index2 >= n2 {break}
        vv1 = v1[index1];
        vv2 = v2[index2];
    }
    println!("{}", ans);


}