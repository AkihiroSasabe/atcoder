#![allow(dead_code, unused_imports)]
use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::BTreeMap;
use std::hash::Hash;
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::collections::{HashSet, BTreeSet};
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    input! {
        n: usize,
        mut s: Chars,
        q: usize,
    }
    let mut t = vec![];
    let mut x = vec![];
    let mut c = vec![];
    for i in 0..q {
        input! {
            t_i: usize,
            x_i: usize,
            c_i: char,
        }
        t.push(t_i);
        if t_i == 1 {
            x.push(x_i - 1);
        }
        else {
            x.push(x_i);
        }
        c.push(c_i);
    }
    

    let mut last_size_change_index = q;
    let mut last_size = 1;
    for i in (0..q).rev() {
        if t[i] == 2 || t[i] == 3 {
            last_size_change_index = i;
            last_size = t[i];
            break
        }
    }


    for i in 0..q {
        if t[i] == 1 {
            s[x[i]] = c[i];
        }
    }

    for i in 0..n {
        if last_size == 2 {
            // 全て小文字に。
            s[i] = to_lowercase(s[i]);
        }
        else if last_size == 3 {
            // 全て大文字に。
            s[i] = to_uppercase(s[i]);
        }
    }

    for i in last_size_change_index..q {
        if t[i] == 1 {
            s[x[i]] = c[i];
        }
    }
    for i in 0..s.len() {
        print!("{}", s[i]);
    }
}

fn to_lowercase(x: char) -> char {
    let x_u8 = x as u8;
    if 65 <= x_u8 && x_u8 <= 90 {
        // xが'A' ~ 'Z'であれば、'a' ~ 'z' に変換して返す
        let x_lowercase = (x_u8 + 32) as char;
        return x_lowercase
    }
    else {
        // 何もしないでそのまま返す
        return x
    }
}

fn to_uppercase(x: char) -> char {
    let x_u8 = x as u8;
    if 97 <= x_u8 && x_u8 <= 122 {
        // xが'a' ~ 'z'であれば、'A' ~ 'Z' に変換して返す
        let x_uppercase = (x_u8 - 32) as char;
        return x_uppercase
    }
    else {
        // 何もしないでそのまま返す
        return x
    }
}