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
    // 2025-07-24 21:08-21:56 (48min)
    // 2025-07-25 12:17- (DPでやるべきか?)
    // 2025-07-28 19:40-21:00 (20min)
    // 2025-07-29 12:30-12:44 (14min)
    // Total 82min

    // let mut is_debug = true;
    let mut is_debug = false;
    
    if !is_debug {
        input! {
            n: usize,
            ab: [(i128, i128); n],
        }
        solve(n, ab);
        // solve(n, ab.clone());
        // solve_brute_force(n, ab);
    }
    else {
        loop {
            let (n, ab) = make_test_case();
            let res0 = solve(n, ab.clone());
            let res1 = solve_brute_force(n, ab.clone());
            if res0 != res1 {
                println!("------------------------------");
                println!("{}", n);
                for i in 0..n {
                    println!("{} {}", ab[i].0, ab[i].1);
                }
                println!("res0 = {}, res1 = {}", res0, res1);
                break;
            }        
        }
    }
    
    
}

// 合わないテストケース
// 3
// -2 1
// -1 -2
// -1 1
// res0 = 5, res1 = 7

fn make_test_case() -> (usize, Vec<(i128, i128)>) {
    let mut rng = rand::thread_rng();
    // let n = rng.gen_range(10..=20);
    let n = rng.gen_range(1..=3);
    let mut ab = vec![];
    for _ in 0..n {
        // let a = rng.gen_range(-1_000_000_000_000_000_000_..=1_000_000_000_000_000_000_);
        // let b = rng.gen_range(-1_000_000_000_000_000_000_..=1_000_000_000_000_000_000_);
        let a = rng.gen_range(-2..=2);
        let b = rng.gen_range(-2..=2);
        ab.push((a, b));
    }
    return (n, ab);
}

fn solve_brute_force(n: usize, ab: Vec<(i128, i128)>) -> i128 {
    let mut ans = 0;
    for mask in 1..1<<n {

        let mut lists = vec![];
        for i in 0..n {
            if mask & (1 << i) != 0 {
                lists.push(ab[i]);
            }
        }
        // println!("lists = {:?}", lists);
        let mut is_ok = true;
        for i in 0..lists.len() {
            for j in i+1..lists.len() {
                let (ai, bi) = lists[i];
                let (aj, bj) = lists[j];
                if ai * aj + bi * bj == 0 {
                    is_ok = false;
                    break;
                }
            }
            if !is_ok {break;}
        }

        if is_ok {
            ans += 1;
            ans %= 1_000_000_007;
        }
    }
    println!("ans = {:?}", ans);
    return ans
}


fn solve(n: usize, ab: Vec<(i128, i128)>) -> i128 {
    let mut a = vec![]; // 美味しさ
    let mut b = vec![]; // 香り
    for i in 0..n {
        let ai = ab[i].0; // 美味しさ
        let bi = ab[i].1; // 香り
        if ai == 0 && bi == 0 {
            a.push(0);
            b.push(0);
            continue;
        }
        else if ai == 0 {
            a.push(0);
            b.push(1);
            continue;
        }
        else if bi == 0 {
            a.push(1);
            b.push(0);
            continue;
        }
        
        let gcd_i = gcd(ai.abs() as i128, bi.abs() as i128);

        // bi が絶対プラスになるようする。
        if bi < 0 {
            a.push(-ai / gcd_i);
            b.push(-bi / gcd_i);
        }
        else {
            a.push(ai / gcd_i);
            b.push(bi / gcd_i);
        }
    }
    let modulus = 1_000_000_007;

    // 平行なイワシ同士は、同じグループに入れてよい。
    // 直行するグループ同士をカウントすればよい。
    // a>0にする。N個に分割可能。

    let mut btree: BTreeMap<(i128, i128), i128> = BTreeMap::new();
    for i in 0..n {
        *btree.entry((a[i], b[i])).or_insert(0) += 1;
    }
    // println!("btree = {:?}", btree);

    let mut pow = vec![1; n+10];
    for i in 0..n+9 {
        pow[i+1] = pow[i] * 2;
        pow[i+1] %= modulus;
    }

    let mut ans: i128 = 1;
    let mut seen: BTreeSet<(i128, i128)> = BTreeSet::new();
    let mut num_free = 0;
    let mut num_zero_zero = 0;
    for (&(ai, bi), &num) in btree.iter() {
        if ai == 0 && bi == 0 {
            num_zero_zero += num;
            continue;
        }
        // println!("(ai, bi), num = {:?}, {}", (ai, bi), num);
        let opp = (bi, -ai);
        // println!("opp = {:?}", opp);
        if seen.contains(&(ai, bi)) {
            continue;
        }
        seen.insert((ai, bi));
        seen.insert(opp);
        if let Some(&num_opp) = btree.get(&opp) {
            // println!("opp, num_opp = {:?}, {} ----", opp, num_opp);
            let cont1 = pow[num as usize];
            let cont2 = pow[num_opp as usize];
            let diff = (cont1 + cont2 + modulus - 1) % modulus;
            // println!("diff = {:?}", diff);
            ans = ans * diff;
            ans %= modulus;
        }
        else {
            num_free += num;
        }
    }
    ans = ans * pow[num_free as usize] % modulus;
    ans = (ans + num_zero_zero) % modulus; // (0,0)のイワシ
    ans = (modulus + ans - 1) % modulus; // 全部0個のパターンを除去。 (イワシは、1匹以上は必要なので。)
    println!("{}", ans);

    // 包除原理 か dp を疑うべき。
    return ans


}


// ユークリッドの互除法で最大公約数を求める (Euclidean Algorithm)
// ユークリッドの互除法とは、x < y のとき、gcd(x, y)=gcd(x, y % x)
fn gcd(x: i128, y:i128) -> i128 {
    if y == 0 {
        // 任意の整数xは0の約数と言える(∵0 % x == 0)ので、0とxの最大公約数はx
        return x;
    }
    else {
        return gcd(y, x % y);
    }
}
