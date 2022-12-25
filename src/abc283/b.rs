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
        mut a: [usize; n],
        q: usize,
    }
    let mut query = vec![vec![]; q];
    for i in 0..q {
        input! {
            qq: usize,
            k: usize
        }
        query[i].push(qq);
        query[i].push(k);
        if qq == 1 {
            input! {
                x: usize
            }
            query[i].push(x);
        }

    }
    for i in 0..q {
        let qq = query[i][0];
        let k = query[i][1];
        if qq == 1 {
            let x = query[i][2];
            a[k-1] = x;
        }
        else {
            println!("{}", a[k-1]);
        }
    }
}