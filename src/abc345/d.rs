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
    // DFSのバックトラック (袋小路まで行ったら、1歩戻る。帰りがけにseenを元に戻せば良い。) による順列全探索は、次のページを参考 https://zenn.dev/yamasakit/articles/b40cc269f80b1b
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
    let mut seen = vec![vec![false; w]; h];

    // 順列全探索を、bit-maskで管理 <- 64 msec
    let mask = (1 << n) - 1; // v桁目が1なら、タイルvが未使用
    if dfs2(mask, &mut seen, &a, &b) {
        println!("Yes");
        return
    }

    // 順列全探索を、hash-setで管理 <- 85 msec
    // let mut hash: HashSet<usize> = (0..n).collect();
    // if dfs(&mut hash, &mut seen, &a, &b) {
    //     println!("Yes");
    //     return
    // }
    println!("No");
}

fn dfs2(mask: usize, seen: &mut Vec<Vec<bool>>, a: &Vec<usize>, b: &Vec<usize>) -> bool {
    // 順列全探索を、bit-maskで管理
    // mask := 未使用のタイル

    // 左上からタイルを貼っていく
    let h = seen.len();
    let w = seen[0].len();
    
    // 全てのマスにタイルが隙間なく敷かれているか、確認O(HW)
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

    for v in 0..a.len() {
        if mask & (1 << v) == 0 {continue}

        for (hv, wv) in [(a[v], b[v]), (b[v], a[v])] {
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
                
                // 「  ^ 1 」は、bit反転(0->1, 1->0)を意味する。
                // ただし、既に 0 のものは、↑の mask & (1 << v) == 0 {continue} で弾いているので、
                // ここでの「  ^ 1 」は、実質 1 -> 0 への変換、つまり、未使用 -> 使用済 への変換を
                if dfs2(mask ^ (1 << v), seen, a, b) {
                    return true
                }
                remove_tile(yp, xp, hv, wv, seen);
            }
        }
    }
    return false
}




fn dfs(hash: &mut HashSet<usize>, seen: &mut Vec<Vec<bool>>, a: &Vec<usize>, b: &Vec<usize>) -> bool {
    // 順列全探索を、hash-setで管理
    // hash := 未使用のタイル

    // 左上からタイルを貼っていく
    let hash2 = hash.clone();
    let h = seen.len();
    let w = seen[0].len();
    
    // 全てのマスにタイルが隙間なく敷かれているか、確認O(HW)
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

        for (hv, wv) in [(a[v], b[v]), (b[v], a[v])] {
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
                if dfs(hash, seen, a, b) {
                    return true
                }
                remove_tile(yp, xp, hv, wv, seen);
            }
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