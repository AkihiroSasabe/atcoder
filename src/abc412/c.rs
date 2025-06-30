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
        t: usize,

    }
    let mut n = vec![];
    let mut s = vec![];
    for i in 0..t {
        input!{
            ni: usize,
            si: [usize; ni],
        }
        n.push(ni);
        s.push(si);
    }

    for i in 0..t {
        let ni = n[i];
        let mut si = s[i].clone();
        let a0 = si[0];
        let an = si[ni-1];

        si.pop();
        si.remove(0);
        si.sort();
        si.insert(0, a0);

        let mut ans: usize = 2;
        let mut now = a0;
        let mut is_ok = false;

        let mut pre_ind = 0;
        loop {
            if now * 2 >= an {
                is_ok = true;
                break
            }
            else {
                if si.len() == 1 {
                    break
                }
                if pre_ind == si.len()-1 {
                    break
                }


                ans += 1;
                let ind = si.upper_bound(&(now*2)) - 1;
                if pre_ind == ind {
                    break
                }
                let cand = si[ind];
                pre_ind = ind;
                now = cand;
            }
        }

        if is_ok {
            println!("{}", ans);
        }
        else {
            println!("-1");
        }
    }

    // s[i] * 2 >= s[i+1] ならi+1も倒れる

}