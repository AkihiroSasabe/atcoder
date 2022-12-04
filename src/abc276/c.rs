#![allow(dead_code, unused_imports)]
use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    input! {
        n: usize,
        p: [usize; n]
    }
    let mut perm: Vec<usize> = (1..(n+1)).collect();
    // println!("{:?}", perm);
    let mut ketu= p[n-1];
    let mut cand = vec![];
    let mut ketu_index = n - 1;
    for i in 0..n {
        if p[n-1-i] > ketu {
            ketu_index = n-1-i;
            break
        }
        else {
            ketu = p[n-1-i];
            cand.push(ketu);
        }
    }
    for i in 0..ketu_index {
        print!("{} ", p[i]);
    }


    cand.sort();
    cand.reverse();
    for i in 0..cand.len() {
        if p[ketu_index] > cand[i] {
            print!("{} ", cand[i]);
            cand[i] = p[ketu_index];
            break
        }
    }

    for i in 0..cand.len() {
        if cand[i] == p[ketu_index] - 1 {
            print!("{} ", p[ketu_index]);
        }
        else {
            print!("{} ", cand[i]);
        }
    }

}