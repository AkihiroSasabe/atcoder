#![allow(dead_code, unused_imports)]
use proconio::{input, marker::{Usize1, Isize1}};
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
use num::{BigUint, ToPrimitive};
use num_bigint::ToBigUint;
fn main() {
    // 2025-03-24 19:56-20:29 (33min)
    input! {
        n: usize, // 荷物
        m: usize, // 箱
        q: usize,
    }
    let mut ws = vec![];
    let mut vs = vec![];
    for i in 0..n {
        input!{
            wsi: usize, // 重さ
            vsi: usize, // 価値
        }
        ws.push(wsi);
        vs.push(vsi);
    }
    input! {x: [usize; m]} // 箱のキャパ
    let mut ls = vec![];
    let mut rs = vec![];
    for i in 0..q {
        input!{
            lsi: Usize1,
            rsi: Usize1,
        }
        ls.push(lsi);
        rs.push(rsi);
    }

    // 価値が高いものを貪欲にしまうか?
    // let mut heap = BinaryHeap::new();
    let mut goods = vec![];
    for i in 0..n {
        // heap.push((v[i], w[i]));
        goods.push((vs[i], ws[i]));
    }
    goods.sort();
    goods.reverse();
    let mut x2 = vec![];
    // let mut x2 = BinaryHeap::new();
    for i in 0..m {
        x2.push((x[i], i));
    }
    // x2.sort();
    // x2.reverse();
    for i in 0..q {
        let mut heap = x2.clone();

        let mut ans = 0;
        for j in 0..n {
            let (vi, wi) = goods[j];
            let mut is_ok = false;
            let mut backs = vec![];
            heap.sort();
            heap.reverse();
            while let Some(elem) = heap.pop() {
                let box_size = elem.0;
                let box_ind: usize = elem.1;
                if ls[i] <= box_ind && box_ind <= rs[i] {continue}
                if box_size >= wi {
                    is_ok = true;
                    break
                }
                backs.push(elem);
            }
            for elem in backs {
                heap.push(elem);
            }
            if is_ok {
                ans += vi;
            }
        }
        println!("{}", ans);
    }



}