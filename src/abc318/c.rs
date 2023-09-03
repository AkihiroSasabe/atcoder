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
        d: usize,
        p: usize,
        mut f: [usize; n]
    }
    f.sort();
    f.reverse();
    let mut f_qum = vec![0; n];
    f_qum[0] = f[0];
    for i in 1..n {
        f_qum[i] = f_qum[i-1] + f[i];
    }

    f_qum.push(f[0]);

    let mut ans = 0;
    for i in 0..n {
        ans += f[i];
    }

    // 周遊パスを購入する枚数x
    let mut x_max = n / d;
    if n % d != 0 {
        x_max += 1;
    }
    for x in 1..x_max+1 {
        let mut temp_ans = 0;
        temp_ans += x * p;
        let free_days = x * d;
        if free_days >= n {
            
        }
        else {
            temp_ans += f_qum[n-1] - f_qum[free_days-1];
        }
        ans = min(ans, temp_ans);
    }
    println!("{}", ans);

}