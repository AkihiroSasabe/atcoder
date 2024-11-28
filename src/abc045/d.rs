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
    // 2024-11-28 19:50-20:14 (24min)
    // 【寄与】Nマスをランダムに黒く塗る。HxWのテーブルで、3x3領域にx個黒マスを含んでいる領域数を答える問題。
    // 1つの黒マスは、最大9個の3x3領域に寄与できるので、その領域をNマス分について、全て数え上げればいいだけ。計算量O(9N)
    input! {
        h: usize,
        w: usize,
        n: usize,
    }
    let mut a = vec![];
    let mut b = vec![];
    for i in 0..n {
        input!{
            ai: Isize1,
            bi: Isize1,
        }
        a.push(ai);
        b.push(bi);
    }

    let mut btree = BTreeMap::new();
    let dir_x = [-2,-1,0,    -2,-1,0, -2,-1,0];
    let dir_y = [-2,-2,-2,  -1,-1,-1,  0,0,0];

    for i in 0..n {
        for j in 0..9 {
            // 左上
            let ny = a[i] + dir_y[j];
            let nx = b[i] + dir_x[j];
            if ny < 0 || h as isize <= ny || nx < 0 || w as isize <= nx {continue}

            // 右下
            let ny2 = ny + 2;
            let nx2 = nx + 2;
            if ny2 < 0 || h as isize <= ny2 || nx2 < 0 || w as isize <= nx2 {continue}


            *btree.entry((ny, nx)).or_insert(0) += 1;
        }
    }

    let mut ans = vec![0; 10];
    let num_zero = (h-2) * (w-2) - btree.len(); // 一個も黒がない、3x3領域の個数
    for (key, num) in btree { 
        // println!("key = {:?}, num = {:?}", key, num);
        ans[num] += 1;
    }
    ans[0] = num_zero;

    for ai in ans {
        println!("{}", ai);
    }

    
}