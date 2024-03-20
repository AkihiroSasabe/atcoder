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
    // 2024-03-20 13:11-14:49 (1h38min = 98min)
    input! {
        n: usize,
        h: usize,
        w: usize,
    }
    let mut a = vec![];
    let mut b = vec![];
    for i in 0..n {
        input!{
            ai: usize,
            bi: usize,
        }
        a.push(ai);
        b.push(bi);
    }

    // Back-Track
    // N枚のタイルを貼る順番 O(N!) = 7! = 5040
    // 縦横それぞれ考えると、O(2^N) = 128
    // 全てのマスにタイルを貼ったり剥がしたりチェックする計算量：O(HxW) =100
    // Total = O(N! * 2^N * HW) = 64_512_000 <= 7*10^7
    let mut hash: BTreeSet<usize> = (0..n).collect();
    let mut seen = vec![vec![false; w]; h];

    for bits in 0..1<<n {
        if dfs(&mut hash, bits, &mut seen, &a, &b) {
            println!("Yes");
            return
        }
    }
    println!("No");
}

fn dfs(hash: &mut BTreeSet<usize>, bits: usize, seen: &mut Vec<Vec<bool>>, a: &Vec<usize>, b: &Vec<usize>) -> bool {
    // 左上からタイルを貼っていく
    let hash2 = hash.clone();
    let h = seen.len();
    let w = seen[0].len();
    
    let mut is_completed = true;
    for i in 0..h {
        for j in 0..w {
            if !seen[i][j] {
                is_completed = false;
                break
            }
        }
        if !is_completed {
            break
        }
    }
    if is_completed {
        return true
    }

    for &v in hash2.iter() {
        hash.remove(&v);

        // d := 桁番号
        let d = hash.len();
        let (hv, wv) = if bits & (1 << d) != 0 {
            // 縦に置く
            (a[v], b[v])
        }
        else {
            // 横に置く
            (b[v], a[v])
        };

        // 置ける場所を見つける
        let mut can_place = false;
        let mut yp = 0;
        let mut xp = 0;
        for y in 0..h {
            for x in 0..w {
                if judge_place(y, x, hv, wv, seen) {
                    can_place = true;
                    yp = y;
                    xp = x;
                    break
                }
            }
            if can_place{break}
        }

        if can_place {
            place(yp, xp, hv, wv, seen);
            if dfs(hash, bits, seen, a, b) {
                return true
            }
            remove_tile(yp, xp, hv, wv, seen);
        }
        // Back Track
        hash.insert(v);
    }
    return false
}

fn judge_place(y: usize, x: usize, hi: usize, wi: usize, seen: &Vec<Vec<bool>>) -> bool {
    let h = seen.len();
    let w = seen[0].len();
    if h < y + hi || w < x + wi {
        return false
    }
    for r in 0..hi {
        for c in 0..wi {
            let yi = y + r;
            let xi = x + c;
            if seen[yi][xi] {
                return false
            }
        }
    }
    return true
}

fn place(y: usize, x: usize, hi: usize, wi: usize, seen: &mut Vec<Vec<bool>>) {
    for r in 0..hi {
        for c in 0..wi {
            let yi = y + r;
            let xi = x + c;
            seen[yi][xi] = true;
        }
    }
}

fn remove_tile(y: usize, x: usize, hi: usize, wi: usize, seen: &mut Vec<Vec<bool>>) {
    for r in 0..hi {
        for c in 0..wi {
            let yi = y + r;
            let xi = x + c;
            seen[yi][xi] = false;
        }
    }
}