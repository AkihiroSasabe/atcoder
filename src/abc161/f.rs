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
    // 2024-04-28 20:20-21:27 (1h7min)
    input! {
        n: usize,
    }

    // ■問題設定
    // while N >= k
    //     [1] N / K (if N % k == 0)
    //     [2] N - K
    // 最終的に、N == 1 になるようなKは何個ある?
    // ただし、2 <= K <= N

    // ■考察
    // 2 <= N <= 10^12より、Kを全探索すると、少なくともO(N)でTLEする。
    // ★初手で、割り切れるKは、2 <= K <= √N と Nだけ。
    //   (∵Kが√Nを超えると、N / K が Kよりも小さくなって、終わる。)
    // ★初手で、引き算できるKは、 2 <= K <= N / 2
    //   (∵K が N/2を超えると、 N / K が Kより小さくなって、終わる。)
    // ★初手で、引き算できるKの条件を深堀りする。初手で引き算するということは、その後もずっとKで引き続けることに等しい(Nをkで引いても、一生kで割れない)ので、以下の式が成り立つ
    // N = X * K + 1 (X ∈ Z)
    // <=> N - 1 = X * K
    // よって、N-1 の約数全てが、条件を満たすKと考えることができる。
    
    // ★N-1の素因数分解
    // N = p0^x0 * p1^x1 * p2^x2 * ...
    // Nの約数は、(x0 + 1) * (x1 + 1) * (x2 + 1) * ...(xm + 1) - 1 個ある。
    // (※最後の-1は、K=1の分を除外。)
    let p_exps = prime_factorize(n-1);
    // println!("p_exps = {:?}", p_exps);

    // setには、N-1 の素因数を追加していく。
    let mut set = BTreeSet::new();
    let mut ans = 1;
    for p_exp in p_exps.iter() {
        let p = p_exp[0];
        let exp = p_exp[1];
        ans *= exp + 1;
        set.insert(p);
    }
    ans -= 1; // K = 1 の分 を除去 
    ans += 1; // K = N の分 を追加

    // 初手で割り算できるK (2 <= K <= √N) を探す。(上でカウントしたものは、setで重複して数えない)
    // println!("ans = {:?}", ans);
    for ki in 2..((n as f64).sqrt() as usize + 1) {
        // println!("ki = {:?}", ki);
        let mut ni = n;
        while ni >= ki {
            // println!("ni = {:?}", ni);
            if ni % ki == 0 {
                ni /= ki;
            }
            else {
                // 一度引き算モードに入ると、
                // あとはni<kiになるまで、ずっと引き算を続ける。
                // ni % ki != 0 にたいして、
                // (ni - ki * xxx) % ki != 0となるため。
                ni = ni % ki;
            }
        }
        let mut break_flag = false;
        if ni == 1 {
            // println!("ki = {:?}", ki);
            // set に含まれていなければよい
            for p in set.iter() {
                if ki % p == 0 {
                    break_flag = true;
                    break
                }
            }
            if !break_flag {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}

/// 素因数分解。出力: Vec<<素数, 指数>>
fn prime_factorize(mut x: usize) -> Vec<Vec<usize>> {
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
            prime_num_list.push(vec![i, exponent]);
        }
    }
    if x != 1 {
        prime_num_list.push(vec![x, 1]);
    }
    return prime_num_list
}
