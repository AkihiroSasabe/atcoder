#![allow(dead_code, unused_imports)]
use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n]
    }
    // println!("{:?}", s[0]);

    let mut ans = 0;
    for i in 0..n {
        for j in (i+1)..n {
            let mut flag = true;
            for k in 0..m {
                if s[i][k] == 'x' && s[j][k] == 'x' {
                    flag = false;
                    break
                }
            }
            if flag {
                ans += 1;
            }
        }
    }
    println!("{}", ans);


}