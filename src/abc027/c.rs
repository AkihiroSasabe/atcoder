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
    // 2025-01-13 19:52-20:30 (38min)
    // 2025-01-14 20:20-21:00 (40min)
    // 2025-01-14 23:14-24:36 (1h22min)
    // Total: 2h40min
    input! {
        n: usize
    }
    // println!("{n} = {:05b}", n);

    // a[i] := nを2進数で表現したビット列
    let mut a = vec![];
    for i in (0..64).rev() {
        if (1 << i) & n != 0 {
            a.push(1);
        }
        else {
            if a.len() == 0 {continue}
            a.push(0);
        }
    }
    // println!("a = {:?}", a);

    // 111=N
    // 100
    // 11
    // 
    // 状態 (N >> depth と比較した場合に、取りうる状態)
    // 2: Nより大きい
    // 1: Nと同じ
    // 0: Nより小さい
    // set: <(深さ, 状態), 勝敗>
    let mut set: BTreeMap<(usize, usize), bool> = BTreeMap::new();
    let init_depth = 0;
    let init_state = 1;
    if dfs(init_depth, init_state, &a, &mut set) {
        println!("Takahashi");
    }
    else {
        println!("Aoki");
    }
    // println!("set = {:?}", set);
}

// 終状態A:Nより、1個多いビット数で終わる。同じビット数のときには、Nより小さい
// 終状態B:Nと同じビット数で終わる
fn dfs(depth: usize, state: usize, a: &Vec<usize>, set: &mut BTreeMap<(usize, usize), bool>) -> bool {
    // この状態で渡されたときに勝てるか?
    if let Some(&res) = set.get(&(depth, state)) {
        return res
    }

    if depth == a.len() {
        // Nより大きい状態で、渡されているので、勝ち
        set.insert((depth, state), true);
        return true
    } 
    if depth == a.len() - 1 {
        if (state == 0 || state == 1) {
            // 最後まで詰まった状態で、Nより小さいまたは等しい状態で渡されると、負け
            set.insert((depth, state), false);
            return false
        }
        else {
            // Nより大きい状態で渡されれば、勝ち
            set.insert((depth, state), true);
            return true
        }
    }

    // N = 100
    // x = 11
    // [0]今の状態がNより大きい または 小さい
    if state == 0 || state == 2 {
        let res = !dfs(depth + 1, state, &a, set);
        set.insert((depth, state), res);
        return res
    }
    else {
        // 今の状態がNと等しい
        let res0;
        let res1;
        if a[depth+1] == 0 {
            res0 = dfs(depth + 1, 1, &a, set); // 等しいままで渡す
            res1 = dfs(depth + 1, 2, &a, set); // 大きい状態で渡す
        }
        else {
            res0 = dfs(depth + 1, 1, &a, set); // 等しいままで渡す
            res1 = dfs(depth + 1, 0, &a, set); // 小さい状態で渡す
        }
        if !res0 || !res1 {
            set.insert((depth, state), true);
            return true
        }
        else {
            set.insert((depth, state), false);
            return false
        }
    }
}
