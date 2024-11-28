#![allow(dead_code, unused_imports)]
use proconio::{input, marker::Usize1};
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
fn main() {
    input! {
        s: Chars
    }

    let mut num1 = 0;
    let mut num2 = 0;
    let mut num3 = 0;

    for si in s {
        if si == '1' {
            num1 += 1;
        }
        else if si == '2' {
            num2 += 1;
        }
        else if si == '3' {
            num3 += 1;
        }
    }

    if num1 == 1 && num2 == 2 && num3 == 3 {
        println!("Yes");
    }
    else {
        println!("No");

    }

}