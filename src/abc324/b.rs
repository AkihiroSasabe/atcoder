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
        mut n: usize
    }

    while n % 2 == 0 {
        n = n / 2;
    }
    while n % 3 == 0 {
        n = n / 3;
    }
    if n == 1 {
        println!("Yes");
    }
    else {
        println!("No");
    }

    // let a = prime_factorize(n);
    // if a.len() == 2 && a[0][0] == 2 && a[1][0] == 3 {
    //     println!("Yes");
    // }
    // else if a.len() == 1 && (a[0][0] == 2 || a[0][0] == 3) {
    //     println!("Yes");
    // }
    // else if n == 1 {
    //     println!("Yes");
    // }
    // else {
    //     println!("No");
    // }
}

// 素因数分解
fn prime_factorize(mut x: usize) -> Vec<Vec<usize>> {
    // let root_x = (x as f64).sqrt() as usize;
    let mut prime_num_list = vec![];
    let mut i = 1;
    while i * i <= x {
    // for i in 2..(root_x+1) {
        i += 1;
        let mut exponent = 0;
        while x % i == 0 {
            x /= i;
            exponent += 1;
        }
        if exponent != 0 {
            prime_num_list.push(vec![i, exponent]);
        }
    }
    if x != 1 {
        prime_num_list.push(vec![x, 1]);
    }
    return prime_num_list
}