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
        d: isize,
    }

    let mut x: isize = 0;
    let mut ans = 1_000_000_000_000_000_000;
    while x * x <= 100_000_000_000_000 {
        let z = x * x - d;
        if z >= 0 {
            ans = min(ans, z);
        }
        else {
            let diff = get_w(1, z) - get_w(0, z);
            if diff >= 0 {
                ans = min(ans, get_w(0, z));
                ans = min(ans, get_w(1, z));
                ans = min(ans, get_w(2, z));
            }
            else {
                // 2分探索
                let mut ok = 0;
                let mut ng = 2_000_000_001;
                while (ng as i128 - ok as i128).abs() > 1 {
                    let mid = (ng + ok) / 2;
                    let diff = get_w(mid + 1, z) - get_w(mid, z);
                    if diff <= 0 {
                        ok = mid;
                    }
                    else {
                        ng = mid;
                    }
                }
                ans = min(ans, get_w(ok, z));
                ans = min(ans, get_w(ok + 1, z));
                ans = min(ans, get_w(ok + 2, z));
            }
        }
        x += 1;
    }
    println!("{}", ans);

}

fn get_w(y: isize,  z: isize) -> isize {
    let w = (y * y + z).abs();
    return w
}