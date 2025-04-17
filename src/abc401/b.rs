#![allow(dead_code, unused_imports)]
use proconio::{input, marker::{Usize1, Isize1, Chars}};
use itertools::Itertools;
use std::cmp::{max, min, Ordering, Reverse};
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::BTreeMap;
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::collections::{HashSet, BTreeSet};
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
use rand::Rng;
use num::{BigUint, ToPrimitive};
use num_bigint::ToBigUint;
fn main() {
    input! {
        n: usize,
        s: [String; n],
    }
    let mut is_login = false;
    let mut ans = 0;
    for i in 0..n {
        if s[i] == "login" {
            is_login = true;
        }
        else if s[i] == "logout" {
            is_login = false;
        }
        else if s[i] == "private" {
            if !is_login {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}