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
fn main() {
    // 2024-11-29 20:02-20:30 (28min)
    input! {
        h: usize,
        w: usize,
        a: usize,
        b: usize,
    }
    let modulus = 1_000_000_007;
    use crate::combination::Combination;
    let comb = Combination::new(h+w+1, modulus);

    // 
    let mut ans = 0;
    for x in b..w {
        let y = h - a - 1;
        // (0, 0) -> (y, x) にたどり着くルートの総数
        let num_mid = comb.get_comb(x + y, x); // 移動総回数はx+y回で、そのうち、右に曲がるx回を選択する組合せはx+y_C_x

        // (y+1, x) -> (h-1, w-1) にたどり着くルートの総数
        let num_fin = comb.get_comb(w-1-x + a-1, a-1);

        // (0, 0) -> (y, x) -> (y+1, x) -> (h-1, w-1) のルートの総数
        ans += num_mid * num_fin % modulus;
        ans %= modulus;
    }
    println!("{}", ans);
}

pub mod combination {
    #[derive(Debug, Clone)]
    pub struct Combination {
        modulus: usize,
        factorial: Vec<usize>,
        inv_factorial: Vec<usize>
    }
    impl Combination {
        pub fn new(max_n: usize, modulus: usize) -> Self {
            let (factorial, inv_factorial) = get_factorial(max_n, modulus);
            Combination {
                modulus,
                factorial,
                inv_factorial
            }
        }
        // nCrを求める
        pub fn get_comb(&self, n: usize, r: usize) -> usize {
            // nCr = n! / ((n-r)! * r!) % modulus ;
            // n!は事前にメモ化して計算済み
            // 分母の逆数(逆元)は、フェルマーの小定理により求める
            let top = self.factorial[n];
            let bottom = self.inv_factorial[n-r] * self.inv_factorial[r] % self.modulus;
            let ncr = (top * bottom) % self.modulus;
            return ncr
        }
    }

    // フェルマーの小定理x^(p-1) = 1 (mod p)により逆元を求める x^(-1) = x ^ (p - 2) (mod p)
    pub fn get_inverse(x: usize, modulus: usize) -> usize {
        // x^(p-2)はO(p-2)の計算量がかかってしまうが、繰り返し二乗法で、O(log2(p-2))まで落とせる。
        let inverse =  iterative_square_method(x, modulus - 2, modulus);
        return inverse;
    }

    fn get_factorial(max_n: usize, modulus: usize) -> (Vec<usize>, Vec<usize>) {
        // n!を格納した配列
        let mut factorial = vec![1; max_n+1];
        let mut inv_factorial = vec![1; max_n+1];
        for i in 1..(max_n+1) {
            factorial[i] = (i * factorial[i-1]) % modulus;
        }
        inv_factorial[max_n] = get_inverse(factorial[max_n], modulus);
        for i in 1..max_n {
            inv_factorial[max_n - i] = inv_factorial[max_n - i + 1] * ((max_n - i + 1) % modulus);
            inv_factorial[max_n - i] %= modulus;
        }

        return (factorial, inv_factorial)
    }
    
    // 繰り返し2乗法 a^xを求める
    fn iterative_square_method(mut a: usize, mut x: usize, modulus: usize) -> usize {
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
    
        a %= modulus;
        let mut answer = 1;
        while x >= 1 {
            if x % 2 == 1 {
                answer = (answer * a) % modulus;
            }
            x = x / 2;
            a = a * a % modulus;
        }
    
        return answer;
    }
}
