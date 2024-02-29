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
        m: usize,
        k: usize,
    }

    // 公約数はダメ
    // 公約数を考える

    let cd = gcd(min(n, m), max(n, m));
    let lcm = n / cd * m;

    // めぐる式二分探索
    let mut ng: i128 = 0;
    // let mut ok: usize = 1_000_000_000_000_000_000;
    let mut ok: i128 = (usize::MAX / 2 + usize::MAX / 4) as i128;
    while (ng as i128 - ok as i128).abs() > 1 {
        let mid = (ng + ok) / 2;
        if judge(mid as usize, lcm, n, m, k) {
            ok = mid;
        }
        else {
            ng = mid;
        }
    }
    println!("{}", ok);


}

fn judge(x: usize, lcm: usize, n: usize, m: usize, k: usize) -> bool {
    let num_n = x / n;
    let num_m= x / m;
    let num_lcm = x / lcm;
    let num_sum = num_n + num_m - 2 * num_lcm;
    // println!("x = {x}, num_sum = {:?}", num_sum);
    return k <= num_sum

}

// ユークリッドの互除法で最大公約数を求める (Euclidean Algorithm)
// ユークリッドの互除法とは、x < y のとき、gcd(x, y)=gcd(x, y % x)
fn gcd(x: usize, y:usize) -> usize {
    if y == 0{
        // 任意の整数xは0の約数と言える(∵0 % x == 0)ので、0とxの最大公約数はx
        return x
    }
    else {
        // return gcd(min(y, x % y), max(y, x % y));
        return gcd(y, x % y)
    }
}