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
        s: Chars
    }
    let mut remains = BTreeSet::new();

    let n = s.len();
    for i in 0..n {
        remains.insert(i);
    }

    for i0 in 0..n {
        if !remains.contains(&i0) {
            continue
        }
        if let Some(&i1) = remains.range(i0+1..).next() {
            if let Some(&i2) = remains.range(i1+1..).next() {
                if s[i0] == 'A' && s[i1] == 'B' && s[i2] == 'C' {
                    remains.remove(&i0);
                    remains.remove(&i1);
                    remains.remove(&i2);
        
                    let mut left = i0;
                    let mut right = i2;
                    
                    loop {
                        if let Some(&x1) = remains.range(..left).rev().next() {
                            if let Some(&x0) = remains.range(..x1).rev().next() {
                                if let Some(&x2) = remains.range(x1+1..).next() {
                                    if s[x0] == 'A' && s[x1] == 'B' && s[x2] == 'C' {
                                        remains.remove(&x0);
                                        remains.remove(&x1);
                                        remains.remove(&x2);
                                        left = x0;
                                        right = x2;
                                        continue
                                    }
                                }
                            }
                        }

                        if let Some(&x0) = remains.range(..left).rev().next() {
                            if let Some(&x1) = remains.range(right+1..).next() {
                                if let Some(&x2) = remains.range(x1+1..).next() {
                                    if s[x0] == 'A' && s[x1] == 'B' && s[x2] == 'C' {
                                        remains.remove(&x0);
                                        remains.remove(&x1);
                                        remains.remove(&x2);
                                        left = x0;
                                        right = x2;
                                    }
                                    else {
                                        break
                                    }
                                }
                                else {
                                    break
                                }
                            }
                            else {
                                break
                            }
                        }
                        else {
                            break
                        }
                    }
                }
            }
        }
    }
    for i in 0..n {
        if remains.contains(&i) {
            print!("{}", s[i]);
        }
    }



}