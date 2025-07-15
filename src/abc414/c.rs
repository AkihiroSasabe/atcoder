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
use num::{pow, BigUint, Integer, ToPrimitive};
use num_bigint::ToBigUint;
fn main() {
    input! {
        a: usize,
        n: usize,
    }

    

    let max_keta = get_keta(n);
    // println!("max_keta = {:?}", max_keta);

    let mut pow10 = vec![1; 15];
    for i in 1..15 {
        pow10[i] = pow10[i - 1] * 10;
    }

    let mut ans = 0;
    for keta in 1..max_keta+1 {
        // println!("keta = {:?}----", keta);
        let cont = get_kaibun_num(keta, n, a, &pow10);
        ans += cont;
    }
    println!("{}", ans);
    

}


fn get_kaibun_num(keta: usize, n: usize, a: usize, pow10: &Vec<usize>) -> usize {

    let mut ans = 0;
    if keta == 1 {
        for p0 in 1..10 {
            if is_kaibun(p0, a) {
                if n >= p0 {
                    ans += p0;
                }
            }
        }
    }
    else if keta % 2 == 0 {
        // 偶数回文
        for p0 in 1..10 {
            let mut num = p0 * (pow10[0] + pow10[keta-1]);;
            let mut cont = 0;
            dfs(1, keta, pow10, num, n, a, &mut cont);
            ans += cont;
        }
    }    
    else {
        // 奇数回文
        for p0 in 1..10 {
            let mut num = p0 * (pow10[0] + pow10[keta-1]);;
            let mut cont = 0;
            dfs(1, keta, pow10, num, n, a, &mut cont);
            ans += cont;
        }
    }
    return ans
}

// 8
// 999999999999
// 1000000000000

fn dfs(d: usize, keta: usize, pow10: &Vec<usize>, num: usize, n: usize, a: usize, ans: &mut usize) {
    
    if (keta % 2 == 0 && d >= keta / 2) || (keta % 2 == 1 && d >= keta / 2 + 1) {
        // println!("num = {:?}", num);
        if num > n {
            return;
        }

        // a新方で回分か?
        if is_kaibun(num, a) {
            *ans += num;
        }
        return;
    }
    
    for i in 0..10 {
        let n_num;
        if keta % 2 == 1 && d == keta / 2 {
            n_num = num + i * (pow10[d]);
        }
        else {
            n_num = num + i * (pow10[d] + pow10[keta - 1 - d]);
        }
        dfs(d+1, keta, &pow10, n_num, n, a, ans);
    }
}

fn is_kaibun(mut num: usize, a: usize) -> bool {
    if num == 0 {return false}

    let mut bun = vec![];
    while num > 0 {
        let r = num % a;
        num /= a;
        bun.push(r);
    }
    // println!("bun = {:?}", bun);

    for i in 0..bun.len() / 2 {
        if bun[i] != bun[bun.len() - 1 - i] {
            return false;
        }
    }
    return true;
}

// fn get_keta(mut x: usize) -> usize {
//     // 10進数の桁数を求める

//     if x == 0 {return 1}
//     let mut keta = 0;
//     while x > 0 {
//         x /= 10;
//         keta += 1;
//     }
//     return keta
// }

fn get_keta(mut x: usize) -> usize {
    let mut keta = 1;
    while x / 10 != 0 {
        x /= 10;
        keta += 1;
    }
    return keta
}