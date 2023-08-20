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
    input! {
        s: Chars
    }
    let temp = vec!['a', 'e', 'i', 'o', 'u'];

    let mut ans = vec![];
    for i in 0..s.len() {
        let mut flag = false;
        for j in 0..temp.len() {
            if s[i] == temp[j] {
                flag = true;
                break
            }
        }
        if flag {
            continue
        }
        else {
            ans.push(s[i]);
        }
    }

    for i in 0..ans.len() {
        print!("{}", ans[i]);
    }

}