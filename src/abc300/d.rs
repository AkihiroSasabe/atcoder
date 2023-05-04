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
use std::collections::HashSet;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    input! {
        n: usize
    }
    
    let n_root = (n as f64).sqrt() as usize + 1;
    let prime_list_usize = sieve_of_eratosthenes(n_root);
    let prime_list: Vec<u128> = prime_list_usize.iter().map(|x| *x as u128).collect();
    // println!("{:?}", prime_list);
    println!("prime_list.len()={}", prime_list.len());

    let mut ans = 0;
    for i in 0..prime_list.len() {
        let a_2 = prime_list[i] * prime_list[i];
        if a_2 > n  as u128  {break}
        for j in i+1..prime_list.len() {
            let a_2_b = a_2 * prime_list[j];
            if a_2_b > n  as u128  {break}

            // (1)全探索しても間に合う。
            // なぜなら入力例2が10^12で入力の上限であり、その解が2817785個であることから、
            // a^2×b×c^2 を小さいものから順番に全探索していけば、
            // その計算量はO(2,817,785) <= O(3*10^6)となるので間に合う
            // for k in j+1..prime_list.len() {
            //     let a_2_b_c = a_2_b * prime_list[k];
            //     if a_2_b_c > n  as u128  {break}
            //     let a_2_b_c_2 = a_2_b_c * prime_list[k];
            //     if a_2_b_c_2 > n  as u128  {break}
            //     ans += 1;                
            // }

            // (2)aとbを決めたあと、最小のcを2分探索で求めて高速化することもできる
            // √(10^12) = 10^6以下の素数の数は78498個 <= 8*10^4個ある。
            if a_2_b * prime_list[j+1] * prime_list[j+1] > n as u128 {break}
            if a_2_b * prime_list[prime_list.len()-1] * prime_list[prime_list.len()-1] <= n as u128 {
                // 1 2 3 4
                ans += prime_list.len() -1 - j;
                break 
            }
            let mut ok = j+1;
            let mut ng = prime_list.len() - 1;
            while (ok as isize - ng as isize).abs() > 1 {
                let mid = (ng + ok) / 2;
                if a_2_b * prime_list[mid] * prime_list[mid] <= n  as u128 {
                    ok = mid;
                }
                else {
                    ng = mid;
                }
            }
            // println!("prime_list.len()={}, ok={}, ng={}, abc_ok={}, abc_ng={}", prime_list.len(), ok, ng, a_2_b * prime_list[ok] * prime_list[ok], a_2_b * prime_list[ng] * prime_list[ng]);
            ans += ok - j;
        }
    }
    println!("{}", ans);

}

// エラトステネスの篩(ふるい)
// n以下の素数を全て列挙する為のアルゴリズムO(n*log(log(n)))
fn sieve_of_eratosthenes(n: usize) -> Vec<usize> {
    // prime_judge[i] := iが素数なら1, そうでなければ0
    let mut prime_judge = vec![1; n + 1];
    prime_judge[0] = 0;
    prime_judge[1] = 0;

    // prime_list := n以下の素数を格納したリスト
    let mut prime_list = vec![];
    for i in 2..(n+1) {
        if prime_judge[i] == 0 {continue}
        for j in 2..((n/i)+1) {
            prime_judge[j * i] = 0;
        }
        prime_list.push(i);
    }
    return prime_list;
}
