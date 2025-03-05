#![allow(dead_code, unused_imports)]
use proconio::{input, marker::{Usize1, Isize1}};
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
use rand::Rng;
use num::{BigUint, ToPrimitive};
use num_bigint::ToBigUint;
fn main() {
    // ゼータ変換について、知らなくても解ける。
    // 競プロフレンズの解説: https://x.com/kyopro_friends/status/1890760581977657463
    // ゼータ変換の解説(赤コーダーの人): https://hackmd.io/@tatyam-prime/H1EhuQAt1x
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }
    // solve(n, k, a); // 本番中に解いた解法 TLEする
    // solve_by_yakusu_zenrekkyo_tle(n, k, a); // 約数全列挙で解いた解法 TLEする
    solve_by_zeta_transform(n, k, a); // 解説通りの方法 ゼータ変換というらしい。
}

fn solve_by_zeta_transform(n: usize, k: usize, a: Vec<usize>) {
    // 2025-02-22 12:32-12:54 (22min)
    // 時間計算量 O(N+MlogM) の解法
    // 
    // counts[x] := 数列aの中にあるxの個数 O(N)で取得
    // mul_counts[d] := 数列aの中にあるdの倍数の個数 O(MlogM)で取得. MはAの最大値
    // mul_counts[d] = Σ[x ∈ S, S={x % d == 0}] counts[x] の関係にある (ゼータ変換は、N次の累積和)
    // max_gcd[x] := 数列aの中にK個以上ある、xの最大公約数の最大値 O(MlogM). MはAの最大値
    // max_gcd[x] = max(d) [d ∈ S, S={x % d == 0 かつ mul_counts[d]>=k}] 

    // let max_a = *a.iter().max().unwrap();
    let max_a = 1_000_001;

    // counts[x] := 数列aの中にあるxの個数 O(N)で取得
    let mut counts = vec![0; max_a +  1];
    for i in 0..n {
        counts[a[i]] += 1;
    }

    // mul_counts[d] := 数列aの中にあるdの倍数の個数 O(MlogM)で取得. MはAの最大値
    let mut mul_counts = vec![0; max_a +  1];
    for d in 1..max_a+1 {
        let mut r = 1;
        let mut x;
        while {
            x = r * d;
            x <= max_a
        }
        {
            mul_counts[d] += counts[x];
            r += 1;
        }
    }

    // max_gcd[x] := 数列aの中にK個以上ある、xの最大公約数の最大値 O(MlogM). MはAの最大値
    let mut max_gcd = vec![0; max_a + 1];
    
    for d in 1..max_a+1 {
        let mut r = 1;
        let mut x;
        while {
            x = r * d;
            x <= max_a
        } {
            if mul_counts[d] >= k {
                max_gcd[x] = max(max_gcd[x], d);
            }
            r += 1;
        }
    }

    for i in 0..n {
        let ans = max_gcd[a[i]];
        println!("{}", ans);
    }


}

fn solve_by_yakusu_zenrekkyo_tle(n: usize, k: usize, a: Vec<usize>) {
    // - 約数列挙
    // - エラトステネスの篩
    // - 素因数分解

    // 1<= k <= n <= 1.2*10^6
    // 1<= a[i] <= 10^6
    // root(a[i]) <= 10^3
    // 計算量は、 n * root(a[i]) = 1.2*10^9 でやや厳しいか。。。
    let mut counts = vec![0; 1_000_001];

    let mut devisors_list = vec![];
    for i in 0..n {
        let devisors = enumerate_devisors(a[i]);
        for &devisor in devisors.iter() {
            counts[devisor as usize] += 1;
        }
        // println!("devisors = {:?}", devisors);
        devisors_list.push(devisors);
    }
    for i in 0..n {
        for &devisor in devisors_list[i].iter().rev() {
            if counts[devisor as usize] >= k {
                println!("{}", devisor);
                break
            }
        }
    }

}

/// 自然数Nの約数をO(√N)で昇順に全列挙する関数。
/// 約数の個数は意外と少ない https://github.com/E869120/kyopro_educational_90/blob/main/editorial/085-01.jpg
/// N <= 10^6 なら、N=720,720 が最大で、240個
/// N <= 10^9 なら、N=735,134,400 が最大で、1,344個
/// N <= 10^12 なら、N=963,761,198,400 が最大で、6,720個
/// N <= 10^18 なら、N=897,612,484,786,617,600 が最大で、103,680個
fn enumerate_devisors<T>(n: T) -> Vec<T> 
    where T: 
        Default + 
        Copy + 
        std::fmt::Display + 
        std::cmp::PartialEq + 
        std::ops::Not<Output = T> + 
        std::ops::Add<Output = T> +
        std::ops::Div<Output = T> +
        std::ops::Rem<Output = T> + 
        std::ops::Mul<Output = T> +
        std::cmp::PartialOrd

{
    // 自然数Nの約数を列挙する関数O(√N)
    let mut devisors = vec![];
    let mut devisors_over_root_n = vec![]; // √N より大きい約数を一時的に格納するリスト

    let zero: T = Default::default();
    let not_zero: T = !zero;
    let one: T = not_zero / not_zero;

    let mut devisor = one;
    while (devisor * devisor) <= n {
        if n % devisor == zero {
            devisors.push(devisor);
            if n / devisor != devisor {
                devisors_over_root_n.push(n / devisor);                
            }
        }
        devisor = devisor + one;
    }
    // println!("devisors_over_root_n = {:?}", devisors_over_root_n);

    for &devisor_over_root_n in devisors_over_root_n.iter().rev() {
        devisors.push(devisor_over_root_n);
    }
    return devisors
}


fn solve(n: usize, k: usize, a: Vec<usize>) {
    let mut ps = vec![];
    let mut set = BTreeMap::new();
    for i in 0..n {
        let prime_num_list = prime_factorize(a[i]);
        // println!("prime_num_list = {:?}", prime_num_list);
        dfs(0, 1, &mut set, &prime_num_list);

        // println!("set = {:?}", set);
        ps.push(prime_num_list);
    }

    // println!("set = {:?}", set);
    for i in 0..n {
        let mut ans = 1;
        let mut set2 = BTreeMap::new();
        dfs(0, 1, &mut set2, &ps[i]);
        // println!("set2 = {:?}", set2);
        for (v, _) in set2 {
            if let Some(nnn) = set.get(&v) {
                if *nnn >= k {
                    ans = max(ans, v);
                }
            }
        }
        println!("{}", ans);
    }
    // 独立に...
}

fn dfs(depth: usize, value: usize, set: &mut BTreeMap<usize, usize>, prime_num_list: &Vec<(usize, usize)>) {
    // println!("depth = {:?}", depth);
    // println!("value = {:?}", value);
    if prime_num_list.len() == depth{
        *set.entry(value).or_insert(0) += 1;
        return
    }
    let mut pp = 1;
    let mut val = value;
    for ei in 0..prime_num_list[depth].1+1 {
        val = value * pp;
        dfs(depth +1, val, set, prime_num_list);
        pp = pp * prime_num_list[depth].0;

    }
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