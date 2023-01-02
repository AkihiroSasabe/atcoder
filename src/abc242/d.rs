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
    // 23-01-02 
    // 10:22 - 11:29 = 1h7m
    // 14:12 - 16:35 = 2h23m
    // sum = 3h30m
    input! {
        s: Chars,
        q: usize
    }
    let mut t = vec![];
    let mut k = vec![];
    for i in 0..q {
        input! {
            t_i: usize,
            k_i: usize,
        }
        t.push(t_i);
        k.push(k_i-1);
    }

    let mut hash: HashMap<char, Vec<char>> = HashMap::new();
    hash.insert('A', "BC".chars().collect());
    hash.insert('B', "CA".chars().collect());
    hash.insert('C', "AB".chars().collect());

    for i in 0..q {
        let ans_i = saiki(t[i], k[i], &hash, &s);
        println!("{}", ans_i);
    }
}


fn saiki(ti: usize, ki: usize, hash: &HashMap<char, Vec<char>>, s: &Vec<char>) -> char {
    if ti == 0 {
        return s[ki];
    }
    if ki == 0 {
        let mut index = 0;
        if s[0] == 'A' {
            index += 0;
        }
        else if s[0] == 'B' {
            index += 1;
        }
        else if s[0] == 'C' {
            index += 2;
        }
        index = (index + ti) % 3;
        let c = ['A', 'B', 'C'][index];
        return c
    }
    let parent = saiki(ti-1, ki/2, hash, s);
    let c = hash[&parent][ki%2];
    return c
}

// a->bc
// b->ca
// c->ab

// 1<=|s|<=10^5
// 1<=q<=10^5
// 0<=ti<=10^18
// 1<=ki<=min(10^18,sの長さ)
// s(0):=s

// 考察
// ・ti % 3で先頭の文字はわかるが。。。
// ・f(ti, ki-1) = dict[f(ti-2, (ki-1)/4)][ki-1%4]
// 文字数は常に2倍になっていく

// current_depth:  0 A
// current_depth:  1 BC
// current_depth:  2 CAAB
// current_depth:  3 ABBCBCCA
// current_depth:  4 BCCACAABCAABABBC
// current_depth:  5 CAABABBCABBCBCCAABBCBCCABCCACAAB
// current_depth:  6 ABBCBCCABCCACAABBCCACAABCAABABBCBCCACAABCAABABBCCAABABBCABBCBCCA

// 0 A
// 1 BC
// 2 CAAB
// 3 ABBCBCCA
// 4 BCCACAABCAABABBC
// 5 CAABABBCABBCBCCAABBCBCCABCCACAAB
// 6 ABBCBCCABCCACAABBCCACAABCAABABBCBCCACAABCAABABBCCAABABBCABBCBCCA
// 7 BCCACAABCAABABBCCAABABBCABBCBCCACAABABBCABBCBCCAABBCBCCABCCACAABCAABABBCABBCBCCAABBCBCCABCCACAABABBCBCCABCCACAABBCCACAABCAABABBC
// 8 CAABABBCABBCBCCAABBCBCCABCCACAABABBCBCCABCCACAABBCCACAABCAABABBCABBCBCCABCCACAABBCCACAABCAABABBCBCCACAABCAABABBCCAABABBCABBCBCCAABBCBCCABCCACAABBCCACAABCAABABBCBCCACAABCAABABBCCAABABBCABBCBCCABCCACAABCAABABBCCAABABBCABBCBCCACAABABBCABBCBCCAABBCBCCABCCACAAB
// CAABABBCABBCBCCAABBCBCCABCCACAABABBCBCCABCCACAABBCCACAABCAABABBCABBCBCCABCCACAABBCCACAABCAABABBCBCCACAABCAABABBCCAABABBCABBCBCCAABBCBCCABCCACAABBCCACAABCAABABBCBCCACAABCAABABBCCAABABBCABBCBCCABCCACAABCAABABBCCAABABBCABBCBCCACAABABBCABBCBCCAABBCBCCABCCACAAB