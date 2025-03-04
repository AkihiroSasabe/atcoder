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
    // 2025-03-02 11:55-12:44 (49min, 解説見た)
    // 2025-03-02 12:44-12:57 (13min, 思いつき)
    // Total: 62min
    input! {
        n: usize,
        q: usize,
    }
    let mut k = vec![];
    let mut a = vec![];
    let mut b = vec![];
    for i in 0..q {
        input!{
            ki: usize,
            ai: Usize1,
        }
        k.push(ki);
        a.push(ai);
        if ki != 3 {
            input! {
                bi: Usize1
            }
            b.push(bi);
        }
        else {
            b.push(0);
        }
    }

    // 例: 
    // もともとの巣:          1, 2, 3
    // 操作2(1,2)実行後の巣:  2, 1, 3
    // 操作2(2,3)実行後の巣:  3, 1, 2

    // MAP[i] := もともとiの巣は、現在どの巣にあるか? (↑でいう、上から下への操作)
    // MAP[1] = 3
    // MAP[2] = 1
    // MAP[3] = 2
    
    // INV_MAP[i] := 現在iにある巣は、もともとはどこの巣か? (↑でいう、下から上への操作)
    // INV_MAP[1] = 2
    // INV_MAP[2] = 3
    // INV_MAP[3] = 1

    // hatos[i] := 鳩iがもともといる場所
    let mut hatos = vec![];

    // map[i] := 初期の場所i の転移先
    // rev_map[i] := 場所i はもともとどこにいたか?
    let mut map = vec![];
    let mut rev_map = vec![];
    for i in 0..n {
        hatos.push(i);
        map.push(i);
        rev_map.push(i);
    }
    for i in 0..q {
        if k[i] == 1 {
            // 鳩 a を今いる巣から取り出し、巣 b へ移動する。
            let original_pos = rev_map[b[i]];
            hatos[a[i]] = original_pos;
        }
        else if k[i] == 2 {
            // 巣 a にいる鳩をすべて巣 b へ移動し、巣 b にいる鳩をすべて巣 a へ移動する。
            // これら 2 つの移動は一斉に行われる。
            let original_ai = rev_map[a[i]];
            let original_bi = rev_map[b[i]];

            rev_map[a[i]] = original_bi;
            rev_map[b[i]] = original_ai;

            let pre_ai = map[original_ai];
            let pre_bi = map[original_bi];

            map[original_ai] = pre_bi;
            map[original_bi] = pre_ai;
        }
        else if k[i] == 3 {
            // 鳩 a が今いる巣の番号を報告する
            println!("{}", map[hatos[a[i]]] + 1);
        }
    }
}