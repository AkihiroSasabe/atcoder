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
    // 2024-01-14 12:21-13:30 (69min)
    // 2024-01-14 17:32-17:57 (25min)
    // total 94min
    input! {
        x: usize,
        y: usize,
        a: usize,
        b: usize,
        c: usize,
    }

    let mut ss = vec![a, b, c];


    // 長方形の3分割方法は、下記の4パターン https://x.com/kyopro_friends/status/1449733792583729161?s=20
    // (1)全部横並び (縦切り、縦切り)
    // ABC

    // (2)2個が上側に横並び、1個が下側 (縦切り、横切り)
    // AB
    //  C

    // (3)1個が上側に横並び、2個が下側 (横切り、縦切り)
    // A
    // BC

    // (4)全部が縦並び（横切り、横切り）
    // A
    // B
    // C

    // AとBとCの順番3P3=6 を順列でシャッフルする。
    // (1)なら、ABC, BCA, CAB, BAC, ACB, CBA  
    // 全部で4*6=24通り

    let mut flag = false;
    for s in ss.iter().permutations(3) {
        // 1個目の長方形を、枠の縦辺全てに埋める
        if let Some((rx, ry)) = h_judge(x, y, *s[0]) {
            // 2個目の長方形を、
            if let Some((rx2, ry2)) = h_judge(rx, ry, *s[1]) {
                if let Some((_rx3, _ry3)) = h_judge(rx2, ry2, *s[2]) {
                    flag = true;
                }
            }
            if let Some((rx2, ry2)) = w_judge(rx, ry, *s[1]) {
                if let Some((_rx3, _ry3)) = h_judge(rx2, ry2, *s[2]) {
                    flag = true;
                }
            }
        }
        if let Some((rx, ry)) = w_judge(x, y, *s[0]) {
            if let Some((rx2, ry2)) = h_judge(rx, ry, *s[1]) {
                if let Some((_rx3, _ry3)) = h_judge(rx2, ry2, *s[2]) {
                    flag = true;
                }
            }
            if let Some((rx2, ry2)) = w_judge(rx, ry, *s[1]) {
                if let Some((_rx3, _ry3)) = h_judge(rx2, ry2, *s[2]) {
                    flag = true;
                }
            }
        }
    }

    if flag {
        println!("Yes");
    }
    else {
        println!("No");
    }
}


fn h_judge(x: usize, y: usize, s: usize) -> Option<(usize, usize)> {
    // 長さx, yの長方形のスペースに、面積Sの長方形を、
    // 縦にはめ込めるか? (y軸に沿ってはめ込めるか?)
    // はめ込めた場合、はめ込んで残った部分の長方形のスペースの横と縦の長さ(rx, ry)を出力
    if y == 0 {
        // 残りyの辺の長さが0だと問答無用で無理
        return None
    }

    // はめ込む長方形のサイズ(w0, h0)
    let h0 = y;
    let w0 = match s % h0 == 0 {
        true => s / h0,
        false => s / h0 + 1
    };
    if w0 <= x {
        // 残りの長方形のサイズ
        let ry = y;
        let rx = x - w0;
        return Some((rx, ry))
    }
    else {
        return None
    }
}

fn w_judge(x: usize, y: usize, s: usize) -> Option<(usize, usize)> {
    // 長さx, yの長方形のスペースに、面積Sの長方形を、
    // 横にはめ込めるか? (x軸に沿ってはめ込めるか?)
    if x == 0 {
        // 残りxの辺の長さが0だと問答無用で無理
        return None
    }

    // はめ込む長方形のサイズ(w0, h0)
    let w0 = x;
    let h0 = match s % w0 == 0 {
        true => s / w0,
        false => s / w0 + 1
    };

    if h0 <= y {
        // 残りの長方形のサイズ
        let ry = y - h0;
        let rx = x;
        return Some((rx, ry))
    }
    else {
        return None
    }

}