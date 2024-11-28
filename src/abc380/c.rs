#![allow(dead_code, unused_imports)]
use proconio::{input, marker::Usize1};
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
        k: usize,
        s: Chars,
    }

    // k-1番目のケツ
    // k番目の先頭と、長さが、わかれば良い。

    let mut start = 0;
    let mut length = 0;
    let mut is_0 = true;
    let mut katamris = vec![];
    for i in 0..n {
        if i == n-1 && s[i] == '1' {
            if is_0 {
                katamris.push((i, 1));
            }
            else {
                length += 1;
                katamris.push((start, length));    
            }
            break
        }
        if is_0 && s[i] == '1' {
            start = i;
            length = 1;
        }
        else if !is_0 && s[i] == '1' {
            length += 1;
        }
        else if !is_0 && s[i] == '0' {
            katamris.push((start, length));
        }
        is_0 = s[i] == '0';
    }

    let (k_1th, length_1) = katamris[k-2];
    let (k_0th, length_0) = katamris[k-1];

    let mut index = 0;

    let mut ans = vec![];
    for i in 0..k_1th + length_1 {
        ans.push(s[i]);
    }
    for i in 0..length_0 {
        ans.push('1');
    }
    for i in k_1th + length_1..k_0th {
        ans.push(s[i]);
    }
    for i in k_0th + length_0..n {
        ans.push(s[i]);
    }

    for ai in ans {
        print!("{}", ai);
    }




    // if k_1th + length_1 <= i && i < k_1th + length_1 + length_0 {
    //     print!("1");
    // }
    // else if k_1th + length_1 + length_0 <= i  {
    //     print!("{}", s[i-]);
    // }
    // else {
    //     print!("{}", s[i]);
    // }



}