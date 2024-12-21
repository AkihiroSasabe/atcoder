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
fn main() {
    // 2024-12-19 20:23-21:19 (56min)
    input! {
        k: usize, // 1 <= k <= 9
        n: usize, // 1 <= n <= 50
    }
    let mut vss = vec![];
    let mut ws = vec![];
    for i in 0..n {
        input!{
            vsi: Chars, // <= 9 桁
            wsi: Chars, // <= 27桁
        }
        vss.push(vsi);
        ws.push(wsi);
    }
    let mut vs: Vec<Vec<usize>> = vec![];
    for i in 0..n {
        let mut temp = vec![];
        for j in 0..vss[i].len() {
            temp.push(vss[i][j] as usize - '0' as usize);
        }
        vs.push(temp);
    }

    // 全探索するだけでいけるのでは?
    // *,*,_
    // 先頭と、末尾は分かる
    // 9 * 50 = 450文字
    // 1-9が1文字、2文字、3文字のときを考える


    let mut lens = vec![0];
    dfs(1, k, &mut lens, &vs, &ws);
}

fn dfs(d: usize, k: usize, lens: &mut Vec<usize>, vs: &Vec<Vec<usize>>, ws: &Vec<Vec<char>>) -> bool {
    if d == k + 1 {
        // judge
        let n = vs.len();
        let mut dicts: Vec<Vec<char>> = vec![vec![]; k+1];
        for i in 0..n {
            let mut sum = 0;
            for j in 0..vs[i].len() {
                sum += lens[vs[i][j]];
            }
            if sum != ws[i].len() {return false}
            let mut ind = 0;
            for j in 0..vs[i].len() {
                let len = lens[vs[i][j]];
                if dicts[vs[i][j]].len() == 0 {
                    let str0: Vec<char> = ws[i][ind..ind+len].to_vec();
                    dicts[vs[i][j]] = str0;
                }
                else {
                   // 矛盾がないか確認 
                    let str0: Vec<char> = ws[i][ind..ind+len].to_vec();
                    if dicts[vs[i][j]] != str0 {
                        return false
                    }
                }
                ind += len;
            }
        }
        // println!("lens = {:?}", lens);
        for i in 1..k+1 {
            for ch in dicts[i].clone() {
                print!("{}", ch);
            }
            println!("");
        }
        return true
    }
    for len in 1..4 {
        lens.push(len);
        let is_ok = dfs(d+1, k, lens, vs, ws);
        lens.pop();
        if is_ok {return is_ok}
    }

    return false
}