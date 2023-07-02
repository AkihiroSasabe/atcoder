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
        s: [usize; 8]
    }

    let mut flag = true;
    for i in 0..8 {
        if !(s[i] % 25 == 0 && 100 <= s[i] && s[i] <= 675) {
            flag = false;
        }
        if i > 0 {
            if s[i] < s[i-1] {
                flag = false;
            }
        }
    }
    if flag {
        println!("Yes");
    }
    else {
        println!("No");
    }


}