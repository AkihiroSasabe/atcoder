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
use proconio::marker::{Chars, Usize1};
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    input! {
        n: usize,
        
    }
    let mut a = vec![];
    let mut xys = vec![];
    for i in 0..n {
        input!{
            ai: usize,
            xysi: [(Usize1, usize); ai],
        }
        a.push(ai);
        xys.push(xysi);
    }
    // 32768
    // 7_372_800

    let mut ans = 0;
    for mask in 0..1<<n {
        let mut honests = vec![];
        let mut is_honest = vec![false; n];
        for i in 0..n {
            if mask & (1 << i) != 0 {
                honests.push(i);
                is_honest[i] = true;
            }
        }
        let mut is_ok = true;
        for i in 0..n {
            let ai = a[i];
            for j in 0..ai {
                let (x, y) = xys[i][j];
                if is_honest[i] {
                    if (y == 1 && !is_honest[x]) || (y==0 && is_honest[x]) {
                        is_ok = false;
                    }
                }
            }
        }

        if is_ok {
            ans = max(ans, honests.len());
        }
    }
    println!("{}", ans);
}