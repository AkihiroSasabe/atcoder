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
        s1: String,
        s2: String,
    }
    if s1 == "sick" && s2 == "sick" {
        println!("1");
    }
    if s1 == "sick" && s2 != "sick" {
        println!("2");
    }
    if s1 != "sick" && s2 == "sick" {
        println!("3");
    }
    if s1 != "sick" && s2 != "sick" {
        println!("4");
    }
    

}