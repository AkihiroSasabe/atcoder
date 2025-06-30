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
        mut s: Chars,   
        q: usize,   
    }
    let mut t = vec![];
    let mut a = vec![];
    let mut b = vec![];
    for i in 0..q {
        input!{
            ti: usize,
            ai: usize,
            bi: usize,
        }
        t.push(ti);
        a.push(ai);
        b.push(bi);
    }

    // let mut num_2 = 0;
    let mut is_change = false;
    for i in 0..q {
        let ti = t[i];
        if ti == 1 {
            let mut ai = a[i] - 1;
            let mut bi = b[i] - 1;
            if is_change {
                // n=3
                // 0,1,2 3,4,5 
                if ai < n {
                    ai = n + ai; 
                }
                else {
                    ai = ai - n;
                }
                if bi < n {
                    bi = n + bi;
                }
                else {
                    bi = bi - n;
                }
            }
            s.swap(ai, bi);

        }
        else if ti == 2 {
            is_change = !is_change;
        }
    }
    let mut ans = vec![];
    if is_change {
        for i in n..2*n {
            ans.push(s[i]);
        }
        for i in 0..n {
            ans.push(s[i]);
        }
    }
    else {
        ans = s;
    }
    println!("{}", ans.iter().collect::<String>());

}