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
    input! {
        n: usize,
    }

    let mut ans = vec![];
    for i in 0..n {
        input! {
            a_i: isize,
            b_i: isize,
        }
        ans.push(vec![a_i, b_i, -1 * (i as isize + 1)]);
    }

    ans.sort_by(
        |x1, x2| {
            // 条件1: 分数の大きさで比較
            let fraction_cmp = (x1[0]*(x2[0] + x2[1])).cmp(&(x2[0] * (x1[0] + x1[1])));
            if fraction_cmp != std::cmp::Ordering::Equal {
                return fraction_cmp;
            }
            // 条件2: ベクトルのindexで比較
            x1[2].cmp(&x2[2])
        }
    );

    ans.reverse();
    for i in 0..n {
        print!("{} ", ans[i][2] * (-1));
    }
}
