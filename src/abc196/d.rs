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
    // 2024-03-07 21:46-22:00 (14min)
    // 2024-03-07 22:05-22:22 (17min)
    // 2024-03-08 21:32-22:47 (1h15min)
    // Total 1h46min
    input! {
        h: usize,
        w: usize,
        a: usize,
        b: usize,
    }

    // [1] a の場所を決める Maxで16_C_max(a) = 16C8 = 12_870
    // [2] a の向きを決める Maxで1 << max(a) = 1 << 8 = 256
    // [3] あとはaが枠外にはみ出ない、かつ、他のaと干渉しない、という条件さえ満たせば、
    // bは隙間に埋められるので合格
    // 計算量は 12870 * 256 = 3_294_720

    // a の場所を決める
    let mut ans = 0;
    for comb in (0..h*w).combinations(a) {
        // println!("comb = {:?}", comb);

        // 各aの向きを決める。
        for bit in 0..1<<comb.len() {
            let mut is_ok = true;
            let mut seen = vec![vec![false; w]; h];
            for (i, &bi) in comb.iter().enumerate() {
                let y = bi / w;
                let x = bi % w;
                let mut ny = y;
                let mut nx = x;
                if bit & 1 << i != 0 {
                    // aを縦におく
                    ny += 1;
                }
                else {
                    // aを横におく
                    nx += 1;
                }
                if ny >= h || nx >= w || seen[y][x] || seen[ny][nx]{
                    is_ok = false;
                    break
                }
                seen[y][x] = true;
                seen[ny][nx] = true;
            }
            // println!("bit = {:04b}, is_ok = {:?}", bit, is_ok);
            if is_ok {
                ans += 1;
            }
        }
    }
    println!("{}", ans);

}