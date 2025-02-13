#![allow(dead_code, unused_imports)]
use proconio::{input, marker::{Usize1, Isize1}};
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
use rand::Rng;
use num::{BigUint, ToPrimitive};
use num_bigint::ToBigUint;
fn main() {
    input! {
        n: usize,
        q: usize,
    }

    // set := 複数鳩がいる場所の集合
    let mut set = BTreeSet::new();

    // hatos[i] : = 鳩iがいる場所
    let mut hatos = vec![];
    for i in 0..n {
        hatos.push(i);
    }

    // poss[i] := 場所i にいる鳩の集合
    let mut poss = vec![BTreeSet::new(); n];
    for i in 0..n {
        poss[i].insert(i);
    }

    for i in 0..q {
        input!{
            ki: usize,
        }
        if ki == 1 {
            input! {
                psi: Usize1,
                hsi: Usize1,
            }
            let pre_pos = hatos[psi];
            let next_pos = hsi;
            hatos[psi] = hsi;
            
            poss[pre_pos].remove(&psi);
            poss[next_pos].insert(psi);

            if poss[next_pos].len() >= 2 {
                set.insert(next_pos);
            }

            if poss[pre_pos].len() < 2 {
                if set.contains(&pre_pos) {
                    set.remove(&pre_pos);
                }
            }
        }
        else {
            println!("{}", set.len());
        }

    }
}