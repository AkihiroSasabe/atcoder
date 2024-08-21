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
        k: usize,
        r: [usize; n],
    }

    // let mut cands = vec![];
    let mut stack = vec![];
    dfs(0, &r, k, 0, &mut stack);


}

fn dfs(d: usize, r: &Vec<usize>, k: usize, sum: usize, stack: &mut Vec<usize>) {
    let n = r.len();
    if d == n {
        if sum % k == 0 {
            for s in stack.iter() {
                print!("{} ", s);
            }
            println!("");
        }
        return
    }

    for x in 1..r[d]+1 {
        stack.push(x);
        dfs(d+1, r, k, sum + x, stack);
        stack.pop();
    }
}