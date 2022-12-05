#![allow(dead_code, unused_imports)]
use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    input! {
        k: usize,
    }

    // 1. 素因数分解 k = p0^a0 * p1^a1 * ... * pm^am
    let prime_num_list = prime_factorize(k.clone());
    // println!("{:?}", prime_num_list);

    // 2. 各素因数piについて、N!がpi^aiの倍数となる最小のNiをそれぞれ求める
    // 3. N = max(N1, N2, ..., Nm) となるNを求める
    // 例えばもしk = 1458 = 2^1 * 3^6 だとしたら、N0=2, N1=15, N = max(N0, N1) = max(2,15)=15となる。
    // 2の計算方法: 15!内にある3の倍数を列挙すると、(1*3), (2*3), (3*3), (4*3), (5*3)で、3は6回かけられる。
    // 各項を(coeffient * p)とおき、これらの総乗のpの対数expが、a以上になるcoeffientを求める。
    // coeffientもpの倍数になるときがあるので注意
    let mut ans = 2;
    for i in 0..prime_num_list.len() {
        let mut coeffient = 0;
        let mut exp = 1;
        while exp <= prime_num_list[i][1]{
            coeffient += 1;
            exp += 1;
            let mut coeffient2 = coeffient;
            while coeffient2 % prime_num_list[i][0] == 0 {
                coeffient2 /= prime_num_list[i][0];
                exp += 1;
            }
        }
        // let cand = prime_num_list[i][0] * prime_num_list[i][1];
        let cand = prime_num_list[i][0] * coeffient;
        ans = max(ans, cand);
    }
    println!("{}", ans);


    // 検算
    // let mut ans = 0;
    // let mut kk = k;
    // println!("kk:{}", kk);

    // let mut kaijo: usize = 1;
    // for i in 2..(k+1) {
    //     kaijo *= i;
    //     if kaijo % kk == 0 {
    //         println!("i: {}, kk: {}, kaijo: {}", i, kk, kaijo);
    //         ans = i;
    //         break
    //     }
    // }
    // println!("{}", ans);

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