use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use proconio::marker::Chars;
fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
    }
    // 20:40~21:16
    // aの倍数の数
    let a_sum = arithmetic_sequence_sum(n, a);
    // bの倍数の数
    let b_sum = arithmetic_sequence_sum(n, b);
    // println!("{} {}", a_sum, b_sum);

    // aとbの最小公倍数
    let lcm = a * b / gcd(a, b);
    let lcm_sum = arithmetic_sequence_sum(n, lcm);

    let ans = (1+n)*n/2 - (a_sum+b_sum-lcm_sum);
    println!("{}", ans);
}

// 公差dでn以下の等差数列の和
fn arithmetic_sequence_sum(n: usize, d: usize) -> usize {
    if n < d {
        return 0
    }
    else if n == d {
        return d
    }
    else {
        let mut first_term = d;
        let mut final_term = n - n % d;
        let num = final_term / d;
        let sum = (first_term + final_term) * num / 2;
        return sum
    }
}

// ユークリッドの互除法で最大公約数を求める
// ユークリッドの互除法とは、x < y のとき、gcd(x, y)=gcd(x, y % x)
fn gcd(mut x: usize, mut y:usize) -> usize {
    if y <= x {
        let y_before = y;
        y = x;
        x = y_before;
    } 
    if y % x == 0 {
        return x
    }
    else {
        return gcd(x, y % x);
    }
}