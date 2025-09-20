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
    // 2025-09-19 20:08-20:56 (48min)
    // 2025-09-20 11:16-12:16 (1h, 降参)
    // 2025-09-20 12:16-12:27 (11min)
    // Total: 1h59min
    input! {
        n: usize,
        mut s: [isize; n]
    }

    // 愚直解法
    // let s_usize = s.clone().iter().map(|&x| x as usize).collect::<Vec<usize>>();
    // let ans = solve_bf(n, s_usize);

    // s.sort();
    // println!("s = {:?}", s);

    // ◆畳み込み (2025-09-20 ABC292-G問題)
    // ACL の畳み込み https://atcoder.github.io/ac-library/production/document_ja/convolution.html
    // ACL の convolution 練習問題: https://atcoder.jp/contests/practice2/tasks/practice2_f
    // 畳み込みとは、 
    // 　数列 a := [a[0], a[1], ⋯ , a[N−1]] と
    // 　数列 b := [b[0], b[1], ⋯ , b[M−1]] から、
    // 長さ N + M − 1 の
    // 　数列 c := [c[0], c[1], ⋯ , c[N + M − 2]]
    // 　c[i] = a[0]*b[i] + a[1]*b[i-1] + ... + a[i]*b[0] 
    //        = Σ [j= 0..=i] (a[j] * b[i-j])
    // を求める操作である。
    // つまり、
    // c = [a[0]*b[0], a[0]*b[1]+a[1]*b[0], a[0]*b[2]+a[1]*b[1]+a[2]*b[0], ..., a[N-1]*b[M-1]]
    // である。
    // 使うときは、典型090.rsの以下6つの関数をコピペすること。
    // mod_inv, my_pow, make_root, make_invroot, ntt, convolution_by_ntt

    // NTTの畳み込みで解く
    // B-A = C-B <=> A + C = 2B
    // よって、
    // x^A * x^C = x^(A+C) = x^(2B)
    // となるから、
    // (a[0] * x^0 + a[1] * x^1 + ... + ) * (a[0] * x^0 + a[1] * x^1 + ... + ) 
    // = (a[0]*a[0] * x^0 + (a[0]*a[1] + a[1]*a[0])*x^2 + ... (a[0]*a[3] + a[1]*a[2] + a[2]*a[1] + a[3]*a[0])*x^3 + ...)
    // となるような、右辺の係数を畳み込みで求めればいい。
    // a[x] は x が s に含まれるかどうか (0 or 1)。
    // これをs2として求めていく。

    let modulus: isize = 998244353;
    let root = make_root(modulus);
    let invroot: Vec<isize> = make_invroot(&root, modulus);

    // s2[x] := s に x が何個あるか? (重複は無いので、0個か1個)
    let mut s2 = vec![0_isize; *s.iter().max().unwrap() as usize +1];
    for i in 0..n {
        s2[s[i] as usize] += 1;
    }
    // println!("s2 = {:?}", s2);

    let mut a = s2.clone();
    let mut b = s2.clone();
    let c = convolution_by_ntt(&mut a, &mut b, &root, &invroot, modulus);
    // println!("c = {:?}", c);

    let mut ans = 0;
    for i in 0..n {
        let bi = s[i] as usize;
        ans += c[2*bi] - 1;
    }
    ans /= 2;
    // println!("ans = {:?}", ans/2);
    println!("{}", ans);

}


fn solve_bf(n: usize, mut s: Vec<usize>) -> usize {
    s.sort();
    println!("s = {:?}", s);
    let s_max = *s.iter().max().unwrap();
    let s_min = *s.iter().min().unwrap();
    let mut is_exists = vec![false; s_max+1];
    for i in 0..n {
        is_exists[s[i]] = true;
    }

    let mut ans = 0;
    for i in 0..n {
        let b = s[i];
        for diff in 1..=(s_max-s_min) {
            if b < diff {
                break;
            }
            if b + diff > s_max {
                break;
            }
            let a = b - diff;
            let c = b + diff;

            if is_exists[a] && is_exists[c] {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
    return ans
}




// mod を法とする x の逆元を計算する．
fn mod_inv(x: isize, modulus: isize) -> isize {
    return my_pow(x, modulus - 2, modulus);
}

// 繰り返し二乗法で x ^ n を mod で割った余りを求める．
fn my_pow(x: isize, n: isize, modulus: isize) -> isize{
    let mut ret = 0;
    if n == 0 {
        ret = 1;
    }
    else if n % 2 == 1 {
        ret = (x * my_pow((x * x) % modulus, n / 2, modulus)) % modulus;
    }
    else {
        ret = my_pow((x * x) % modulus, n / 2, modulus);
    }
    return ret;
}

// NTT に必要となる r の累乗数を求める．
fn make_root(modulus: isize) -> Vec<isize> {
    let mut ret = vec![];
    let mut r = my_pow(3, 119, modulus);
    for i in 0..23 {
        ret.push(r);
        r = (r * r) % modulus;
    }
    ret.reverse();
    return ret;
}

// NTT に必要となる r の累乗数の逆元を求める．
fn make_invroot(root: &Vec<isize>, modulus: isize) -> Vec<isize> {
    let mut ret = vec![];
    for i in 0..root.len() {
        ret.push(mod_inv(root[i], modulus));
    }
    return ret
}


// 数論変換 (Number Theoretic Transform: NTT)
fn ntt(a: &Vec<isize>, depth: isize, root: &Vec<isize>, modulus: isize) -> Vec<isize>{
    // inv = 1 ならば普通の NTT，
    // inv = -1 ならば INTT になるようにする（今回は，呼び出す root が逆元かそうでないかによって調整する）．
    let n = a.len();
    // println!("depth == {}", depth);
    // a のサイズが 1 であるときは，それがそのまま DFT である．
    if n == 1{
        return a.clone()
    }
    else{
        let mut ret = vec![];
        let mut even = vec![];
        let mut odd = vec![];
        for i in 0..n {
            if i % 2 == 0 {
                even.push(a[i]);
            }
            else {
                odd.push(a[i]);
            }
        }

        // even と odd の DFT を，再帰的に求める．
        let d_even = ntt(&even, depth - 1, root, modulus);
        let d_odd = ntt(&odd, depth - 1, root, modulus);

        let index = if depth >= 0 {
            depth as usize
        }
        else {
            root.len() - depth as usize
        };
        let r = root[index];
        
        let mut now = 1;
        for i in 0..n {
            ret.push((d_even[i % (n / 2)] + (now * d_odd[i % (n / 2)]) % modulus) % modulus);
            // ret.push((d_even[i % (n / 2)] + (now * d_odd[i % (n / 2)]) % modulus + 2*modulus) % modulus); // 負だった場合に、正にする <- この処理は要らない (Rustの負の数の余りは、- |r| % m となってくれるので。)
            now = (now * r) % modulus;
        }
        return ret;
    }
}


fn convolution_by_ntt(a: &mut Vec<isize>, b: &mut Vec<isize>, root: &Vec<isize>, invroot: &Vec<isize>, modulus: isize) -> Vec<isize> {
    // 配列 a, b は，それぞれ A(x) と B(x) の係数を次数の小さい順に並べたもの．
    let len_a = a.len();
    let len_b = b.len();
    let len_c = len_a + len_b - 1; // len_c は A(x) * B(x) の次数
    let mut n = 1;
    // len_c より大きい最小の 2 べきの数を求める
    while n <= len_c {
        n *= 2;
    }

    // 配列の長さが n になるまで，配列の末尾に 0 を追加する
    while a.len() < n {
        a.push(0);
    }
    while b.len() < n {
        b.push(0);
    }

    let mut log_2n = 1;
    while (1 << log_2n) < n {
        log_2n += 1;
    }

    // A(x) の NTT DA(t), b(x) の NTT DB(t) を求める．
    // 配列 da, db は，それぞれ DA(t), DB(t) の係数を次数の小さい順に並べたもの．
    let da = ntt(a, log_2n as isize - 1, root, modulus);
    let db = ntt(b, log_2n as isize - 1, root, modulus);

    // C(x) の NTT DC(t). これの k 次の係数は， DA(t) と DB(t) の k 次の係数を掛け合わせれば求まる．
    let mut dc = vec![0; n];
    for i in 0..n {
        dc[i] = (da[i] * db[i]) % modulus;
        // dc[i] = (dc[i] + modulus) % modulus; // 負だった場合に、正にする <- この処理は要らない (Rustの負の数の余りは、- |r| % m となってくれるので。)
    }

    // C(x) は DC(t) を INTT すれば求まる．このようにしてできた配列 c は，C(x) の係数を次数の小さい順に並べたものとなっている．
    let c = ntt(&dc, log_2n as isize - 1, invroot, modulus);
    // INTT の後は最後に n で割ることを忘れずに．(ここは、nで割るとき、ループ毎に毎回mod_inv計算するのは勿体ない)
    let mut ret: Vec<isize> = vec![];
    let inverse_n = mod_inv(n as isize, modulus);
    // for i in 0..n {
    for i in 0..len_c {
        ret.push((c[i] * inverse_n) % modulus);
        // ret.push(((c[i] * inverse_n) % modulus + modulus) % modulus); // 負だった場合に、正にする <- この処理は要らない (Rustの負の数の余りは、- |r| % m となってくれるので。)
    }

    return ret
}
