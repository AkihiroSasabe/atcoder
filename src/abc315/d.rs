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
        h: usize,
        w: usize,
        c: [Chars; h]
    }
    solve(h, w, c);
}

fn solve(h: usize, w: usize, c: Vec<Vec<char>>) {
    // 方針
    // 下記の3種類 x 2(xとy) = 6個のスタックとハッシュセットで、状態を管理していく
    // num_cookies_for_color_x[color][x] := x列目のcolorの数 (Stack)
    // [1]num_cookies_for_color_x
    // [2]num_cookies_for_color_y

    // 残っている行と列のindex (HashSet)
    // [3]remaining_y
    // [4]remaining_x

    // num_remaining_y[y] := y行目に何個残っているか (Stack)
    // [5]num_remaining_y
    // [6]num_remaining_x


    // char -> 数字に置き換え
    let a_num = 'a' as usize;

    // 色毎の縦と、横の数
    // num_cookies_for_color_x[color][x] := x列目のcolorの数
    // num_cookies_for_color_y[color][y] := y行目のcolorの数
    let mut num_cookies_for_color_x = vec![vec![0 as isize; w]; 26];
    let mut num_cookies_for_color_y = vec![vec![0 as isize; h]; 26];
    for y in 0..h {
        for x in 0..w {
            // char -> 数字に置き換え ('a'が0, 'b'が1, ..., 'z'が25)
            let color_num = c[y][x] as usize - a_num;
            num_cookies_for_color_x[color_num][x] += 1;
            num_cookies_for_color_y[color_num][y] += 1;
        }
    }

    // 残っている行と列のindex
    let mut remaining_y = HashSet::new(); // 残っている行
    let mut remaining_x = HashSet::new(); // 残っている列
    // 初期化 (最初は全行と全列が残っている)
    for i in 0..h {
        remaining_y.insert(i);
    }
    for i in 0..w {
        remaining_x.insert(i);
    }
    
    // num_remaining_y[y] := y行目に何個残っているか
    let mut num_remaining_y = vec![w as isize; h];
    
    // num_remaining_x[x] := x列目に何個残っているか
    let mut num_remaining_x = vec![h as isize; w];
    
    loop {
        // 操作1: 行毎に印を付ける
        let mut remove_list_y = vec![];
        for &y in &remaining_y {
            // 2枚以上のクッキーが残っていない場合はスキップ
            if num_remaining_y[y] < 2 {continue}
            for i in 0..26 {
                if num_cookies_for_color_y[i][y] == remaining_x.len() as isize {
                    remove_list_y.push((i, y));
                }
            }
        }

        // 操作2: 列毎に印を付ける
        let mut remove_list_x = vec![];
        for &x in &remaining_x {
            // 2枚以上のクッキーが残っていない場合はスキップ
            if num_remaining_x[x] < 2 {continue}
            for i in 0..26 {
                if num_cookies_for_color_x[i][x] == remaining_y.len() as isize {
                    remove_list_x.push((i, x));
                }
            }
        }

        // 操作3: 印をつけたやつを削除していく
        // 印を付けていなかったらループ終了
        if remove_list_y.len() == 0 && remove_list_x.len() == 0 {break}
        for &(i, y) in remove_list_y.iter() {
            // y行目を全部削除
            num_cookies_for_color_y[i][y] = 0;
            remaining_y.remove(&y);
            num_remaining_y[y] = 0;

            // 色iが居る列を削除していく
            for &x in &remaining_x {
                num_cookies_for_color_x[i][x] -= 1;
                num_remaining_x[x] -= 1;
            }
        }

        for &(i, x) in remove_list_x.iter() {
            // x列目を全部削除
            num_cookies_for_color_x[i][x] = 0;
            remaining_x.remove(&x);
            num_remaining_x[x] = 0;

            // 色iが居る行を削除していく
            for &y in &remaining_y {
                num_cookies_for_color_y[i][y] -= 1;
                num_remaining_y[y] -= 1;
            }
        }
    }

    println!("{}", remaining_y.len() * remaining_x.len());
}
