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
    // 2023-01-29 17:10-
    input! {
        n: usize,
        mut s: [String; n]
    }

    let mut ss = vec![];
    for i in 0..n {
        let i_string: String = i.to_string();
        ss.push(vec![s[i].clone(), i_string]);
    }
    ss.sort();
    for i in 0..n {
        println!("ss[{}]: {:?}", i, ss[i]);
    }

    let mut counts = vec![0; n];
    let mut pre=0;
    let mut post=0;
    for i in 0..n {
        // if i == 1 {
        //     pre = min(ss[i-1][0].len(), ss[i][0].len());
        //     for j in 0..min(ss[i-1][0].len(), ss[i][0].len()) {
        //         let c0 = ss[i-1][0].chars().nth(j).unwrap();
        //         let c1 = ss[i][0].chars().nth(j).unwrap();
        //         // println!("pre: i: {}, c0:{} c1:{}",i, c0, c1);
        //         if c0 != c1 {
        //             pre = min(pre, j);
        //             break
        //         }
        //     }
        // }

        if i+1 < n {
            post = min(ss[i+1][0].len(), ss[i][0].len());
            for j in 0..min(ss[i+1][0].len(), ss[i][0].len()) {
                let c0 = ss[i+1][0].chars().nth(j).unwrap();
                let c1 = ss[i][0].chars().nth(j).unwrap();
                // println!("post: i: {}, c0:{} c1:{}",i, c0, c1);
                if c0 != c1 {
                    post = min(post, j);
                    break
                }
            }
        }
        else {
            post = 0;
        }
        let c: usize = ss[i][1].parse().unwrap();
        // println!("i:{}, pre:{} post:{}, c:{}", i, pre, post, c);
        counts[c] = max(pre, post);
        pre = post;
    }
    for i in 0..n {
        println!("{}", counts[i]);
    }
}