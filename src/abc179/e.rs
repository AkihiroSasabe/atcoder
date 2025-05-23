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
        x: usize,
        m: usize,
    }

    if x == 0 {
        println!("0");
        return
    }
    else if x == 1 {
        println!("{}",n);
        return
    }
    if n == 1 {
        println!("{}",x);
        return
    }
    if m == 1 {
        println!("{}",x);
        return
    }

    let mut a = vec![0; m+5];
    a[0] = x;

    let mut btree = BTreeMap::new();
    let mut start_ind = 0;
    let mut cycle = 0;
    for i in 1..m+5 {
        a[i] = a[i-1] * a[i-1] % m;
        if let Some(&ind) = btree.get(&a[i]) {
            start_ind = ind;
            cycle = i - start_ind;
            break
        }
        btree.insert(a[i], i);
    }
    // println!("a = {:?}", a);
    // println!("cycle = {:?}", cycle);
    // println!("start_ind = {:?}", start_ind);


    let mut ans = 0;
    for i in 0..start_ind {
        ans += a[i];
    }
    // println!("ans0 = {:?}", ans);
    let mut cum = vec![0; cycle];
    for (index, i) in (start_ind..start_ind+cycle).enumerate() {
        cum[index] = a[i];
    }
    // println!("cum = {:?}", cum);
    for i in 1..cycle {
        cum[i] = cum[i-1] + cum[i];
    }
    // println!("cum = {:?}", cum);

    let num_cycle = (n - start_ind) / cycle;
    // println!("num_cycle = {:?}", num_cycle);
    ans += num_cycle  * cum[cum.len() - 1];
    if (n - start_ind) % cycle != 0 {
        ans += cum[(n - start_ind) % cycle - 1];
    }

    println!("{}", ans);
}