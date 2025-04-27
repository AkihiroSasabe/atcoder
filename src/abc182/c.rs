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
        nn: Chars
    }
    let mut n = nn.iter().map(|c| c.to_digit(10).unwrap() as usize % 3).collect::<Vec<usize>>();

    let sum: usize = n.iter().sum::<usize>() % 3;
    // println!("sum = {:?}", sum);

    let mut counter = vec![0; 3];
    for &ni in n.iter() {
        counter[ni] += 1;
    }

    if sum == 0 {
        println!("0");
        return;
    }
    else if sum == 1 {
        if counter[1] >= 1 && n.len() > 1 {
            println!("1");
            return;
        }
        else if counter[2] >= 2 && n.len() > 2 {
            println!("2");
            return;
        } else {
            println!("-1");
            return;
        }
    }
    else if sum == 2 {
        if counter[2] >= 1  && n.len() > 1 {
            println!("1");
            return;
        }
        else if counter[1] >= 2  && n.len() > 2{
            println!("2");
            return;
        } else {
            println!("-1");
            return;
        }
    }



}