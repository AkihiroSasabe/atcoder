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
    // 2023-12-08 17:07-18:34 (1h27min)
    input! {
        t: usize,
    }
    let mut n = vec![];
    let mut s = vec![];
    for i in 0..t {
        input! {
            ni: usize,
            si: Chars
        }
        n.push(ni);
        s.push(si);
    }
    const MODULUS: usize = 998244353;

    const N_MAX: usize = 1_000_001;
    // const N_MAX: usize = 10;
    let mut pow_26 = vec![1; N_MAX];
    for i in 1..N_MAX {
        pow_26[i] = pow_26[i-1] * 26 % MODULUS;
    }
    // println!("pow_26 = {:?}", pow_26);

    // Xは回文
    for i in 0..t {
        let mut ans = 0;
        let mut temp = 0;
        if n[i] % 2 == 0 {
            for j in 0..(n[i]/2) {
                temp += (s[i][j] as usize - 'A' as usize) * pow_26[n[i]/2-1-j] % MODULUS;
                temp %= MODULUS;
                // println!("temp = {:?}, n[i]/2-1-j = {}", temp, n[i]/2-1-j);
            }
            let mut bigger_flag = false;
            for j in 0..(n[i]/2) {
                if s[i][n[i]/2 - 1 - j] < s[i][j + n[i]/2] {
                    break;
                }
                else if s[i][n[i]/2 - 1 - j] > s[i][j + n[i]/2] {
                    bigger_flag = true;
                    break
                }
            }
            if !bigger_flag {
                temp += 1;
                temp %= MODULUS;
            }
            ans += temp;
            ans %=MODULUS;
        }
        // Xが奇数個の場合
        else {
            for j in 0..(n[i]/2+1) {
                temp += (s[i][j] as usize - 'A' as usize) * pow_26[n[i]/2-j] % MODULUS;
                temp %= MODULUS;
            }
            let mut bigger_flag = false;
            for j in 0..(n[i]/2) {
                if s[i][n[i]/2 - 1 - j] < s[i][j + 1 + n[i]/2] {
                    break;
                }
                else if s[i][n[i]/2 - 1 - j] > s[i][j + 1 + n[i]/2] {
                    bigger_flag = true;
                    break
                }
            }
            if !bigger_flag {
                temp +=  1;
                temp %= MODULUS;
            }
            ans += temp;
            ans %=MODULUS;
        }
        println!("{}", ans);
    }

}