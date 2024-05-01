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
    // 2024-04-30 11:09-11:52 (43min)
    // 2024-04-30 11:52-12:10 (18min)
    // ひらめき 11:52
    // WA: 12:10
    // 2024-05-01 8:32-9:36 (1h4min, 解説AC, 競プロフレンズのtweetみた。)
    // Total 2h5min
    // 自分は、1人の部屋が何個あるか、で場合分けしようとして、解けなかったが、
    // kyopro_friendsは、0人の部屋が何個あるか、で場合分けしていた。
    input! {
        n: usize,
        k: usize,
    }
    let modulus = 1_000_000_007;

    // 人の移動: 部屋i -> 部屋j
    let mut factorial = vec![1; 2*n+10];
    for i in 1..2*n+10 {
        factorial[i] = i * factorial[i-1] % modulus;
    }
    let mut ans = 0;
    for i in 0..min(k+1, n) {
        // i := 0人の部屋の個数
        // N個の部屋から、i個の0人部屋を選択: nCi
        // 「N人を、N-i個の部屋で分け合う。ただし、各部屋は確実に1人以上持つこと。」: n-1_C_i
        // 証明：
        // 先に、N-i人を1人ずつN-i個の部屋に配っておいて、残りの人を分配することを考える。
        // <=> 「i人(=N-(N-i))を、N-i個の部屋で分け合う。ただし、各部屋は0人でもいい。」
        // ボールi個, 仕切り: N-i-1個: i+(n-i-1)_C_i = n-1_C_i
        ans += get_ncr(n, i, modulus, &factorial) * get_ncr(n-1, i, modulus, &factorial) % modulus;
        ans %= modulus;
    }
    println!("{}", ans);
}

fn get_ncr(n: usize, r: usize, modulus: usize, factorial: &Vec<usize>) -> usize {

    let bunsi = factorial[n];
    let bunbo1 = mod_inverse(factorial[r], modulus);
    let bunbo2 = mod_inverse(factorial[n-r], modulus);
    let ncr = bunsi * bunbo1 % modulus * bunbo2 % modulus;
    return ncr
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



// https://qiita.com/drken/items/3b4fdf0a78e7a138cd9a
// 「1000000007 で割ったあまり」の求め方を総特集！ 〜 逆元から離散対数まで 〜 by けんちょん
// 【導入】
// 逆元の計算例
// 9 / 4 = 12 (mod 13)　が成り立つことを示す。：
// <=> 9 = 4 * 12 (mod 13)
// <=> 9 = 48 (mod 13)
// <=> 9 = 3 * 13 + 9 (mod 13)

// 【定理】
// p: 素数
// b: pで割れない切れない整数
// bx = a (mod p)
// を満たすxが一意に存在する

// 【mod pにおける逆元】
// 	  a / b (mod p)
// 	= a * (1 / b) (mod p)
// つまり、1 / b (mod p)が計算出来ればよい。
// 【定義】(1 / b)は「mod pにおけるbの逆元」という。
// 【定義】bをかけると、1になる数 (mod pの元で)

// ★逆元の求め方は2つ。計算量はO(logP)
// 1. Fermatの小定理を利用		（法pが素数でないと使えない。実装は楽）
// 素数pについて、aをpの倍数ではない整数として、下記が成立する
// 		a^(p-1) = 1 (mod p)
// ===========================================================
// 	<=>	a * a^(p-2) = 1(mod p)
// 	<=>	a^(p-2)がmod pにおけるaの逆元
// よってa^(p-2) mod pを求めれば良い。愚直にやると、O(p-2)かかるが、
// 繰り返し二乗法(90選のNo.69)でO(log2(p-2))の時間で計算できる

// 2. 拡張Euclidの互除法を利用	（法pが素数でなくても、逆元存在条件を満たせばok）
// 		a * x = 1 (mod p)
// 	<=>	a * x + p * y = 1 を満たす整数yが存在する
// 上記を満たすxを拡張Euclidの互除法で求める
// 	ax + by = 1	(a,bは互いに素。整数x,yを求める)

// 一般に
// 	ax + by = c　が解をもつとき、cがgcd(a,b)で割り切れる。	(*1)
// そこでc = c'gcd(a,b)とおくと、
// 	ax + by = c'gcd(a,b)
// また、一般に
// 	ax' + by' = gcd(a,b)					(*2)
// も成り立つ。


// ★逆元が存在する条件
// 逆元は常に存在しない。
// mod p でのaの逆元が存在する条件は、pとaとが互いに素であること。
	