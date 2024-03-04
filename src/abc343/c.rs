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
        n: usize
    }

    let mut src = 1;
    let mut ans = 1;
    while src * src * src <= n {
        let cand = src * src * src;
        let mut temp = cand;

        let mut str = vec![];
        while temp != 0 {
            str.push(temp % 10); 
            temp /= 10;
        }
        let len = str.len();
        let mut kaibun = true;
        for i in 0..len {
            if str[i] != str[len-1-i] {
                kaibun = false;
                break;
            }
        }
        if kaibun {
            ans = max(cand, ans);
        }
        src += 1;
    }

    println!("{}", ans);

}