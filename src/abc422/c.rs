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
        t: usize
    }
    let mut a = vec![];
    let mut b = vec![];
    let mut c = vec![];
    for i in 0..t {
        input!{
            ai: usize,
            bi: usize,
            ci: usize,
        }
        a.push(ai);
        b.push(bi);
        c.push(ci);
    }
    for i in 0..t {
        let ai = a[i];
        let bi = b[i];
        let ci = c[i];
        let ac_min = min(ai, ci);
        let ac_max = max(ai, ci); 
        let amari = bi + ac_max - ac_min;
        if ac_min <= amari {
            println!("{}", ac_min);
            continue;
        }

        // めぐる式二分探索
        // 関数じゃなくて、クロージャーを使うと、引数を少なく出来る。
        let judge = |mid: usize| -> bool {
            return ac_min - mid <= amari + 2 * mid
        };
        // fn judge(mid: usize) -> bool {
        //    return true
        // }
        
        let mut ng = 0;
        let mut ok = ac_min;
        if judge(ng) {
            ok = ng;
        }
        while (ng as i128 - ok as i128).abs() > 1 {
            let mid = (ng + ok) / 2;
            let is_ok = judge(mid);
            if is_ok {
                ok = mid;
            }
            else {
                ng = mid;
            }
        }
        let ans = ac_min - ok;
        println!("{}", ans);
        // println!("{}", ok);
    }
}