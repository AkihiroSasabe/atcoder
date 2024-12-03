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
    input! {
        n: usize,
        m: usize,
    }
    let mut ans = vec![];
    let mut num_ans = 0;
    dfs(0, 0, n, m, &mut ans, &mut num_ans);
    println!("{}", num_ans);
    let mut ans = vec![];
    dfs2(0, 0, n, m, &mut ans);
}

fn dfs(depth: usize, v: usize, n: usize, m: usize, ans: &mut Vec<usize>, num_ans: &mut usize) {
    if depth == n {
        *num_ans += 1;
        // for i in 0..n {
        //     print!("{} ", ans[i]);
        // }
        // println!("");
        return
    }

    if depth == 0 {
        for nv in 1..m - (n - (depth + 1)) * 10 + 1 {
            ans.push(nv);
            dfs(depth+1, nv, n, m, ans, num_ans);
            ans.pop();
        }
    }
    else {
        for nv in v+10..m - (n - (depth + 1)) * 10 + 1 {
            ans.push(nv);
            dfs(depth+1, nv, n, m, ans, num_ans);
            ans.pop();
        }
    }
}

fn dfs2(depth: usize, v: usize, n: usize, m: usize, ans: &mut Vec<usize>) {
    if depth == n {
        for i in 0..n {
            print!("{} ", ans[i]);
        }
        println!("");
        return
    }

    if depth == 0 {
        for nv in 1..m - (n - (depth + 1)) * 10 + 1 {
            ans.push(nv);
            dfs2(depth+1, nv, n, m, ans);
            ans.pop();
        }
    }
    else {
        for nv in v+10..m - (n - (depth + 1)) * 10 + 1 {
            ans.push(nv);
            dfs2(depth+1, nv, n, m, ans);
            ans.pop();
        }
    }
}