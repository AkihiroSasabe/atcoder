#![allow(dead_code, unused_imports)]
use proconio::{input, marker::{Usize1, Isize1, Chars}};
use itertools::Itertools;
use std::cmp::{max, min, Ordering, Reverse};
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::BTreeMap;
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::collections::{HashSet, BTreeSet};
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
use rand::Rng;
use num::{BigUint, ToPrimitive};
use num_bigint::ToBigUint;
fn main() {
    // let ps = sieve_of_eratosthenes(1_000_000);
    // println!("ps.len() = {:?}", ps.len());
    // // ps.len() = 78_498
    // println!("ps = {:?}", ps);

    // 平方数なら何でもよくね?

    input! {
        q: usize,
        a: [usize; q]
    }
    let n = 1_000_000;
    // ps := n 以下の素数の集合
    let ps = sieve_of_eratosthenes(n);
    // println!("ps = {:?}", ps);
    // num_prime_list[x] := xの素因数の種類数
    let num_prime_list = pseudo_sieve_of_eratosthenes(n, &ps);
    // println!("num_prime_list = {:?}", num_prime_list);

    let mut set = BTreeSet::new();
    for x in 1..n+1 {
        if num_prime_list[x] == 2 {
            set.insert(x);
        }
    }
    // println!("set = {:?}", set);

    for i in 0..q {
        let ai = a[i];
        let ok = usize_floor_sqrt(ai);
        // let judge = |mid: usize| -> bool {
        //     return ai >=  mid * mid
        // };

        // let mut ng = 2_000_000_000;
        // let mut ok = 1;
        // if judge(ng) {
        //     ok = ng;
        // }
        // while (ng - ok) > 1 {
        //     let mid = (ng + ok) / 2;
        //     let is_ok = judge(mid);
        //     if is_ok {
        //         ok = mid;
        //     }
        //     else {
        //         ng = mid;
        //     }
        // }

        let ans = *set.range(..ok+1).rev().next().unwrap();
        println!("{}", ans * ans);

    }

}

fn usize_floor_sqrt(n: usize) -> usize {
    // エビちゃん ( @rsk0315 ) が作った、nの平方根 (= √n) 以下で最大の整数を取得する関数 
    // 二分探索で真面目に書いても良いかも。 (この関数は、u64だと動くけど、u128とかだと動くかは未検討なので注意)
    // https://atcoder.jp/contests/abc400/editorial/12642

    // saturating_sub は、除算で負になったときに、0を返してくれる。
    // assert_eq!(5usize.saturating_sub(2), 3);
    // assert_eq!(1usize.saturating_sub(2), 0);
    // https://doc.rust-lang.org/stable/std/primitive.usize.html#method.saturating_sub

    let tmp = (n as f64).sqrt() as usize;
    let tmp_m1 = tmp.saturating_sub(1);
    if tmp_m1 * (tmp_m1 + 2) < n { tmp } else { tmp_m1 }
}


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



// N以下の全ての整数について、素数の種類数を答えるO(NlogNlogNで求める)
fn pseudo_sieve_of_eratosthenes(n: usize, prime_list: &Vec<usize>) -> Vec<usize> {
    // num_prime_list[i] := 整数iの素因数の個数
    let mut num_prime_list = vec![0; n + 1]; // この初期化でO(N)かかる!
    num_prime_list[0] = 0; // 0は素数ではない
    num_prime_list[1] = 0; // 1は素数ではない


    // ここの計算で、O(N/2 + N/3 + N/5 + N/7 + N/11 + ...)  = O(N (1/2 + 1/3 + 1/5 + 1/7 + 1/11 + ... )) = O(Nlog(logN))かかる。
    // ※素数の逆数和は、log(logN)と漸近していくため。自然数の逆数和は、logNに漸近する。
    for &p in prime_list {
        for i in 1..n {
            let x = p * i;
            if x >= num_prime_list.len() {break}
            num_prime_list[x] += 1;
        }        
    }
    return num_prime_list

}



// エラトステネスの篩(ふるい)
// n以下の素数を全て列挙する為のアルゴリズムO(n*log(log(n)))
fn sieve_of_eratosthenes(n: usize) -> Vec<usize> {
    // is_prime_list[i] := iが素数なら true , そうでなければ false
    let mut is_prime_list = vec![true; n + 1]; // この初期化でO(N)かかる!
    is_prime_list[0] = false; // 0は素数ではない
    is_prime_list[1] = false; // 1は素数ではない

    // prime_list := n以下の素数を格納したリスト
    let mut prime_list = vec![];
    
    // ここの計算で、O(N/2 + N/3 + N/5 + N/7 + N/11 + ...)  = O(N (1/2 + 1/3 + 1/5 + 1/7 + 1/11 + ... )) = O(Nlog(logN))かかる。
    // ※素数の逆数和は、log(logN)と漸近していくため。自然数の逆数和は、logNに漸近する。
    for i in 2..(n+1) {
        if !is_prime_list[i] {continue}
        for j in 2..((n/i)+1) {
            // i の倍数が素数ではないことを記録
            is_prime_list[j * i] = false;
        }
        prime_list.push(i);
    }
    return prime_list;
}