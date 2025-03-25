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
        n: usize
    }

    let mut max_num = 0;
    let mut temp = 1000;
    let mut pow = vec![1; 10];
    pow[1] = 1000;
    while temp <= n {
        max_num += 1;
        temp *= 1000;
        pow[max_num + 1] = temp;
    }

    let mut ans = 0;
    for num in 1..max_num+1 {
        if num == max_num {
            ans += (n + 1 - pow[num]) * num;
        }
        else {
            ans += (pow[num+1] - pow[num]) * num;
        }
    }
    println!("{}", ans);

    


    
}