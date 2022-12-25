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
    let mut ans = s.len();

    let mut flag= false;
    for i in 1..s.len() {
        if flag {
            if s[i] == '0' {
                ans -= 1;
                flag = false;
            }
            else {
                flag = false;
            }
        }
        else {
            if s[i] == '0'  {
                flag = true;
            }
        }
    }
    println!("{}", ans);
}