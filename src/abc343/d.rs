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
        n: usize,
        t: usize,
    }

    let mut hash = HashMap::new();
    let mut points = vec![0; n];

    hash.insert(0, n);

    for i in 0..t {
        input!{
            aii: usize,
            bi: usize,
        }
        let ai = aii - 1;
        
        let p_before = points[ai];
        points[ai] += bi;
        let p_after = points[ai];

        reduce_from_hash(&mut hash, p_before);
        *hash.entry(p_after).or_insert(0_usize) += 1;
        println!("{}", hash.len());
        // println!("hash = {:?}", hash);

    }
}

// HashMapから要素xを1個減らす関数
fn reduce_from_hash<T: std::hash::Hash + Eq>(hash: &mut HashMap<T, usize>, x: T) {
    // hash := {key: 要素, value: 格納した要素の個数}
    // 連想配列hashにxが存在しないなら何もしない
    if !hash.contains_key(&x) {return}
    if hash[&x] == 1 {
        // 連想配列hashのxの個数が1個ならxのkeyごと消去する
        hash.remove(&x);
    }
    else {
        // 連想配列hashのxの個数を1個減らす
        *hash.get_mut(&x).unwrap() -= 1;
    }
}