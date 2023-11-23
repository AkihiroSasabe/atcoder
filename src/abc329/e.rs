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
    // 2023-11-18 21:30-22:40 (90 min)
    // 2023-11-19 11:38-12:38 (60 min)
    // total 150min = 2h30min
    input! {
        n: usize,
        m: usize,
        s: Chars,
        t: Chars
    }
    // sとtの先頭と末尾は、必ず一致していないといけない
    if !(s[0] == t[0] && s[s.len()-1] == t[t.len()-1]) {
        println!("No");
        return
    }

    // tに含まれていない文字を、sが持っていたら、No
    let mut hash = HashSet::new();
    for i in 0..m {
        hash.insert(t[i]);
    }
    for i in 0..n {
        if !hash.contains(&s[i]) {
            println!("No");
            return
        }
    }

    // 何文字分の部分連続文字列か? (Sのi文字目から始まる文字列は何文字目までが、Tの部分文字列になるかをカウントしていく)
    // counts[i] := s[i:i + counts[i]]は、tの部分連続文字列
    let mut counts = vec![0; n];
    for i in 0..n {
        // j: tの先頭インデックス
        for j in 0..t.len() {
            for k in j..t.len() {
                if i + (k - j) >= n {break}
                if s[i + (k - j)] != t[k] {
                    break
                }
                // println!("i ={i}, j = {j}, k = {k}");
                counts[i] = max(counts[i], k-j);
            }
        }
    }
    // println!("counts = {:?}", counts);

    // 連続部分文字列の"次の文字はt[0]" でなければならない。
    // ただし、連続部分文字列の最後が、T[m-1]と一致するならば、次の文字がt[0]じゃなくても良い。
    let mut next_start = 0;
    for i in 0..n {
        if i < next_start {continue}
        if counts[i] == m - 1 {
            // iは一番表面にtをスタンプした場所
            // スタンプを押した次の文字までは、調べる必要なし
            next_start = i + counts[i] + 1;
            continue
        }
        else {
            if i + counts[i] == n - 1 {continue}
            // "連続部分文字列の最後が t[m-1] であれば、次に来る文字は何でもよい"
            if s[i + counts[i] + 1] != t[0] && s[i + counts[i]] != t[m-1] {
                // println!("i = {i}, s[i + counts[i] + 1] = {}", s[i + counts[i] + 1]);
                println!("No");
                return
            }
        }
    }
    println!("Yes");
}