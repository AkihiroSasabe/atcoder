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
    // 2025-01-27 20:47-21:00 (13min)
    // 2025-01-29 20:21-21:00 (39min)
    // 2025-01-30 20:00-20:57 (57min, give up)
    // 2025-01-30 21:00-21:14 (14min, 競プロフレンズの解説を見た)
    // Total: 123min
    input! {
        n: usize,
        a: [usize; n]
    }
    // for i in 0..n {
    //     println!("a[i] = {:03b}", a[i]);
    // }
    // solve(n, a);
    // solve2(n, a);

    // 1 * 2 * 3 * ... * 12
    // 12! = 479_001_600
    let mut res = vec![];
    let mut set = BTreeSet::new();
    rec(0, &a, &mut res, &mut set);
    println!("{}", set.len());
}

fn rec(depth: usize, a: &Vec<usize>, res: &mut Vec<usize>, set: &mut BTreeSet<usize>) {
    if depth == a.len() {
        let mut xor = 0;
        for &v in res.iter() {
            xor ^= v;
        }
        set.insert(xor);
        return
    }
    res.push(a[depth]);
    rec(depth + 1, a, res, set);
    res.pop();

    for i in 0..res.len() {
        res[i] += a[depth];
        rec(depth + 1, a, res, set);
        res[i] -= a[depth];
    }
}


fn solve2(n: usize, a: Vec<usize>) {
    let mut set = BTreeSet::new();
    for num_group in 1..n+1 {
        // println!("**** **** **** num_group = {:?} **** **** ****", num_group);
        let mut groups: Vec<usize> = vec![1; num_group];
        groups[0] += n - num_group; // 最初のグループに全ぶっぱ！
        dfs(n, &mut groups, 0, &mut set, &a);
        // 12個でグループ数が4の場合
        // 9,1,1,1
        // 8,2,1,1
        // 7,3,1,1 ***
        // 7,2,2,1
        // 6,4,1,1 ***
        // 6,3,2,1
        // 5,5,1,1
        
        
        // 5,0,0
        // 4,1,0
        // 3,2,0
        // 3,1,1
        // 
// 6
// 71 74 45 34 31 60
// num_group = 1 ----
// groups = [6]
// num_group = 2 ----
// groups = [5, 1]
// groups = [4, 2]
// groups = [3, 3]
// num_group = 3 ----
// groups = [4, 1, 1]
// groups = [3, 2, 1] <- [2,2,2] が足りてない
// num_group = 4 ----
// groups = [3, 1, 1, 1]
// groups = [2, 2, 1, 1]
// num_group = 5 ----
// groups = [2, 1, 1, 1, 1]
// num_group = 6 ----
// groups = [1, 1, 1, 1, 1, 1]


        // 4,1,1
        // 3,2,1
        // 2,2,2

        // 6個でグループ数が3の場合
        // 1,1,4: nC1 * n-1C1 通り
        // 1,2,3
        // 2,2,2

        // 7個でグループ数が3の場合
        // 1,1,5
        // 1,2,4
        // 1,3,3
        // 2,2,3

        // 12個でグループ数が3の場合
        // 1,1,10
        // 1,2,9 ***
        // 1,3,8
        // 1,3,7
        // 1,4,6
        // 1,5,5
        // 2,2,8 *** 
        // 2,3,7
        // 2,4,6
        // 2,5,5
        // 3,3,6
    }
    // println!("set = {:?}", set);
    println!("{}", set.len());
}

// [0,1,2,3]
// 4C2 = {{0,1}, {0,2}, {0,3}, {1,2}, {1,3}, {2,3}}
// 2C2 = {{2,3}, {1,3}, {1,2}, {0,3}, {0,2}, {0,1}}

fn dfs2(d: usize, groups: &Vec<usize>, set: &mut BTreeSet<usize>, is_remain: &mut Vec<bool>, xor: &mut usize, a: &Vec<usize>) {
    if d == groups.len() {
        set.insert(*xor);
        return
    }
    let size_d = groups[d];
    let mut remains = vec![];
    let n = is_remain.len();
    for i in 0..n {
        if is_remain[i] {
            remains.push(i);
        }
    }
    for comb in remains.iter().combinations(size_d) {
        // println!("comb = {:?}", comb);
        let mut sum = 0;
        for &&v in &comb {
            is_remain[v] = false;
            sum += a[v];
        }
        *xor ^= sum;
        dfs2(d+1, groups, set, is_remain, xor, a);
        for &&v in &comb {
            is_remain[v] = true;
        }
        // 0 ^ 0 -> 0 -> ^0 = 0
        // 1 ^ 0 -> 1 -> ^0 = 1
        // 0 ^ 1 -> 1 -> ^1 = 0
        // 1 ^ 1 -> 0 -> ^1 = 1
        *xor ^= sum;
    }
}

fn dfs(n: usize, groups: &mut Vec<usize>, depth: usize, set: &mut BTreeSet<usize>, a: &Vec<usize>) {
    // println!("groups = {:?}", groups);
    // println!("depth = {:?}", depth);
    
    if groups.len() - 1 == depth {
        // println!("---- dfs tip ----");
        // println!("groups = {:?}", groups);
        let mut is_remain = vec![true; n];
        let mut xor = 0;
        dfs2(0, groups, set, &mut is_remain, &mut xor, a);
        return
    }

    let num = groups[depth];
    dfs(n, groups, depth+1, set, a); // 配らないとき

    // 注目位置から配るとき
    for num_give in 1..num {
        groups[depth] -= num_give;
        dfs3(n, groups, depth, set, a, depth + 1, num_give);
        // for target in depth+1..groups.len() {
        //     if groups[target] + num_give > groups[target-1] {continue}
        //     groups[target] += num_give;
        //     dfs(n, groups, target, set, a);
        //     groups[target] -= num_give;
        // }

        groups[depth] += num_give;
    }
}


fn dfs3(
    n: usize, groups: &mut Vec<usize>, depth: usize, set: &mut BTreeSet<usize>, a: &Vec<usize>,
    target: usize, num_give: usize) {

    // 最後まで来たら?
    if target == groups.len() - 1 {
        // println!("---- dfs3 tip ----");
        // println!("target = {:?}", target);
        // println!("num_give = {:?}", num_give);
        // println!("groups = {:?}", groups);
        if groups[target] + num_give > groups[target-1]  {return}
        groups[target] += num_give;
        dfs(n, groups, target, set, a);
        groups[target] += num_give;
        // std::process::exit(1);
        return
    }
    // 渡す数が減ったら?
    if num_give == 0 {
        // println!("---- num_give == 0 ----");
        // println!("target = {:?}", target);
        // println!("groups = {:?}", groups);
        // dfs(n, groups, target, set, a);
        dfs(n, groups, groups.len() - 1, set, a);
        // std::process::exit(1);
        return
    }


    // 注目位置から、それより右にいるグループに配る。
    for num_give_to_target in (0..num_give+1).rev() {
        if groups[target] + num_give_to_target > groups[target-1] {continue}
        groups[target] += num_give_to_target;
        dfs3(n, groups, depth, set, a, target + 1, num_give - num_give_to_target);
        groups[target] -= num_give_to_target;
    }
}


fn solve(n: usize, a: Vec<usize>) {
    let mut set = BTreeSet::new();

    // 合併なし
    let mut all_xor = 0;
    for i in 0..n {
        all_xor ^= a[i];
    }
    set.insert(all_xor);

    // 合併あり
    for num in 2..n+1 {
        // num := 合併数
        for comb in (0..n).combinations(num) {
            // 合併した人たち
            let mut seen = vec![false; n];
            let mut xor = 0;
            for x in comb {
                xor += a[x];
                seen[x] = true;
            }


            for i in 0..n {
                if seen[i] {continue}
                xor ^= a[i];
            }
            set.insert(xor);
        }
    }

    // 8_916_100_448_256
    println!("{}", set.len());

}