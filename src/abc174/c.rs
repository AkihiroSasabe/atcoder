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






    // let ps = sieve_of_eratosthenes(1_000_000);
    // println!("{:?}", ps);
    // println!("{:?}", ps.len());
    // 78_498 
    // 10-6以下の素数は7.8 * 10^4 個くらいある。

    // 777 [(3, 1), (7, 1), (37, 1)]
    // 777777777 [(3, 2), (7, 1), (37, 1), (333667, 1)]
    // 777777777777 [(3, 1), (7, 2), (11, 1), (13, 1), (37, 1), (101, 1), (9901, 1)]
    // 777777777777777 [(3, 1), (7, 1), (31, 1), (37, 1), (41, 1), (271, 1), (2906161, 1)]
    // 777777777777777777 [(3, 2), (7, 2), (11, 1), (13, 1), (19, 1), (37, 1), (52579, 1), (333667, 1)]

    // k=5なら? -k=5、きつい。
    // 
    // 7 + 70 + 700 + 7000 + 
    // 2 + 0 + 0 + 0 + ...

    // k=25なら?
    // 7 + 70 + 700 + 7000 + 
    // 7 + 20 + 0 + 0 + ...

    // let k = 101;
    // let mut x = 7;
    // for _ in 0..20 {
    //     let r = x % k;
    //     println!("x = {x}, r = {:?}", r);
    //     x *= 10;
    // }


    // k=

    // let mut x = 7;
    // for _ in 0..19 {
    //     let ps = prime_factorize(x);
    //     println!("{:020} {:?}",x, ps);
    //     x *= 10;
    //     x += 7;
    // }

    // 7 % k
    // 70 % k
    // 700 %k

    input! {
        k: usize
    }
    // k が偶数なら一発アウト
    // if k % 2 == 0 {
    //     println!("-1");
    //     return;
    // }

    // 19:05- ひらめき
    // let k = 101;
    let mut x = 7;
    let mut set = BTreeMap::new();
    let mut rs = vec![];
    let mut count = 0;
    let mut st_loop = 0;
    let mut len_loop = 1;
    let mut cum = 0;

    // ループは、k回以内 (10^6)に終わる。
    loop {
        count += 1;
        let r = x % k;
        cum += r;
        cum %= k;
        // println!("r = {}, cum = {:?}", r, cum);

        if cum == 0 {
            println!("{}", count);
            return;
        }

        if set.contains_key(&cum) {
            println!("-1");
            return;
            // st_loop = *set.get(&cum).unwrap();
            // len_loop = count - st_loop;
            // break;
        }
        rs.push(r);
        set.insert(cum, count);
        // println!("x = {x}, r = {:?}", r);
        x *= 10;
        x %= k;
    }

    // 7 = 7
    // 77 = 7 * 11
    // 777 = 7 * 3 * 37
    // 7777 = 7 * 1111

    // 10^6以下の素数を列挙



    // 1,3,5,7,9,11
    // 111 / 3 = 37
    // k で割ったときの剰余が0になればいい。
    // 10倍して7を足すという操作は、
    // (10*x+7)%k = (10%k) * (x%k) + 7%k
    

    // k * x = 7 * 1111...111
    // (7 * 111...111) % k == 0
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
