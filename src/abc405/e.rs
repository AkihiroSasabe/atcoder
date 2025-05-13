#![allow(dead_code, unused_imports)]
use proconio::{input, marker::{Usize1, Isize1, Chars}};
use itertools::Itertools;
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
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    }
    let modulus = 998244353;
    use crate::combination::Combination;
    let n = a + b + c + d;

    let comb = Combination::new(n, modulus);

    let mut ans = 0;
    let cont0 = comb.get_comb(a+b, b);
    let cont1 = comb.get_comb(c+d, c);
    ans = cont0 * cont1 % modulus;

    for num in a+b+1..a+b+c+1 {
        let banana_left = num - (a+b);
        let banana_right = c - banana_left;
        let cont0 = comb.get_comb(num-1, b-1);
        let cont1 = comb.get_comb(banana_right + d, d);
        let diff = (cont0 ) % modulus * cont1 % modulus;
        // println!("(num, cont0, cont1, diff) = {:?}", (num, cont0, cont1, diff));
        // println!("diff = {:?}", diff);
        ans += diff;
        ans %= modulus
    }
    println!("{}", ans);
    // count += comb.get_comb(n - (x-1) * (k-1), x);
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



fn solve(a: usize, b: usize, c: usize, d: usize, ) {
    let modulus = 998244353;
    let mut set = BTreeMap::new();
    let ans = dfs(a,b,c,d,&mut set);
    println!("{}", ans);
}

fn dfs(a: usize, b: usize, c: usize, d: usize, set: &mut BTreeMap<(usize, usize, usize, usize), usize>) -> usize {
    let modulus = 998244353;
    if a == 0 && b == 0 && c == 0 && d == 0 {
        set.insert((0,0,0,0), 1);
        return 1;
    }
    if set.contains_key(&(a,b,c,d)) {
        let ans = *set.get(&(a,b,c,d)).unwrap();
        return ans;
    }

    let mut ans = 0;
    if a != 0 {
        ans += dfs(a-1, b, c, d, set);
        ans %= modulus;
    }
    if b != 0 {
        ans += dfs(a, b-1, c, d, set);
        ans %= modulus;
    }
    if c != 0 && a == 0  {
        ans += dfs(a, b, c-1, d, set);
        ans %= modulus;
    }
    if d != 0 && a ==0 && b == 0 {
        ans += dfs(a, b, c, d-1, set);
        ans %= modulus;
    }

    set.insert((a,b,c,d), ans);
    return ans
}


    // // dp[りんごfull?][おれんじfull?][ばななfull?][ぶどうfull?]
    // let mut dp = vec![vec![vec![vec![vec![0; 2]; 2]; 2]; 2]; n+1];

    // // if a == 1 {
    // //     dp[1][0][0][0] = 1;
    // // }
    // // if b == 1 {
    // //     dp[0][1][0][0] = 1;
    // // }
    // dp[0][0][0][0][0] = 1;
    // dp[a][1][0][0][0] = 1;
    // dp[b][0][1][0][0] = 1;
    // for num in 1..n+1 {
    //     dp[num][]
    // }
    

// }