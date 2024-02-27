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
        s: Chars,
        q: usize,
    }
    let mut c = vec![];
    let mut d = vec![];
    for i in 0..q {
        input!{
            ci: char,
            di: char
        }
        c.push(ci);
        d.push(di);
    }

    let mut abc = vec![];
    for i in 0..26 {
        abc.push(i);
    }

    for i in 0..q {
        for j in 0..26 {
            if abc[j] == c[i] as usize - 'a' as usize {
                abc[j] = d[i] as usize - 'a' as usize;
            }
        }
    }

    let lowercase: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();

    for i in 0..n {
        let ind = abc[s[i] as usize - 'a' as usize];
        print!("{}", lowercase[ind]);
    }

}