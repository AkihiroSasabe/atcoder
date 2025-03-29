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
    input! {
        n: usize,
        k: usize,
    }
    let mut a = vec![];
    let mut b = vec![];
    // let mut ab = vec![];
    use std::cmp::Reverse;

    let mut heap = BinaryHeap::new();
    for i in 0..n {
        input!{ 
            ai: usize,
            bi: usize,
        }
        heap.push(Reverse((ai, bi)));
        a.push(ai);
        b.push(bi);
    }
    // ab.sort();

    let mut life = k;
    let mut pos = 0;
    while life != 0 {

        pos = pos + life;
        life = 0;
        while let Some(Reverse((ai, bi))) = heap.pop() {
            if ai > pos {
                heap.push(Reverse((ai, bi)));
                break
            }
            else {
                life += bi;
            }
        }
    }
    println!("{}", pos);




}