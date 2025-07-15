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
    // 2025-07-13 16:07-17:41 (1h34m)
    input! {
        n: u128,
    }
    let modulus = 998244353;

    // ◆解法
    // b <= √n, √n < b の2パターンで、解き方を分ける。
    // 計算量はO(√n)となる。 

    // (戦略1) b > n/√n のとき、区間毎に処理する。つまり、
    // b = (n/2+1..n), (n/3+1..=n/2), ..., (n/√n+1..=n/((√n)-1)) の区間毎に処理する。
    let root_n= (n as f64).sqrt().round() as u128;
    // println!("root_n = {:?}", root_n);
    let mut ans = 0;
    for i in 2..=root_n {
        // println!("i = {}, b > n/i = {}/{} = {} のとき ---- ---- ---- ----", i, n,i,n/i);

        // b > i のとき
        let max_b = min(n-1, (n / (i-1)));
        let min_b = n / i + 1;
        // println!("(max_b, min_b) = {:?}", (max_b, min_b));

        let min_term = n - max_b + 2 - i;
        let max_term = n - min_b + 2 - i;
        // println!("(min_term, max_term) = {:?}", (min_term, max_term));
        
        let diff = (min_term + max_term) * (1+max_b-min_b) / 2 % modulus;
        // let diff = (min_term + max_term) * (1+max_b-min_b) / 2;
        // println!("diff = {:?}", diff);
        ans += diff;
    }


    // (戦略2) b <= n/√n  のとき、bの値で全探索する
    for b in 2..=(n/root_n) {
        // println!("b = {:?} ---- ---- ---- ----", b);
        if n / b == 0 {continue}
        let num_denied_a = n / b - 1;
        let cont = (n - b - num_denied_a) % modulus;
        // println!("cont = {:?}", cont);
        ans += cont;
        ans %= modulus;
    }
    println!("{}", ans);

    // ◆問題要件
    // 以下を満たす(a,b,c) の組み合わせの個数
    // (1) 3 <= n <= 10^12
    // (2) 1 <= a,b,c <= n
    // (3) a != b, b != c, c != a
    // (4) a % b == c

    // (4)より、必然的に,
    // b>c,
    // a>=c
    // (3)より a!=c になるが、a==c の場合は、a>bのときなので、必然的に
    // b>c
    // a>c
    // a>b ということになる
    // c<b<a

    // ◆実験1
    // どれかを固定して、全探索するのが良い気もする。
    // n <= 100 という問題だったら?

    // b = 99 -> (a,c)=(100,1)
    // b = 98 -> (a,c)=(100,2), (99,1)
    // b = 97 -> (a,c)=(100,3), (99,2), (98,1)
    // b = 96 -> (a,c)=(100,4), (99,3), (98,2), (97,1)
    // b = 95 -> (a,c)=(100,5), (99,4), (98,3), (97,2), (96,1)
    // b = 94 -> (a,c)=(100,6), (99,5), (98,4), (97,3), (96,2), (95,1)
    // b = 93 -> (a,c)=(100,7), (99,6), (98,5), (97,4), (96,3), (95,2), (94,1)
    // ...
    // b = 50 -> (a,c)=_, (99,49),(98,48), (97,47), ..., (51,1)
    // 100 だけが駄目
    // 
    // b = 49 -> (a,c)=(100,2), (99,1), _, (97,48), ..., (50,1)
    // 98 だけが駄目
    // ...
    // b = 34 -> (a,c)=(100,32),(99,31),(98,30),...,(69,1),_,(67,33), ..., (35,1)
    // 68だけが駄目

    // b = 33 -> (a,c)=(100,1),_,(98,32),(97,31),...,()
    // // 99, 66が駄目

    // b = 30 -> (a,c)=(100,10),(99,9),(98,8),...,(91,1),_, (89,2),(88,1)
    // // 90,60 が駄目
    
    // b=29
    // // 87, 58 が駄目

    // b = 25
    // // 100,75,50 が駄目

    // b = 24
    // // 96, 72, 48 が駄目
    // ...
    // b = 3 -> (a,c)=(100,1),_, (98,2),(97,1),_, (95,2), ..., (7,1), _, (5,2), (4,1)
    // b = 2 -> (a,c)=(99,1),(97,1),(95,1), ..., (5,1),(3,1)
    //
    // c = a % b == 0 になっては駄目という条件はかなり効く。 
    // この時点で、(a,c)の候補数は、n*(b-1)/bとなる。

    // b=Biのとき、(n-Bi)個の候補があり、そのうちBiで割り切れるものは除外していく。


    // ◆実験1を抽象化
    // n> b >n/2 のとき Σ[b](n-b)個
    // b = n/2 のとき Σ[b](n-b)-1個
    // n/2 > b > n/3 のとき Σ[b](n-b)-1個
    // b = n/3 のとき Σ[b]n-b-2個
    // n/3 > b > n/4 のとき Σ[b]n-b-2個
    // b = n/4 のとき Σ[b]n-b-3個
    // n/3 > b > n/4

    // b > n/2: (100-99) + (100-98) + ... (100-51)
    // b > n/3: (100-50-1) + (100-49-1) + ... (100-34-1)
    // b > n/4: (100-33-2) + (100-32-2) + ... (100-26-2)
    // b > n/5: (100-25-3) + (100-24-3) + ... (100-21-3)
    // b > n/6: (100-20-4) + (100-19-4) + ... (100-17-4)
    // ...
    // b > n/10: (100-11-8)


    // なんとなく、戦略を2パターンに分けると良さそう.
    // (戦略1)b = n/2, n/3, ..., n/√n のときと
    // (戦略2)n/√n > b のとき

}