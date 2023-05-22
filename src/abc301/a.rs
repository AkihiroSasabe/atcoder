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
        n: usize,
        s: Chars
    }
    let mut t_num = 0;
    let mut a_num = 0;
    let mut t_is_big = true;
    for i in 0..n {
        if s[i] == 'T' {
            t_num += 1;
        }
        else {
            a_num += 1;
        }
        if t_num > a_num {
            t_is_big = true;
        }
        else if a_num > t_num {
            t_is_big = false;
        }
    }
    if t_is_big {
        println!("T");
    }
    else {
        println!("A");
    }


}