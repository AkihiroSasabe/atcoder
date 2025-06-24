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
        m: usize,
        mut a: [usize; m],
    }
    a.sort();
    a.dedup();
    a.push(n+1);
    // println!("a = {:?}", a);


    // k はデカいほどいい。
    let mut ok = 1;
    let mut ng = n;
    let judge = |mid: usize|-> bool {
        let mut now = 1;
        for i in 0..a.len() {
            let ai = a[i];
            let num = ai - now;
            if num == 0 {}
            else if num < mid {
                return false
            }
            now = ai+1;
        }
        return true
    };

    if judge(ng) {
        ok = ng;
    }
    while ng - ok > 1 {
        let mid = (ok+ng)/2;
        // println!("(ok, ng, mid) = {:?}", (ok, ng, mid));
        if judge(mid) {
            ok = mid;
        }
        else {
            ng = mid;
        }
    }
    // println!("ok = {:?}", ok);

    // ok := ハンコの最大長さ
    let mut ans = 0;
    let mut now = 1;
    for i in 0..a.len() {
        let length = a[i] - now;
        now = a[i] + 1;
        let r =length % ok;
        let q = length / ok;
        ans += q;
        if r != 0 {
            ans += 1;
        }
    }
    println!("{}", ans);




}