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
    // 2024-12-21 11:20-11:30 (10min)
    // 2024-12-21 13:04-15:15 (2h11min)
    // 2h21min
    solve_mine(); // 自力
    // solve_big_uint(); // BigUintを使って解いた。圧倒的に楽。
}

fn solve_big_uint() {
    // tayu0110 や ngtkana が、 BigUint なる型を使っていた。
    // 自前で、オーバーフローするような大きな数字を扱うとき、自前でベクトルに変換する必要がないので、便利かもしれない。
    // https://atcoder.jp/contests/abc030/submissions/50930889
    // https://atcoder.jp/contests/abc030/submissions/14703428
    input! {
        n: usize,
        mut a: Usize1, // 調べる対象
        mut k: BigUint, // ステップ数
        b: [Usize1; n],
    }
    // println!("k = {}", k);
    let mut v = a;
    let mut counter = vec![n; n]; // 頂点vには、counter[v] 回の移動でいける。
    let mut count = 0;
    loop {
        counter[v] = count;
        v = b[v];
        count += 1;
        if counter[v] != n {break}
    }
    let cycle = count - counter[v];
    if k > cycle.into() {
        k = (k - counter[v]) % cycle + counter[v];
    }
    for _ in 0..(k.to_usize().unwrap()) {
        a = b[a];
    }
    println!("{}", a+1);

    // usize -> BigUint 型への変換も出来る。
    // let aaa: BigUint = 100_usize.to_biguint().unwrap();
    // println!("aaa = {:?}", aaa);
}

fn solve_mine() {
    input! {
        n: usize,
        mut a: Usize1, // 調べる対象
        kc: Chars, // ステップ数
        b: [Usize1; n],
    }
    // 1 <= k <= 100_000_000_000_000_000_000
    // 循環するので、modを取るだけ。

    let mut ku = vec![];
    for ch in kc {
        ku.push(ch as usize - '0' as usize);
    }
    // println!("ku = {:?}", ku);

    // 周期を取得
    let mut v = a;
    // let mut btree = BTreeMap::new();
    let mut count = 0;
    let mut cycle = 0;
    let mut fin = a;
    let mut btree = vec![n; n];
    let mut loops = vec![];
    loop {
        loops.push(v);
        // btree.insert(v, count);
        btree[v] = count;
        v = b[v];
        // println!("a = {}, v = {:?}", a, v);
        if btree[v] != n {
            loops.push(v);
            fin = v;
            cycle = 1 + count - btree[v];
            break
        }
        count += 1;
    }

    let cycle_vec = get_vector_from_number(cycle);

    let k;
    if is_v0_larger_than_v1(&cycle_vec, &ku) {
        k = get_number_from_vec(&ku);
        for _ in 0..k {
            a = b[a];
        }
        println!("{}", a + 1);
    }
    else {
        // ku を cycle で割れ！
        let st = btree[fin];
        let st = get_vector_from_number(st);
        let ku = get_subtraction_from_vector(ku, st);
        k = get_remainder_from_vector(ku, cycle);
        for _ in 0..k {
            fin = b[fin];
        }
        println!("{}", fin + 1);
    }
}

fn get_vector_from_number(mut num: usize) -> Vec<usize> {
    // 桁毎に、ベクトルで表した数を得る関数
    // let vec = get_vector_from_number(100);
    // なら、vec = vec![1, 0, 0] と等しい
    let mut vec: Vec<usize> = vec![];
    while num != 0 {
        let r = num % 10;
        vec.push(r);
        num /= 10;
    }
    vec.reverse();
    return vec
}

fn get_number_from_vec(vec: &Vec<usize>) -> usize {
    // 桁毎にベクトルで表した数字を、ただの数字に変換する
    // let num = get_number_from_vec(vec![1, 0, 0]);
    // num は 100

    let mut num = 0;
    for i in 0..vec.len() {
        num *= 10;
        num += vec[i];
    }
    return num
}

fn get_subtraction_from_vector(mut vec: Vec<usize>, mut vec2: Vec<usize>) -> Vec<usize> {
    // 桁毎に、ベクトルで表した数の引き算
    let mut len_diff = vec.len() - vec2.len();

    vec.reverse();
    vec2.reverse();

    let mut debt = 0;
    for i in 0..vec.len() {
        
        let v2 = if vec2.len() <= i {0} else {vec2[i]};
        if vec[i] >= v2 + debt {
            vec[i] = vec[i] - v2 - debt;
            debt = 0;
        }
        else {
            vec[i] = 10 + vec[i] - v2 - debt;
            debt = 1;
        }
    }
    vec.reverse();
    return vec
}

fn get_remainder_from_vector(vec: Vec<usize>, d: usize) -> usize {
    // 各桁の値をベクトルで格納して表現された数字の剰余を得る関数
    // 例えば、100 は、vec![1, 0, 0];
    // これを3で割った余りは、
    // 1
    
    let mut remainder = 0;
    for i in 0..vec.len() {
        remainder *= 10;
        remainder += vec[i];
        remainder %= d;
    }
    return  remainder
}

fn is_v0_larger_than_v1(v0: &Vec<usize>, v1: &Vec<usize>) -> bool {
    // 桁毎に、ベクトルで表した数を比較する
    // 例えば、
    // let v0 = vec![1, 0, 0];
    // let v1 = vec![9, 9];
    // is_v0_larger_than_v1(&v0, &v1);
    // ならTrueを返す
    
    if v0.len() != v1.len() {
        return v0.len() > v1.len()
    }
    for i in 0..v0.len() {
        if v0[i] == v1[i] {continue}
        return v0[i] > v1[i]
    }
    // 完全に等しいとき
    return false
}
