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
        k: usize,
    }

    let mut a = n;
    for _ in 0..k {
        a = f(a);
    }
    println!("{}",a);

}

fn f(mut x: usize) -> usize {
    let mut list = vec![];

    while x != 0 {
        let r = x % 10;
        list.push(r);
        x /= 10;
    }
    list.sort();

    let mut g1 = 0;
    let mut pow = 1;
    for &elem in list.iter() {
        g1 += pow * elem;
        pow *= 10;
    }

    list.reverse();
    let mut g2 = 0;
    let mut pow = 1;
    for &elem in list.iter() {
        g2 += pow * elem;
        pow *= 10;
    }

    return g1 - g2
    
}