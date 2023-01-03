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
use std::collections::HashSet;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    // 2023-01-04 Wed. 16:36-17:31 (55min)
    input! {
        n: usize
    }
    let mut MODULO = 998244353;
    let num_of_digits = get_num_of_digits(n);
    let mut ans = 0;
    for digit in 1..num_of_digits {
        ans = (ans + get_sum_of_f_for_same_digit(digit, MODULO)) % MODULO;
    }
    ans = (ans + get_sum_of_f_for_final_digit(n, num_of_digits, MODULO)) % MODULO;
    println!("{}", ans);
}

fn get_num_of_digits(mut n: usize) -> usize {
    let mut count = 0;

    while n != 0 {
        count += 1;
        n /= 10;
    }
    return count
}

fn get_sum_of_f_for_same_digit(num_of_digits: usize, modulo: usize) -> usize {
    let max_f = (10_usize.pow(num_of_digits as u32) - 10_usize.pow(num_of_digits as u32 - 1)) % modulo;
    let sum = (max_f + 1) % modulo * max_f / 2 % modulo;
    return sum
}

fn get_sum_of_f_for_final_digit(n: usize, num_of_digits: usize, modulo: usize) -> usize {
    let max_f = (n + 1 - 10_usize.pow(num_of_digits as u32 - 1)) % modulo;
    let sum = (max_f + 1) % modulo * max_f / 2 % modulo;
    return sum
}


// 考察
// ■x=1のとき
// 1で1個ある
// f(1) = 1

// ■x=2のとき
// 1,2で2個ある
// f(2) = 2

// ■x=3のとき
// 1,2,3で3個ある
// f(3) = 3

// ■x=10のとき
// 10の1個
// f(10) = 1

// ■x=20のとき
// 20の1個
// f(20)=1


// 1の位の総和
// f(1) + ... f(9)
// = 1 + ... + 9
// = (1+9) * (9 - 1 + 1) / 2
// = 10*9/2
// = 45

// 2の位の総和
// f(10) + ... + f(99)
// = 1 + ... + 90
// = (1+90) * (99 - 10 + 1) / 2
// 90

// 3の位の総和: 
// f(100) + ... + f(999)
// = 1 + ... + (999 - 99)
// = (1+ 900) * (999 - 100 + 1) / 2
// 900

// 2の位の途中
// f(10) + ... f(16)
// = 1 + ... + 7
// = (1+7) * (16-10+1) / 2

