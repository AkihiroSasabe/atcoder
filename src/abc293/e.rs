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
    input! {
        a: usize,
        x: usize,
        m: usize
    }
    // 剰余の法 M と互いに素とは限らない数 K で除算を行いたい場合、modMK で計算を行えばよい
    // 例えば、
    // ans = A / B % M 
    // は A= (qBM+r) とおくことで、
    // ans = (qBM+r) / B % M 
    //     = (qM + r/B) % M
    //     = (r/B) % M
    //     = (A%(BM) / B) % M
    // となる。ここでr=A%(BM)である
    //(a-1)S = a^X - a^0
    // <=> S = (a^X - a^0) / (a-1)
    //       = (a^x-1) / (a-1) && a != 1
    //       = (a^x-1) % (BM) / B % M && B := a-1 && a != 1

    if a == 1 {
        println!("{}", x % m);
    }
    else {
        let b = a-1;
        let bm = b*m;
        let remainder = (get_remainder_for_exp_func(a as u128, x as u128, bm  as u128) + bm - 1) % bm;
        let ans = remainder / b % m;
        println!("{}", ans);
    }

}

// base^(x) % mod を繰り返し二乗法により、O(log2(x))の計算量で求める　(O(x)だとTLE)
fn get_remainder_for_exp_func(mut base: u128, mut exponent: u128, modulo: u128) -> usize {
    // Example
    // 3^14 mod 100  を求める                                base = 3, exp = 14
    //     3^14  = (3^2)^7                         (mod 100) base = 3^2 = 9, exp = 7
    // <=> 3^14 = (3^2 % 100)^7                    (mod 100)    exp % 2 == 1 => remainder * base
    // <=> 3^14 = (3^4)^3 * (3^2)^1                (mod 100) base = 3^4 = 81, exp = 3
    // <=> 3^14 = (3^4 % 100)^3 * (3^2)^1          (mod 100) 
    // <=> 3^14 = (3^8)^1 * (3^4)^1 * (3^2)^1      (mod 100) base = 3^8 = 6561 = 61, exp = 1
    // <=> 3^14 = (3^8 % 100)^1 * (3^4)^1 * (3^2)^1(mod 100)
    // <=> 3^14 = (6561 % 100)^1 * (81)^1 * (9)^1  (mod 100)
    // <=> 3^14 = (61)^1 * (81)^1 * (9)^1          (mod 100)
    // <=> 3^14 = 44,469 = 69                      (mod 100)
    let mut remainder = 1;
    while exponent != 0 {
        if exponent % 2 == 1 {
            remainder = (remainder * base) % modulo;
        }
        // println!("base = {}, modulo = {}, base % modulo = {}", base, modulo, base % modulo);
        base = ((base % modulo) * (base % modulo)) % modulo;
        exponent /= 2;
    }
    return remainder as usize;
}
