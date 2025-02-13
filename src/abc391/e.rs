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
    // 2025-02-01 21:55-22:52 (57min)
    input! {
        n: usize,
        // a: [usize; n]
        aa: Chars
    }
    let mut a = vec![];
    for ai in aa {
        a.push(ai as usize - '0' as usize);
    }

    // solve(n, a);
    solve2(n, a);

}




fn solve(n: usize, a: Vec<usize>) {

    let mut tree = vec![vec![]; n + 1];
    saiki(a.clone(), &mut tree, 0);

    // for d in 0..tree.len() {
    //     println!("tree[{d}] = {:?}", tree[d]);
    // }

    let ans = dfs(&tree, tree.len() -1, 0);
    println!("{}", ans);

    // 
    // 3^13 = 1_594_323 >= 1.5 * 10^6
    // 1 - x
}


fn dfs(tree: &Vec<Vec<usize>>, depth: usize, ind: usize) -> usize {
    // let max_depth = tree.len();
    if depth == 0 {
        return 1
    }

    let me = tree[depth][ind];

    let mut count: usize = 0; // 同じ奴の数
    for j in 0..3 {
        if me == tree[depth-1][3*ind + j] {
            count += 1;
        }
    }
    let mut cost = 1 << 63;

    if count == 3 {
        let mut cost = 0;
        let mut max_cost = 0;
        for j in 0..3 {
            let temp_cost = dfs(tree, depth - 1, 3*ind + j);
            max_cost = max(max_cost, temp_cost);
            cost += temp_cost;
        }
        cost -= max_cost;
        return cost
    }
    // else if count == 2 {
    else {
        for j in 0..3 {
            if me == tree[depth-1][3*ind + j] {
                cost = min(cost, dfs(tree, depth - 1, 3*ind + j));
            }
        }
        return cost
        }
}



fn saiki(a: Vec<usize>, tree: &mut Vec<Vec<usize>>, depth: usize) -> Vec<usize> {
    // println!("depth = {depth}, a = {:?}", a);
    let mut len = a.len();
    for i in 0..len {
        tree[depth].push(a[i]);
    }

    if len == 1 {return a}

    
    let mut na = vec![];

    for i in 0..len/3 {
        let i0 = 3*i;
        let i1 = 3*i + 1;
        let i2 = 3*i + 2;
        if a[i0] + a[i1] + a[i2] < 2 {
            na.push(0);
        }
        else {
            na.push(1);
        }
    }
    let res = saiki(na, tree, depth + 1);
    return res
}