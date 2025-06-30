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
        mut a: [usize; n], // ふれんどりさ
    }
    a.sort();
    a.reverse();

    // if n == 2 {
    //     println!("{}", a[0]);
    //     return;
    // }

    let mut heap = BinaryHeap::new();
    heap.push((a[1], a[0]));
    heap.push((a[1], a[0]));
    let mut ans = a[0];
    for i in 2..n {
        let (v_min, v_max) = heap.pop().unwrap();
        ans += v_min;
        heap.push((a[i], v_min));
        heap.push((a[i], v_max));
    }
    println!("{}", ans);



    // let ans = a[1..].iter().sum::<usize>();
    // println!("{}", ans);
    

}