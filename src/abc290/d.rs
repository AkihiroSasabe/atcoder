#![allow(dead_code, unused_imports)]
use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::BTreeMap;
use std::hash::Hash;
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::collections::HashSet;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    input! {
        t: usize,
    }
    let mut n = vec![];
    let mut d = vec![];
    let mut k = vec![];
    for i in 0..t {
        input! {
            ni: usize,
            di: usize,
            ki: usize
        }
        n.push(ni);
        d.push(di);
        k.push(ki);
    }

    for i in 0..t {
        let ni = n[i];
        let mut di = d[i];
        if di > ni {
            di = di % ni;
        }
        if di == 0 {
            di = 1;
        }
        let mut ki = k[i]-1;

        let mut ans = 0;
        let mut start = 0;
        // let mut hash = BTreeMap::new();
        let mut hash = HashMap::new();
        hash.insert(start, 0);
        loop {
            // ni-1を超えるまでに足す回数
            let num = (ni-1-start) / di;
            println!("ki: {}, start: {}, num: {}", ki, start, num);
            if ki <= num {
                ans = start + di * ki;
                break
            }
            else {
                start = (start + (num+1) * di) % ni;
                while hash.contains_key(&start) {
                    println!("start exist: {}", start);
                    // start = (start + 1) % ni; 
                    start = (start + 1) % ni;
                }
                hash.insert(start, 0);
                ki -= num + 1;
            }
        }
        println!("{}", ans);
    }




}