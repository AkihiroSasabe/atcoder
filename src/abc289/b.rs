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
    input! {
        n: usize,
        m: usize,
        a: [usize; m],
    }
    // println!("{:?}", a);

    let mut ans = vec![];
    let mut stack = vec![];
    let mut reten_ind = 0;
    let mut reten = n;
    if 0 < a.len() {
        reten = a[0];    
    }
    else {
        reten = n+1;
    }
    for i in 1..n+1 {
        // println!("i: {}", i);
        if reten == i {
            stack.push(i);
            reten_ind += 1;
            if reten_ind < a.len() {
                reten = a[reten_ind];    
            }
            else {
                reten = n+1;
            }
        }
        else {
            ans.push(i);
            while stack.len() != 0 {
                let v = stack.pop().unwrap();
                ans.push(v);
            }
        }
        // println!("ans: {:?}", ans);
        // println!("stack: {:?}", stack);
    }
    for i in 0..ans.len() {
        // let ind = ans.len() - 1 - i;
        // print!("{} ", ans[ind]);
        print!("{} ", ans[i]);
    }

}
