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
    // 2024-05-19 19:49-20:12 (23min)
    input! {
        n: usize,
    }
    let mut a = vec![];
    let mut b = vec![];
    let mut omote = BTreeMap::new();
    let mut ura = BTreeMap::new();
    let mut set: BTreeSet<(usize, usize)> = BTreeSet::new();
    for i in 0..n {
        input!{
            ai: usize, // 表
            bi: usize, // 裏
        }
        a.push(ai);
        b.push(bi);
        omote.entry(ai).or_insert(vec![]).push(i);
        ura.entry(bi).or_insert(vec![]).push(i);
    }
    for (num, aaa) in omote.iter() {
        for i in 0..aaa.len() {
            for j in i+1..aaa.len() {
                set.insert((aaa[i], aaa[j]));
            }
        }
    }

    for (num, aaa) in ura.iter() {
        for i in 0..aaa.len() {
            for j in i+1..aaa.len() {
                set.insert((aaa[i], aaa[j]));
            }
        }
    }

    // 計算量は、状態数が、2^N で、遷移が nC2 = N*(N-1)/2 であるから、全体でO(2^N * N^2)
    let mask  = (1 << n)-1;
    let mut is_winner = HashMap::new();
    let ans = dfs(mask, &set, &mut is_winner, n);
    // println!("set = {:?}", set);
    // for (mask, res) in is_winner {
    //     println!("mask = {:09b}, res = {res}", mask);
    // }
    // 40_108_032
    if ans {
        println!("Takahashi");
    }
    else {
        println!("Aoki");
    }

}

fn dfs(mask: usize, set: &BTreeSet<(usize, usize)>, is_winner: &mut HashMap<usize, bool>, n: usize) -> bool {
    // println!("mask = {:09b}", mask);
    if is_winner.contains_key(&mask) {
        let res = *is_winner.get(&mask).unwrap();
        return res
    }

    let mut is_win = false;
    for i in 0..n {
        for j in i+1..n {
            // 両者が未使用だったら。
            if ((mask & (1 << i)) != 0) && ((mask & (1 << j)) != 0) {
                if !set.contains(&(i, j)) {continue} // カードがなければスキップ
                is_win = !dfs(mask ^ (1 << i) ^ (1 << j), set, is_winner, n);
                if is_win {
                    is_winner.insert(mask, is_win);
                    return is_win
                }
            }
        }
    }

    is_winner.insert(mask, is_win);
    return is_win
}