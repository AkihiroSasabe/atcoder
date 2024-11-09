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
        n: usize,
        q: usize,
    }
    let mut h = vec![];
    let mut t = vec![];
    for i in 0..q {
        input!{
            hi: char,
            ti: usize,
        }
        h.push(hi);
        t.push(ti-1);
    }


    let mut ans = 0;
    let mut l = 0;
    let mut r = 1;
    for i in 0..q {
        let mut ti = t[i];
        
        // println!("l = {}, r = {:?}, hi = {}, ti = {ti}", l, r, h[i]);
        let mut count = 0;
        let mut pos = l;
        let mut opp = r;
        if h[i] == 'R' {
            pos = r;
            opp = l;
        }

        let INF: usize = 1 << 60;
        let mut now = pos;
        let mut cr = 0;
        // 右回り
        loop {
            if now == ti {break}
            // 移動
            now += 1;
            now %= n;
            if now == opp {
                cr = INF;
                break
            }
            cr += 1;
        }

        // 左回り
        now = pos;
        let mut cl = 0;
        loop {
            if now == ti {break}
            // 移動
            if now == 0 {
                now = n - 1;
            }
            else {
                now -= 1;                
            }
            if now == opp {
                cl = INF;
                break
            }
            cl += 1;
        }

        if h[i] == 'R' {
            r = ti;
        }
        else {
            l = ti;
        }

        count = min(cl, cr);
        // println!("count = {:?}", count);
        ans += count;
    }
    // println!("l = {}, r = {:?}", l, r);

    println!("{}", ans);
}