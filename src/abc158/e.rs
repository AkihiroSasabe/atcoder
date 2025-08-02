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
    // 2025-08-01 20:26-21:21 (55min)
    // 2025-08-02 9:30-10:07 (37min)
    // total: 1h32min
    input! {
        n: usize,
        p: usize,
        ss: Chars
    }
    
    let mut s = vec![];
    for i in 0..n {
        s.push(ss[i] as usize - '0' as usize);
    }
    s.reverse();

    if p == 5 {
        let mut ans = 0;
        for i in 0..n {
            if s[i] % 5 == 0 {
                ans += n - i;
            }
        }
        println!("{}", ans);
        return;
    }
    else if p == 2 {
        let mut ans = 0;
        for i in 0..n {
            if s[i] % 2 == 0 {
                ans += n - i;
            }
        }
        println!("{}", ans);
        return;
    }

    let mut pow10 = vec![1; n];
    for i in 1..n {
        pow10[i] = (pow10[i-1] * 10) % p;
    }

    // cum[i] := s[0] * 10^0 + s[1] * 10^1 + ... + s[i] * 10^i (mod p)
    let mut cum = vec![0; n];
    cum[0] = s[0] % p;
    for i in 1..n {
        cum[i] = (cum[i-1] + pow10[i] * s[i] % p) % p;
    }

    let mut counter = vec![0; p];
    for i in 0..n {
        counter[cum[i]] += 1;
    }
    // println!("cum = {:?}", cum);
    // println!("counter = {:?}", counter);

    // 1,10,100,1000,..,
    // 1の桁を絶対含む場合
    let mut ans = counter[0];
    for i in 1..n {
        counter[cum[i-1]] -= 1;
        // let inv10 = mod_inverse(pow10[i], p);

        // (x - cum[i-1]) / (10^i) % p == 0 になるようなxが知りたい。
        // (x - cum[i-1]) / (10^i) = k*p

        // x=cum[i] もしくは inv10 == 0 (mod p)
        // 10^i で割るという行為は、 p の倍数と関係ない...はず?
        // でも 10^i がp の倍数であるケースは考えないといけないかも...

        // p=2で、30->3への遷移を考える
        // p=2と5のときだけは、一番下の桁だけが、2または5で割り切れれば、その部分文字列は全てpで割れると言える。

        let cont: usize = counter[cum[i-1]];
        ans += cont;
        // println!("(i, cont) = {:?}", (i, cont));
    }
    println!("{}", ans);


}


// テストケース
// 2 2
// 30


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

