#![allow(dead_code, unused_imports)]
use proconio::{input, marker::{Usize1, Isize1, Chars}};
use itertools::Itertools;
use std::cmp::{max, min, Ordering, Reverse};
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::collections::{VecDeque, BinaryHeap, HashMap, BTreeMap, HashSet, BTreeSet};
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
use rand::Rng;
use num::{BigUint, ToPrimitive, Integer};
use num_bigint::ToBigUint;
fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut t = vec![];
    let mut p = vec![];
    let mut x = vec![];
    for i in 0..q {
        input!{
            ti: usize,
            pi: usize,
        }
        t.push(ti);
        p.push(pi);

        if ti == 1 {
            input! {
                xi: usize,
            }
            x.push(xi);
        } else {
            x.push(0);
        }
    }

    let mut a = vec![0; n];
    for i in 0..n {
        a[i] = i + 1;
    }


    let mut head_index = 0;
    for i in 0..q {
        if t[i] == 1 {
            let p = p[i] - 1;
            let ind = (p + head_index) % n;
            a[ind] = x[i];
        }
        else if t[i] == 2 {
            let p = p[i] - 1;
            let ind = (p + head_index) % n;
            println!("{}", a[ind]);            
        }
        else if t[i] == 3 {
            let k = p[i];
            head_index = k % n + head_index;
            head_index %= n;
        }
    }




}