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
use std::collections::HashSet;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    // 2023-01-03 10:44-11:40 (56min)
    input! {
        q: usize,
    }

    let mut command = vec![];
    let mut x = vec![];
    let mut k = vec![];
    for i in 0..q {
        input! {
            command_i: usize,
            x_i: usize,
        }
        if command_i != 1 {
            input! {
                k_i: usize
            }
            k.push(k_i);
        }
        else {
            k.push(0);
        }
        command.push(command_i);
        x.push(x_i);
    }
    let mut btree = BTreeMap::new();
    for i in 0..q {
        // println!("btree: {:?}", btree);
        if command[i] == 1 {
            if !btree.contains_key(&x[i]) {
                btree.insert(x[i],1_usize);                
            }
            else {
                *btree.get_mut(&x[i]).unwrap() += 1;
            }
        }
        else if command[i] == 2 {
            let mut count = k[i];
            let mut exist = false;
            for (&key, &value) in btree.range(..x[i]+1).rev() {
                // println!("key, value:{}, {}", key, value);
                if count > value {
                    count -= value;
                }
                else {
                    println!("{}", key);
                    exist = true;
                    break
                }
            }
            if !exist {
                println!("-1");
            }
        }
        else if command[i] == 3 {
            let mut count = k[i];
            let mut exist = false;
            for (&key, &value) in btree.range(x[i]..) {
                // println!("key, value:{}, {}", key, value);
                if count > value {
                    count -= value;
                }
                else {
                    println!("{}", key);
                    exist = true;
                    break
                }
            }
            if !exist {
                println!("-1");
            }
        }
    }
}