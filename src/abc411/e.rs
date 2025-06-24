#![allow(dead_code, unused_imports)]
use proconio::{input, marker::{Usize1, Isize1, Chars}};
use itertools::{all, Itertools};
use std::cmp::{max, min, Ordering, Reverse};
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::collections::{VecDeque, BinaryHeap, HashMap, BTreeMap, HashSet, BTreeSet};
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
use rand::Rng;
use num::{BigUint, ToPrimitive, Integer};
use num_bigint::ToBigUint;
fn main() {
    // 2025-06-21 21:29-22:28 (59min)
    input! {
        n: usize,
        mut a: [[usize; 6]; n]
    }
    let modulus = 998244353;

    // ◆自力解法
    // ユニークなサイコロの目の値vが、高々 6N 個しかないことに着目。
    // 最大がvになる確率をP(v)とすると、
    // E = Σ P(v) * v
    // で計算できる。
    // P(v)は、vを上から順番に考えることで、P(v_i) から P(v_i+1) (※ v_i > v_i+1 ) を少ない計算量で導けることに気づく。
    // 具体的には、
    // P[j][vmax=v] := サイコロjの目の値がv以下になる確率 とすれば、
    // P(v_i) = P[0][vmax=v_i] * P[1][vmax=v_i] * ... * P[n-1][vmax=v_i] 
    //        - P[0][vmax=v_i+1] * P[1][vmax=v_i+1] * ... * P[n-1][vmax=v_i+1]
    // となる。vmax=v_i -> v_i+1 の変化の過程で、p[j][vmax]の更新は、せいぜい6N回しか起きない。
    // ◆計算量
    // O(NlogN)


    // ユニークなサイコロの目の値vを管理
    let mut set: BTreeSet<usize> = BTreeSet::new();

    // HashMap<目の値v, HashMap<サイコロのindex, Vec<サイコロの面のindex>>> 
    // val_to_saikoro := 目の値vは、どのサイコロの、どの面が保持しているか、を管理
    let mut val_to_saikoro: HashMap<usize, HashMap<usize, Vec<usize>>> = HashMap::new();
    for i in 0..n {
        for j in 0..6 {
            set.insert(a[i][j]);
            val_to_saikoro.entry(a[i][j]).or_insert(HashMap::new()).entry(i).or_insert(vec![]).push(j);
        }
    }
    // println!("val_to_saikoro = {:?}", val_to_saikoro);

    // counts[i] := i番目のサイコロの目で、vmax以下である面の個数。これを、動的に更新していく。
    let mut counts= vec![6; n];
    // サイコロの目がvmax=v 以下となる確率を、動的に計算していく。
    // 初期は、max(a[i][j]) なので、100 % 。
    let mut kakuritu = 1;

    let mut ans = 0;
    for &x in set.iter().rev() {
        // xを最大にする確率

        // x未満の個数
        // サイコロi 
        let mut n_kakuritu = kakuritu;
        for (&sai, vec) in val_to_saikoro.get(&x).unwrap().iter() {
            n_kakuritu = n_kakuritu * mod_dev(6, counts[sai], modulus) % modulus;
            counts[sai] -= vec.len();
            n_kakuritu = n_kakuritu * mod_dev(counts[sai], 6, modulus) % modulus;
        }
        ans = ans + x * (modulus + kakuritu - n_kakuritu) % modulus;
        ans %= modulus;
        kakuritu = n_kakuritu;
    }
    println!("{}", ans);

}


// mod p を法とした時の割り算 a / b の値
fn mod_dev(a: usize, b: usize, modulo: usize) -> usize {
    return a % modulo * mod_inverse(b, modulo) % modulo
}

// mod p を法とした時の逆数(逆元という) 1 / b の値
fn mod_inverse(a: usize, modulo: usize) -> usize {
    // フェルマーの小定理
    //     a^(p-1) = 1     (mod p)
    // <=> a * a^(p-2) = 1 (mod p)
    // <=> 1 / a = a^(p-2) (mod p)
    // ただし、法pは素数で、aはpの倍数ではない整数。
    // aがpの倍数だと、a^(p-1)=0 (mod p)となる。

    return mod_pow(a % modulo, modulo - 2, modulo)
}

// mod p を法とした時の累乗
// base^(x) % mod を繰り返し二乗法により、O(log2(x))の計算量で求める　(O(x)だとTLE)
// No.69参照
fn mod_pow(mut base: usize, mut exponent: usize, modulo: usize) -> usize {
    // 例: 3^4= (3^2)^2 = 9^2 = 81^1
    // 初期
    // 3^4
    // remainder=1
    // base=3
    // exp=4

    // i=0:
    // remainder = 1
    // base = 3 * 3 = 9
    // exp = 4 / 2 = 2

    // i=1:
    // remainder = 1
    // base = 9 * 9 = 81
    // exp = 2 / 2 = 1

    // i=2:
    // remainder = remainder * base = 81
    // base = 81 * 81
    // exp = 1 / 2 = 0

    base %= modulo;
    let mut remainder = 1;
    while exponent != 0 {
        if exponent % 2 == 1 {
            remainder = (remainder * base) % modulo;
        }
        base = (base * base) % modulo;
        exponent /= 2;
    }
    return remainder
}

