#![allow(dead_code, unused_imports)]
use proconio::{input, marker::{Usize1, Isize1, Chars}};
use itertools::Itertools;
use std::cmp::{max, min, Ordering, Reverse};
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::BTreeMap;
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::collections::{HashSet, BTreeSet};
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
use rand::Rng;
use num::{BigUint, ToPrimitive};
use num_bigint::ToBigUint;
fn main() {
    input! {
        n: u128,
    }
    // a を決め打ち
    // b は

    // 1152921504606846976
    // 1000000000000000000

    let mut ans = 0;
    for a in 1..62 {
        let d = (1 as u128) << a;
        // めぐる式二分探索
        // 関数じゃなくて、クロージャーを使うと、引数を少なく出来る。
        let judge = |mid: u128| -> bool {
            return n >= d * mid * mid
        };

        let mut ng = 2_000_000_000;
        let mut ok = 0;
        if judge(ng) {
            ok = ng;
        }
        while (ng - ok) > 1 {
            let mid = (ng + ok) / 2;
            let is_ok = judge(mid);
            if is_ok {
                ok = mid;
            }
            else {
                ng = mid;
            }
        }

        // let q = (n as f64 / d as f64);
        // let aaa = q.sqrt().floor() as usize;
        let diff  = (ok + 1) / 2;

        // println!("{}", ok);

        ans += diff;
    }
    println!("{}", ans);

}