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
    input! {
        h: usize,
        w: usize,
        a: [[usize; w]; h],
        b: [[usize; w]; h],
    }

    // 【順列全探索】2次元のルービックキューブ(5x5)。
    // (0,1,2,3,4)と並んだ列の並べ替えは、順列全探索すればいいだけ。
    // 計算量は、行と列についてそれぞれ全探索するから、O(5! x 5!)。
    // 転倒数が、バブルソートした回数となる。
    // 順列全探索に気づいたのが、コンテスト終了1分前。悲しい

    const INF: usize = 1 << 60;
    let mut ans = INF;
    let mut b_temp = a.clone();
    for y_perm in (0..h).permutations(h) {
        for x_perm in (0..w).permutations(w) {
            for y in 0..h {
                for x in 0..w {
                    b_temp[y][x] = a[y_perm[y]][x_perm[x]];
                }
            }
            if b == b_temp {
                // 転倒数が、交換回数
                let mut tento = 0;
                for i in 0..h-1 {
                    for j in i+1..h {
                        if y_perm[i] > y_perm[j] {
                            tento += 1;
                        }
                    }
                }
                for i in 0..w-1 {
                    for j in i+1..w {
                        if x_perm[i] > x_perm[j] {
                            tento += 1;
                        }
                    }
                }
                ans = min(ans, tento);
            }
        }
    }
    if ans == INF {
        println!("-1");
    }
    else {
        println!("{}", ans);
    }

}