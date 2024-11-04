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
use rand::Rng;
fn main() {
    input! {
        n: usize
    }
    let mut qs = vec![];
    let mut rs = vec![];
    for i in 0..n {
        input!{
            qi: usize,
            ri: usize,
        }
        qs.push(qi);
        rs.push(ri);
    }
    input! {
        q: usize
    }
    let mut ts = vec![];
    let mut ds = vec![];
    for i in 0..q {
        input!{
            ti: usize, // 種類
            di: usize, // 出された日
        }
        ts.push(ti-1);
        ds.push(di);
    }

    for i in 0..q {
        let ti = ts[i]; // 種類
        let di = ds[i]; // 出された日
        if di % qs[ti] == rs[ti] {
            println!("{}", di);
        }
        else {
            let ri = di % qs[ti];
            let mut ans = di;
            if ri <= rs[ti] {
                ans = di + rs[ti] - ri;
            }
            else {
                ans = di + qs[ti] + rs[ti] - ri;
            }
            println!("{}", ans);
        }
    }

}