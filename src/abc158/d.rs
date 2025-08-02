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
        s: Chars,
        q: usize,
    }
    let mut k = vec![];
    let mut f = vec![];
    let mut c = vec![];
    for i in 0..q {
        input!{
            ki: usize,
        }
        k.push(ki);
        if ki == 1 {
            f.push(0);
            c.push('a');
        }
        else if ki == 2 {
            input!{
                fi: usize,
                ci: char,
            }
            f.push(fi);
            c.push(ci);
        }
    }


    let mut queue = VecDeque::new();
    for i in 0..s.len() {
        queue.push_back(s[i]);
    }

    let mut is_reverse = false;
    for i in 0..q {
        if k[i] == 1 {
            is_reverse = !is_reverse;
        } else if k[i] == 2 {
            let fi = f[i];
            let ci = c[i];

            if is_reverse {
                if fi == 1 {
                    // 末尾
                    queue.push_back(ci);
                } else if fi == 2 {
                    // 先頭
                    queue.push_front(ci);
                }
            } else {
                if fi == 1 {
                    // 先頭
                    queue.push_front(ci);
                } else if fi == 2 {
                    // 末尾
                    queue.push_back(ci);
                }
            }
        }
    }
    while queue.len() > 0 {
        if is_reverse {
            print!("{}", queue.pop_back().unwrap());
        } else {
            print!("{}", queue.pop_front().unwrap());
        }
    }

}