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
        n: usize,
        s: Chars
    }
    let zero = '0' as usize;
    let mut kosu = vec![0; 10];
    for i in 0..s.len() {
        let num = s[i] as usize - zero;
        kosu[num] += 1;
    }
    // 9_999_995_824_729 = 3_162_277 * 3_162_277
    // 10_000_002_149_284 = 3_162_278 * 3_162_278
    let mut ans = 0;
    for i in 0..3_162_278 {
        let mut heiho = i * i;
        let mut temp: Vec<usize> = vec![0; 10];
        // 文字列に変換
        while heiho / 10 != 0 {
            temp[heiho % 10] += 1;
            heiho /= 10;
        }
        temp[heiho % 10] += 1;
        
        // 比較
        let mut flag = true;
        for i in 1..10 {
            if temp[i] != kosu[i] {
                flag = false;
                break;
            }
        }

        if flag {
            if kosu[0] >= temp[0] {
                ans += 1;
            }
        }
    }

    println!("{}", ans);



}