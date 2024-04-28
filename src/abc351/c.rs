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
        a: [usize; n],
    }

    let mut stack = vec![];

    for i in 0..n {
        stack.push(a[i]);

        loop {
            if stack.len() <= 1 {break}

            if stack[stack.len()-1] != stack[stack.len()-2] {break}
    
            let v0 = stack.pop().unwrap();
            let v1 = stack.pop().unwrap();
            let sum: usize = v0 + 1;
            stack.push(sum);
        }
        // println!("stack = {:?}", stack);
    }
    println!("{}", stack.len());

}