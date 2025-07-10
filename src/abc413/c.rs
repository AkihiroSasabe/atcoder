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
        q: usize,

    }
    let mut t = vec![];
    let mut c = vec![];
    let mut x = vec![];
    for i in 0..q {
        input!{
            ti: usize,
            ci: usize,
        }
        t.push(ti);
        c.push(ci);
        if ti == 1 {
            input! {
                xi: usize,
            }
            x.push(xi);
        } else {
            let xi = 0;
            x.push(xi);
        }
    }

    let mut deque = VecDeque::new();
    for i in 0..q {
        let ti = t[i];
        let ci = c[i];
        let xi = x[i];

        if ti == 1 {
            deque.push_back((xi, ci));
        }
        else {
            let mut ki = ci;
            let (xx, cc) = deque.pop_front().unwrap();
            let mut ans = 0;
            if ki <= cc {
                ans += ki * xx;
                deque.push_front((xx, cc - ki));
                println!("{}", ans);
            }
            else {
                ki -= cc;
                ans += cc * xx;
                loop {
                    let (xx, cc) = deque.pop_front().unwrap();
                    if ki <= cc {
                        ans += ki * xx;
                        deque.push_front((xx, cc - ki));
                        println!("{}", ans);
                        break;
                    }
                    else {
                        ki -= cc;
                        ans += cc * xx;
                    }
                }
            }
        }


    }



    



}