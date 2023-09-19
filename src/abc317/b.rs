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
        mut a: [usize; n]
    }
    a.sort();

    let mut pre = a[0];
    for i in 1..n {
        if a[i] - pre != 1 {
            println!("{}", pre + 1);
            return
        }
        pre = a[i];
    }
    // a[0]-1 か a[n-1]+1が答えのとき
    if a[0] == 1 {
        println!("{}", a[n-1]+1);
    }
    else {
        println!("{}", a[0]-1);
    }


}