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
        n:usize,
        mut a:[usize; n],
    }

    // ペアで互いに素か判定したい。
    // {3,4,5}
    // gcd(3,4) = 1
    // gcd(3,5) = 1
    // gcd(4,5) = 1
    // 動的に判定すればいけそうな気がするが。
    // あるいは エラトステネスの篩を修正すればやれそう。

    // {6,10,15}
    // gcd(6,10)=2
    // gcd(6,15)=3
    // set-wise の方が簡単に、co-prime になれる。


    a.sort();
    a.reverse();

    // まずa がset-wiseか調べるのが楽そう。
    // pair-wise かどうかは...

    // n　以下の素数を全列挙
    // その素数の登場回数を動的に全部カウントしていけば良さそう。

    // let ps = sieve_of_eratosthenes(*a.iter().max().unwrap());
    // println!("ps = {:?}", ps);
    // println!("ps.len() ={}", ps.len());
    // ps.len() =78_498,  amax = 1000000 のとき

    // let mut set = HashSet::new();

    // let mut is_pair_wise_coprime = true;
    // for i in 0..n {
    //     for &p in ps.iter() {
    //         if a[i] < p {continue;}
    //         if a[i] % p == 0 {
    //             if set.contains(&p) {
    //                 is_pair_wise_coprime = false;
    //                 break
    //             }
    //             set.insert(p);
    //         }
    //     }
    //     if !is_pair_wise_coprime {
    //         break
    //     }
    // }

    // ペアワイズの判定が難しい
    // エラトステネスの篩を、修正して解く必要がある。
    let is_pair_wise_coprime = judge_is_pairwise_coprime(n, a.clone());
    if is_pair_wise_coprime {
        println!("pairwise coprime");
        return;
    }

    // セットワイズの判定は簡単
    let mut gcd0 = a[0];
    for i in 1..n {
        gcd0 = gcd(gcd0, a[i]);
        if gcd0 == 1 {
            println!("setwise coprime");
            return;
        }
    }
    println!("not coprime");
}


// ユークリッドの互除法で最大公約数を求める (Euclidean Algorithm)
// ユークリッドの互除法とは、x < y のとき、gcd(x, y)=gcd(x, y % x)
fn gcd(x: usize, y:usize) -> usize {
    if y == 0 {
        // 任意の整数xは0の約数と言える(∵0 % x == 0)ので、0とxの最大公約数はx
        return x;
    }
    else {
        return gcd(y, x % y);
    }
}

// fn judge(n: usize, a: Vec<usize>) -> bool {

//     // a.dedup();
//     let a_max = *a.iter().max().unwrap();
//     let mut counter = vec![0; a_max + 1];

//     for i in 0..n {
//         counter[a[i]] += 1;
//     }

//     // is_prime_list[i] := iが素数なら true , そうでなければ false
//     let mut is_prime_list = vec![true; a_max + 1]; // この初期化でO(N)かかる!
//     is_prime_list[0] = false; // 0は素数ではない
//     is_prime_list[1] = false; // 1は素数ではない

//     // prime_list := n以下の素数を格納したリスト
//     let mut prime_list = vec![];
    
//     // ここの計算で、O(N/2 + N/3 + N/5 + N/7 + N/11 + ...)  = O(N (1/2 + 1/3 + 1/5 + 1/7 + 1/11 + ... )) = O(Nlog(logN))かかる。
//     // ※素数の逆数和は、log(logN)と漸近していくため。自然数の逆数和は、logNに漸近する。
//     for i in 2..(a_max+1) {
//         if !is_prime_list[i] {continue}
//         for j in 2..((n/i)+1) {
//             // i の倍数が素数ではないことを記録
//             is_prime_list[j * i] = false;
            
//         }
//         prime_list.push(i);
//     }
//     return prime_list;
// }


// エラトステネスの篩(ふるい)
// n以下の素数を全て列挙する為のアルゴリズムO(n*log(log(n)))
fn judge_is_pairwise_coprime(n: usize, a: Vec<usize>) -> bool {

    let a_max = *a.iter().max().unwrap();
    let mut counter = vec![0; a_max + 1];

    for i in 0..n {
        counter[a[i]] += 1;
    }

    // is_prime_list[i] := iが素数なら true , そうでなければ false
    let mut is_prime_list = vec![true; a_max + 1]; // この初期化でO(N)かかる!
    is_prime_list[0] = false; // 0は素数ではない
    is_prime_list[1] = false; // 1は素数ではない

    // prime_list := n以下の素数を格納したリスト
    let mut prime_list = vec![];
    
    // ここの計算で、O(N/2 + N/3 + N/5 + N/7 + N/11 + ...)  = O(N (1/2 + 1/3 + 1/5 + 1/7 + 1/11 + ... )) = O(Nlog(logN))かかる。
    // ※素数の逆数和は、log(logN)と漸近していくため。自然数の逆数和は、logNに漸近する。
    for i in 2..(a_max+1) {
        if !is_prime_list[i] {continue}
        let mut count = 0;
        count += counter[i];
        for j in 2..((a_max/i)+1) {
            // i の倍数が素数ではないことを記録
            is_prime_list[j * i] = false;
            count += counter[j * i];
        }
        // println!("(p, count) = {:?}", (i, count));
        prime_list.push(i);
        if count >= 2 {return false}
    }
    return true
}