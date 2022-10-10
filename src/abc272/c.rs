use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
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
    let mut kisu = vec![];
    let mut gusu = vec![];

    for i in 0..n {
        if a[i] % 2 == 0 {
            gusu.push(a[i]);
        }
        else {
            kisu.push(a[i]);
        }
    }

    if kisu.len() >= 2 || gusu.len() >= 2 {
        let mut ans = 0;
        if kisu.len() >= 2 {
            ans = kisu[kisu.len() - 1] + kisu[kisu.len() - 2];
        }
        if gusu.len() >= 2 {
            ans = max(ans, gusu[gusu.len() - 1] + gusu[gusu.len() - 2]);
        }
        println!("{}", ans);
    }
    else {
        println!("-1");
    }


}