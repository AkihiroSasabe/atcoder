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
    input! {
        s: Chars,
        n: u128
    }
    let mut t_min: u128 = 0;
    let mut t_max: u128 = 0;
    let mut hatena = vec![];
    for i in 0..s.len() {
        let index = s.len() - 1 - i;
        if s[i] == '0' {
            continue
        }
        else if s[i] == '1' {
            t_min += 1 << index;
            t_max += 1 << index;
        }
        else {
            t_max += 1 << index;
            hatena.push(index);
        }
    }
    // println!("hatena: {:?}", hatena);
    // println!("t_max: {}, t_min: {}", t_max, t_min);

    if t_min > n {
        println!("-1");
        // dbg!("aaa");
        return
    }
    if t_max <= n {
        println!("{}", t_max);
        // dbg!("bbb");
        return
    }

    let mut ng: u128 = (1 << hatena.len()) - 1;
    let mut ok: u128 = 0;
    while (ng as i128 - ok as i128).abs() > 1 {
        let mid = (ng + ok) / 2;
        // println!("ok: {} ng: {} mid: {}", ok, ng, mid);
        let mut t_num = t_min;
        for i in 0..hatena.len() {
            if mid & (1 << i) != 0 {
                let index = hatena.len() - 1 - i;
                t_num += 1 << hatena[index];
                // t_num += 1 << hatena[i];
            }
        }
        if t_num <= n {
            ok = mid;
        }
        else {
            ng = mid;
        }
    }

    let mut t_num = t_min;
    for i in 0..hatena.len() {
        if ok & (1 << i) != 0 {
            let index = hatena.len() - 1 - i;
            t_num += 1 << hatena[index];
        }
    }

    println!("{}", t_num);
    // dbg!("ccc");



}