#![allow(dead_code, unused_imports)]
use proconio::input;
use itertools::Itertools;
use core::num;
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
        a: isize,
        b: isize,
        c: isize,
    }
    let num_c = c.count_ones() as isize;
    let num_ab = a + b;
    // println!("num_c = {:?}", num_c);
    // println!("c = {:0b}", c);
    if a + b < num_c {
        // println!("x1");
        println!("-1");
        return
    }
    
    let num_diff = num_ab - num_c;
    if num_diff % 2 != 0 {
        // println!("x2");
        println!("-1");
        return
    }

    let mut num_dup = num_diff / 2;
    if num_dup > min(a,b) {
        // println!("x3");
        println!("-1");
        return
    }

    let mut num_sa = a - num_dup;
    let mut num_sb = b - num_dup;

    let mut x: isize = 0;
    let mut y: isize = 0;
    for i in 0..60 {
        if (1 << i) & c != 0 {
            if num_sa > 0 {
                num_sa -= 1;
                x += 1 << i;
            }
            else if num_sb > 0 {
                num_sb -= 1;
                y += 1 << i;
            }
        }
        else {
            if num_dup > 0 {
                num_dup -= 1;
                x += 1 << i;
                y += 1 << i;
            }
        }
    }

    // let final_c = 

    if x.count_ones() as isize == a && y.count_ones() as isize == b&& x ^ y == c {
        println!("{} {}", x, y);
    }
    else {
        println!("-1");
    }


}

fn pop_count(x: isize) -> isize {
    let mut count = 0;
    for i in 0..60 {
        if (1 << i) & x != 0 {
            count += 1;
        }
    }
    return count
}