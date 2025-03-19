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

    // x^3 - y^3 = n
    // (x-y)(x^2+xy+y^2) = n
    // (x-y)^2 = (x^2-2xy+y^2) < (x^2+xy+y^2) であるから、(∵x,y>0)
    // (x-y)^3 < (x-y)(x^2+xy+y^2) = n で
    // (x-y) < n^(1/3) <= (10^18)^(1/3)=10^6
    // となることがわかる。なので、(x-y)=1 ~ 10^6 の範囲で、
    // (x-y)(x^2+xy+y^2) = n 
    // x-y=d
    // x = d + y
    // x^2 + xy + y^2 = 3*y^2 + 3*d*y + d^2

    // solve(n);
    solve2(n);

}

fn solve2(n: usize) {
    // d := x-y
    let mut d = 0;
    while {
        d += 1;
        d * d * d <= n
    } {
        if n % d != 0 {continue}
        let q = n / d;

        let root = (12.0 *q as f64 - 3.0 *(d as f64) * d as f64).sqrt();
        if root.is_nan() {continue}
        let root = root.round() as usize;
        if root < 3*d {continue}
        let top = root-3*d;
        if top % 6 != 0 {continue}
        let y = top / 6;
        let x = y + d;
        if y > 0 && x > 0 && d * (x*x+x*y+y*y) == n {
        // if y > 0 && x > 0  {
            
            println!("{} {}", x, y);
            return;
        }
    }
    println!("-1");
}


fn solve(n: usize) {
    let devisors = enumerate_devisors(n);
    // println!("devisors = {:?}", devisors);
    for d in devisors {
        let q = n / d;
        if d > q {break}
        if 4*q < d*d {continue}
        if (4*q - d*d) % 3 != 0 {continue}
        let square = (4*q - d*d) / 3;

        let root = (square as f64).sqrt();
        if root.is_nan() {continue}
        let root = root.round() as usize;

        // let pes = prime_factorize(square);
        // let mut is_ok = true;
        // let mut root = 1;
        // for (p,e) in pes {
        //     if e % 2 != 0 {
        //         is_ok = false;
        //     }
        //     for _ in 0..e/2 {
        //         root *= p;
        //     }
        // }
        // if !is_ok {continue}
        if (root + d) % 2 != 0 {continue}
        if (root - d) % 2 != 0 {continue}
        let x = (root + d) / 2;
        let y = (root - d) / 2;

        if x > 0 && y > 0 && (x-y)*(x*x+x*y+y*y) == n {
            println!("{} {}", x, y);
            return;
        }
    }
    println!("-1");
}


// 繰り返し二乗法


// 素因数分解
fn prime_factorize(mut x: usize) -> Vec<(usize, usize)> {
    // prime_num_list[i] := (素数p_i, 指数exp_i) が、格納されたリスト
    // 
    // 例: x = 48 = 2^4 * 3^1 のとき、
    // let prime_num_list: Vec<(usize, usize)> = prime_factorize(48);
    // prime_num_list = [(2, 4), (3, 1)]

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
            prime_num_list.push((i, exponent));
        }
    }
    if x != 1 {
        prime_num_list.push((x, 1));
    }
    return prime_num_list
}

/// 自然数Nの約数をO(√N)で昇順に全列挙する関数。
/// 約数の個数は意外と少ない https://github.com/E869120/kyopro_educational_90/blob/main/editorial/085-01.jpg
/// N <= 10^6 なら、N=720,720 が最大で、240個
/// N <= 10^9 なら、N=735,134,400 が最大で、1,344個
/// N <= 10^12 なら、N=963,761,198,400 が最大で、6,720個
/// N <= 10^18 なら、N=897,612,484,786,617,600 が最大で、103,680個
fn enumerate_devisors<T>(n: T) -> Vec<T> 
    where T: 
        Default + 
        Copy + 
        std::fmt::Display + 
        std::cmp::PartialEq + 
        std::ops::Not<Output = T> + 
        std::ops::Add<Output = T> +
        std::ops::Div<Output = T> +
        std::ops::Rem<Output = T> + 
        std::ops::Mul<Output = T> +
        std::cmp::PartialOrd

{
    // 自然数Nの約数を列挙する関数O(√N)
    let mut devisors = vec![];
    let mut devisors_over_root_n = vec![]; // √N より大きい約数を一時的に格納するリスト

    let zero: T = Default::default();
    let not_zero: T = !zero;
    let one: T = not_zero / not_zero;

    let mut devisor = one;
    while (devisor * devisor * devisor) <= n {
        if n % devisor == zero {
            devisors.push(devisor);
            if n / devisor != devisor {
                devisors_over_root_n.push(n / devisor);                
            }
        }
        devisor = devisor + one;
    }
    // println!("devisors_over_root_n = {:?}", devisors_over_root_n);

    for &devisor_over_root_n in devisors_over_root_n.iter().rev() {
        devisors.push(devisor_over_root_n);
    }
    return devisors
}

