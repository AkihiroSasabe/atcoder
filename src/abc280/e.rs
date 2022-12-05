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
        n: usize,
        p: usize
    }
    let MODULO = 998244353;
    let inverse_100 = get_inverse(100, MODULO);

    // n!を格納された配列
    let mut memo_factorial = vec![1;n+1];
    for i in 1..(n+1) {
        memo_factorial[i] = (i * memo_factorial[i-1]) % MODULO;
    }

    // 期待値の計算
    let mut expect = 0;
    let atack_min = n / 2 + n % 2;
    let atack_max = n;

    // 攻撃回数が最小の時の期待値を求める
    let num_2 = atack_min;
    let num_1 = 0;
    // println!("num_1: {}, num_2: {}", num_1, num_2);
    let probability = iterative_square_method(p * inverse_100 % MODULO, num_2, MODULO);
    expect = (expect + atack_min * probability % MODULO) % MODULO;
    if n % 2 == 1 {
        let num_2 = atack_min-1;
        let num_1 = 1;
        let ncr = combination(atack_min, num_1, MODULO, &memo_factorial);
        // println!("num_1: {}, num_2: {}, ncr: {}", num_1, num_2, ncr);
        let prob2 = iterative_square_method(p * inverse_100 % MODULO, num_2, MODULO);
        let prob1 = (100 - p) * inverse_100 % MODULO;
        let probability = ncr * prob2 % MODULO * prob1 % MODULO;

        expect = (expect + atack_min * probability % MODULO) % MODULO;
    }
    for (i, atack) in (atack_min+1..atack_max+1).enumerate() {
        // println!("==============================");
        // println!("i:{} atack_min: {}, atack_max: {}", i, atack_min, atack_max);
        // 最後に体力がちょうど0になるように攻撃する場合
        let mut num_2 = atack_min - (i+1);
        if n % 2 == 1 {
            num_2 -= 1;
        }
        let num_1 = atack - num_2;
        let ncr = combination(atack, num_2, MODULO, &memo_factorial);
        // println!("num_1: {}, num_2: {}, ncr: {}", num_1, num_2, ncr);
        let prob_2 = iterative_square_method(p * inverse_100 % MODULO, num_2, MODULO);
        let prob_1 = iterative_square_method((100 - p) * inverse_100 % MODULO, num_1, MODULO);
        let mut probablity = ncr * prob_2 % MODULO * prob_1 % MODULO;

        // 最後に体力がちょうど-1になるように攻撃(over-kill)する場合 (最後は必ず2で攻撃する)
        let num_2 = num_2 + 1;
        let num_1 = num_1 - 1;
        let ncr = combination(atack-1, num_1, MODULO, &memo_factorial);
        // println!("num_1: {}, num_2: {}, ncr: {}", num_1, num_2, ncr);
        let prob_2 = iterative_square_method(p * inverse_100 % MODULO, num_2, MODULO);
        let prob_1 = iterative_square_method((100 - p) * inverse_100 % MODULO, num_1, MODULO);
        probablity += ncr * prob_2 % MODULO * prob_1 % MODULO;
        
        // 期待値の計算
        expect += probablity % MODULO * atack % MODULO;
        expect %= MODULO;
    }
    println!("{}", expect);

    // 逆元
    // nCrの求め方

}


// 繰り返し2乗法 a^xを求める
fn iterative_square_method(mut a: usize, mut x: usize, MODULO: usize) -> usize {
    // answer = a ^ x を得たいとき
    //        = (a^2)^(x/2) * a^(x%2)

    // answer = 3 ^3 を得たいとき
    //        = (3^2)^(3/2) * 3^(3%2)
    //        = 9^1 * 3^1

    // answer = 3 ^ 4 を得たいとき
    //        = (3^2)^(4/2) * (3^2)^(4%2)
    //        = 9^2 * 3^0
    //        = (9^2)^(2/2) * 9^(2&2) * 3^0
    //        = 81^1 * 9^0 * 3^0

    // answer = 3 ^ 5を得たいとき
    // answer = (3^2)^(5/2) * 3^(5%2)
    //        = (3^2)^2 * 3^1
    //        = ((3^2)^2)^(2/2) * (3^2)^(2%2) * 3^1
    //        = ((3^2)^2)^1 * (3^2)^0 * 3^1
    //        = (3^4)^1 * (3^2)^0 * 3^1

    // answer = 3 ^ 7を得たいとき
    // answer = (3^2)^(7/2) * 3^(7%2)
    //        = (3^2)^3 * 3^1
    //        = 9^3 * 3^1
    //        = (9^2)^(3/2) * 9^(3%2) * 3^1
    //        = 81^1 * 9^1 * 3^1

    a %= MODULO;
    let mut answer = 1;
    while x >= 1 {
        if x % 2 == 1 {
            answer = (answer * a) % MODULO;
        }
        x = x / 2;
        a = a * a % MODULO;
    }

    return answer;
}

// フェルマーの小定理x^(p-1) = 1 (mod p)により逆元を求める x^(-1) = x ^ (p - 2) (mod p)
fn get_inverse(x: usize, MODULO: usize) -> usize {
    // x^(p-2)はO(p-2)の計算量がかかってしまうが、繰り返し二乗法で、O(log2(p-2))まで落とせる。
    let inverse =  iterative_square_method(x, MODULO - 2, MODULO);
    return inverse;
}

// nCrを求める
fn combination(n: usize, r: usize, MODULO: usize, memo_factorial: &Vec<usize>) -> usize {
    // nCr = n! / ((n-r)! * r!) % MODULO ;
    // n!は事前にメモ化して計算済み
    // 分母の逆数(逆元)は、フェルマーの小定理により求める
    let top = memo_factorial[n];
    let bottom = ((memo_factorial[n-r]) * (memo_factorial[r])) % MODULO;
    let ncr = (top * get_inverse(bottom, MODULO)) % MODULO;
    return ncr
}