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
        m: usize,
    }

    let mut p = vec![];
    let mut f = vec![];
    let mut f_hash = vec![HashMap::new(); n];
    for i in 0..n {
        input! {
            p_i: usize,
            c_i: usize,
            f_i: [usize; c_i],
        }
        p.push(p_i);
        for j in 0..f_i.len() {
            f_hash[i].insert(f_i[j], 0);
        }
        f.push(f_i);
    }
    let mut ans= false;

    for i in 0..n {
        for j in 0..n {
            let mut flag = true;
            if i == j {continue}
            if p[i] >= p[j] {
                for k in 0..f[i].len() {
                    if !f_hash[j].contains_key(&f[i][k]) {
                        flag = false;
                        break
                    }
                }
                if flag {
                    if !(p[i] > p[j] || f[i].len() < f[j].len()) {
                        flag = false;
                    }
                }
            }
            else {
                flag = false;
            }
            if flag {
                ans = true;
                println!("Yes");
                return
            }
        }
    }

    println!("No");

}