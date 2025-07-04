#![allow(dead_code, unused_imports)]
use proconio::{input, marker::{Usize1, Isize1, Chars}};
use itertools::Itertools;
use std::{cmp::{max, min, Ordering, Reverse}, thread::current};
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::collections::{VecDeque, BinaryHeap, HashMap, BTreeMap, HashSet, BTreeSet};
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
use rand::Rng;
use num::{BigUint, ToPrimitive, Integer};
use num_bigint::ToBigUint;
fn main() {
    // 2025-07-03 (Thu.) 12:31-12:45 (14min)　色々考察して包除原理っぽいことには気が付きつつ、
    // 2025-07-03 (Thu.) 18:30-19:35 (1h5min)
    // Total: 1h19min
    // 詰めきれず競プロフレンズの解説ツイートをみた
    // 包除原理
    // A∪B∪C∪D = 
    //     + (A+B+C+D) 
    //     - (A∩B + A∩C + A∩D + B∩C + B∩D + C∩D) 
    //     + (A∩B∩C + B∩C∩D + C∩D∩A + D∩A∩B) 
    //     - A∩B∩C∩D

    // s0 := a[0] == b[0] の集合
    // s1 := a[1] == b[1] の集合
    // ...
    // sn-1 := a[n-1] == b[n-1] の集合
    // 求めたいのは、
    // U - |s0 ∪ s1 ∪ s2 ∪ ... ∪ sn-1|
    // である。
    // 全集合 U = (M * (M-1) * ... * (M-N+1)) ^ 2 = (M!/(M-N)!)^2 である。
    // 
    // ここで、|s0| = |s1| = ... = |sn-1| であり、
    // |s0 ∩ s1| = |s0 ∩ s2| = .... = |sn-2 ∩ sn-1|
    // |s0 ∩ s1 ∩ s2| = |s0 ∩ s1 ∩ s3| = .... = |sn-3 ∩ sn-2 ∩ sn-1|
    // ...
    // であることに注意すると、
    // |s0| = M * ((M-1) * ... * (M-N+1))^2 
    // |s0 ∩ s1| = M * (M-1) * ((M-2) * ... * (M-N+1))^2 
    // |s0 ∩ s1 ∩ s2| = M * (M-1) * (M-2) * ((M-3) * ... * (M-N+1))^2 

    input! {
        n: usize,
        m: usize,
    }
    // n <= 4
    // m <= 6
    // brute_force(n,m);
    solve(n,m);
    // 3 4
    // brute=264

}

fn solve(n: usize, m: usize) {
    // 2025-07-03 19:06-
    let modulus = 1_000_000_007;
    use crate::combination::Combination;
    let comb: Combination = Combination::new(n+1, modulus);

    let mut facts = vec![1; n+1];
    for i in (0..n) {
        facts[i+1] = facts[i] * (m-n+1+i) % modulus;
    }
    let u = facts[n] * facts[n] % modulus;
    // println!("u = {:?}", u);

    let mut complementary_event = 0;
    for num_same in 1..n+1 {
        let mut current = facts[n] * facts[n-num_same] % modulus;
        current = current * comb.get_comb(n, num_same) % modulus;
        if num_same % 2 == 1 {
            complementary_event = modulus + complementary_event + current;
        }
        else {
            complementary_event = modulus + complementary_event - current;
        }
        complementary_event = complementary_event % modulus;
    }
    let ans = (modulus + u - complementary_event) % modulus;
    println!("{}", ans);


}

fn solve_botu(n: usize, m: usize) {
    let modulus = 1_000_000_007;
    use crate::combination::Combination;
    let comb: Combination = Combination::new(n+1, modulus);

    // let mut dp = vec![0; n];
    // 余事象?
    // 全事象 (2個目の条件だけ)
    // (m*(m-1)*(m-2)*...*(m-n-1))^2
    // a[i] == b[i] となるiが何個あるか?
    // 
    // 0個だけ等しいケース <- これが知りたい
    // 1個だけ等しいケース ... 
    // 2個だけ等しいケース ... 
    // ...
    // N-2個全部等しいケース ... m*(m-1)*(m-2)*...*(m-n-1) * nC2 * (m-(n-1))
    // N-1個全部等しいケース ... m*(m-1)*(m-2)*...*(m-n-1) * nC1 * (m-n)
    // N個全部等しいケース ... m*(m-1)*(m-2)*...*(m-n-1) 通り


    // a0  b0      a1      b1
    // 1   2,3     2,3     
    // 2   1,3     
    // 3   1,2

    let mut facts = vec![1; m+1];
    for i in 2..m+1 {
        facts[i] = facts[i-1]*i%modulus;
    }

    let mut inv_facts = facts.clone();
    for i in 1..m+1 {
        inv_facts[i] = mod_inverse(facts[i], modulus);
    }

    // println!("facts = {:?}", facts);
    // println!("inv_facts = {:?}", inv_facts);

    let mut memo = vec![1; n+1];
    let mut seen = vec![false; n+1];
    let mut memo2 = vec![0; n+1];
    let mut seen2 = vec![false; n+1];
    // let ans = dfs(n,m,&facts, &inv_facts,modulus,&mut memo, &mut seen,&mut memo2, &mut seen2, &comb);
    // println!("seen = {:?}", seen);
    // println!("memo = {:?}", memo); // 1,2,18
    // println!("seen2 = {:?}", seen2);
    // println!("memo2 = {:?}", memo2); // 2.18,0

    // memo[n] := 数列の長さがnで、m種類の整数で、条件を満たすやつ
    // memo[n-1] := 数列の長さがn-1で、m-1種類の整数で、条件を満たすやつ

    // n=2, m=3
    // n=1, m=2 -> 2通り
    // n=0, m=1 -> 0通り...

    for i in 1..n+1 {
        let root0 = facts[m-n+i] * inv_facts[m-n] % modulus;
        let mut ans = root0 * root0 % modulus;
        ans = (modulus + ans - (m-n+i) * memo[i-1] % modulus * comb.get_comb(i,i-1) % modulus) % modulus;
        // ans = memo2[i-1] * (m-n+i) % modulus;
        ans = (modulus + ans - (m-n+i) * memo2[i-1] % modulus) % modulus;

        memo[i] = ans;
        memo2[i] = (modulus + root0 * root0 % modulus - ans) % modulus;
    }

    println!("{}", memo[n]);
    // 881613484
    // 601640857
    // 1000000000
    
}


fn brute_force(n: usize, m: usize) {
    // n <= 4
    // m <= 6

    fn dfs(d: usize, n: usize, m: usize, seen_a: &mut Vec<bool>, seen_b: &mut Vec<bool>, ans: &mut usize) {
        if d == n {
            *ans += 1;
            return;
        }
        for ai in 1..m+1 {
            for bi in 1..m+1 {
                if ai == bi {continue;}
                if seen_a[ai] || seen_b[bi] {continue}
                seen_a[ai] = true;
                seen_b[bi] = true;
                dfs(d+1, n,m,seen_a,seen_b,ans);
                seen_a[ai] = false;
                seen_b[bi] = false;
            }
        }
    }
    let mut seen_a = vec![false; m+1];
    let mut seen_b = vec![false; m+1];
    let mut ans = 0;
    dfs(0,n,m,&mut seen_a, &mut seen_b, &mut ans);
    let modulus = 1_000_000_007;
    println!("brute={}", ans%modulus);

}


fn dfs(n: usize, m: usize, fact: &Vec<usize>, inv_fact: &Vec<usize>, modulus: usize, memo: &mut Vec<usize>, seen: &mut Vec<bool>, memo2: &mut Vec<usize>, seen2: &mut Vec<bool>, comb: &crate::combination::Combination) -> usize {
    // println!("n = {:?}", n);

    if seen[n] {
        return memo[n]
    }

    // let root0 = fact[m] * inv_fact[m-n] % modulus;
    let root0 = fact[m] / fact[m-n] % modulus;
    let mut ans = root0 * root0 % modulus;

    let mut elem0 = m;
    for i in 1..n+1 {
        let elem1 = if seen[n-1] {
            memo[n-1]
        } else {
            dfs(n-i, m-i, fact, inv_fact, modulus, memo, seen, memo2, seen2, comb)
        };
        if seen2[n-i] {
            ans = (modulus + ans - m * memo2[n-i] % modulus) % modulus;
            break
        }
        ans = (modulus + ans - elem0 * elem1 % modulus * comb.get_comb(n,i) % modulus) % modulus;
        elem0 *= (m-i) % modulus;
        elem0 %= modulus;
    }
    seen[n] = true;
    memo[n] = ans;
    println!("n = {:?}", n);
    // println!("(n, ans, root0) = {:?}", (n, ans, root0));
    if n != 0 {
        seen2[n-1] = true;
        memo2[n-1] = (modulus + root0 * root0 % modulus - ans) % modulus;
    }
    return ans
}

// 141421 
// 122031




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

