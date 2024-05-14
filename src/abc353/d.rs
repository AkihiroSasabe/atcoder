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
        mut a: [usize; n]
    }
    a.reverse();

    let modulus = 998244353;
    let mut cum = vec![0; n];
    cum[0] = a[0];
    for i in 1..n {
        cum[i] += cum[i-1] + a[i];
        cum[i] %= modulus;
    }

    let mut num_digits = vec![];
    for i in 0..n {
        num_digits.push(get_digit_num(a[i]));
    }
    let mut power = vec![1; 100];
    for i in 1..100 {
        power[i] = power[i-1] * 10;
        power[i] %= modulus;
    }
    let mut cum2 = vec![0; n];
    cum2[0] = power[num_digits[0]];
    for i in 1..n {
        cum2[i] = cum2[i-1] + power[num_digits[i]];
        cum2[i] %= modulus;
    }
    
    let mut ans = 0;
    for i in 1..n {
        ans += cum[i-1];
        ans %= modulus;
        ans += cum2[i-1] * a[i] % modulus;
        ans %= modulus;
    }
    println!("{}", ans);

}

fn get_digit_num(mut x: usize) -> usize {
    let mut digit = 0;
    while x > 0 {
        digit += 1;
        x /= 10;
    }
    return digit
}