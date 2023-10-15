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
        m: usize,
        p: usize,
        mut a: [usize; n],
        mut b: [usize; m],
    }
    a.sort();
    b.sort();
    let mut sum = 0;
    let mut a_sum = 0;
    let mut b_sum = 0;

    let mut a_cum = vec![0; n];
    let mut b_cum = vec![0; m];
    for i in 0..n {
        a_sum +=a[i];
        a_cum[i] = a_sum;
    }
    for i in 0..m {
        b_sum +=b[i];
        b_cum[i] = b_sum;
    }
    sum = a_sum * m + b_sum * n;
    // println!("a_cum = {:?}", a_cum);
    // println!("b_cum = {:?}", b_cum);
    for i in 0..n {
        if p <= a[i] {
            sum = sum + p * m - a[i] * m - b_sum;
        }
        else {
            let index = b.upper_bound(&(p-a[i]));
            let num = m - index;
            // println!("i={i}, num={num}");
            if index > 0 {
                sum = sum + p * num - a[i] * num - (b_sum - b_cum[index-1]);
            }
            else {
                sum = sum + p * num - a[i] * num - b_sum;
            }
        }
    }
    println!("{}", sum);



}