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
        n: isize,
        m: usize,
        t: isize,
        ab: [(isize, isize); m]
    }

    let mut life = n;
    let mut time = 0;
    for i in 0..m {
        let (ai, bi) = ab[i];
        // println!("life = {:?}", life);
        if life <= ai - time {
            println!("No");
            return;
        }
        life -= (ai-time);
        life += bi-ai;
        life = min(n, life);
        time = bi;
    }
    // println!("life = {:?}", life);
    // println!("(time) = {:?}", (time));
    if life <= t - time {
        println!("No");
    }
    else {
        println!("Yes");
    }


}