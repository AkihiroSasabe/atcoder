#![allow(dead_code, unused_imports)]
use proconio::input;
use itertools::Itertools;
use core::num;
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
    // 2023-09-21 21:31-22:27 (56min)
    // 23:51-24:30 (39min)
    // 19:52-20:35 (43min)
    // total: 138min
    input! {
        n: usize,
        m: usize,
    }
    let mut t = vec![];
    let mut x = vec![];
    for i in 0..n {
        input! {
            t_i: usize,
            x_i: usize,
        }
        t.push(t_i);
        x.push(x_i);
    }

    // (1)タイプ毎に分類
    // free_delight_list[i] := i番目のtype0の缶を得る喜び (不要)
    // locked_delight_list[i] := i番目のtype1の缶を得る喜び
    // cutter_list[i] := i番目の缶切りが、開けられる缶の本数
    let mut free_delight_list = vec![];
    let mut locked_delight_list = vec![];
    let mut cutter_list = vec![];
    for i in 0..n {
        if t[i] == 0 {
            free_delight_list.push(x[i]);
        }
        if t[i] == 1 {
            locked_delight_list.push(x[i]);
        }
        if t[i] == 2 {
            cutter_list.push(x[i]);
        }
    }
    free_delight_list.sort();
    free_delight_list.reverse();
    locked_delight_list.sort();
    locked_delight_list.reverse();
    cutter_list.sort();
    cutter_list.reverse();

    // 長さをm+1にする
    // 少なければ喜び0を末尾に追加
    while free_delight_list.len() < m + 1 {
        free_delight_list.push(0);
    }
    while locked_delight_list.len() < m + 1 {
        locked_delight_list.push(0);
    }
    while cutter_list.len() < m + 1 {
        cutter_list.push(0);
    }
    // 多ければ末尾を削除
    while free_delight_list.len() > m + 1 {
        free_delight_list.pop();
    }
    while locked_delight_list.len() > m + 1 {
        locked_delight_list.pop();
    }
    while cutter_list.len() > m + 1 {
        cutter_list.pop();
    }

    // (2)累積和
    // cum_cutter[i] := 缶切りをi個入手したときに、切ることが出来る缶切りの合計本数
    // let mut cum_cutter = vec![0; max(m, cutter_list.len())];
    // let mut cum_cutter = vec![0; cutter_list.len()];
    let mut cum_cutter = vec![0; m + 1];
    if cutter_list.len() > 0 {
        cum_cutter[0] = cutter_list[0];
    }
    for i in 1..m+1 {
        if i < cutter_list.len() {
            cum_cutter[i] = cum_cutter[i-1] + cutter_list[i];
        }
        else {
            cum_cutter[i] = cum_cutter[i-1];
        }
    }

    // cum_free_delight[i] := 缶切り不要な缶をi個入手したときに、得られる満足度
    // let mut cum_free_delight = vec![0; max(m, free_delight_list.len())];
    // let mut cum_free_delight = vec![0; free_delight_list.len()];
    let mut cum_free_delight = vec![0; m + 1];
    if free_delight_list.len() > 0 {
        cum_free_delight[0] = free_delight_list[0];
    }
    for i in 1..m+1 {
        cum_free_delight[i] = cum_free_delight[i-1] + free_delight_list[i];
    }

    // cum_locked_delight[i] := 缶切り必要な缶をi個入手したときに、得られる満足度
    // let mut cum_locked_delight = vec![0; max(m, locked_delight_list.len())];
    // let mut cum_locked_delight = vec![0; locked_delight_list.len()];
    let mut cum_locked_delight = vec![0; m + 1];
    if locked_delight_list.len() > 0 {
        cum_locked_delight[0] = locked_delight_list[0];
    }
    for i in 1..m+1 {
        cum_locked_delight[i] = cum_locked_delight[i-1] + locked_delight_list[i];
    }

    // 0個の場合、0になるようにする。list[0] = 0
    cum_free_delight.insert(0, 0);
    cum_locked_delight.insert(0, 0);
    cum_cutter.insert(0, 0);

    // 実質的なロック缶の本数
    let mut effective_num_lock = vec![0; m+1];
    let mut num_lock = 0;
    let mut break_flag = false;
    for num_cutter in 0..m+1 {
        let num_lock_max = cum_cutter[num_cutter];
        while num_lock <= num_lock_max {
            if num_lock + num_cutter > m {
                break_flag = true;
                break
            }
            effective_num_lock[num_lock + num_cutter] = num_lock;
            num_lock += 1;
        }
        if break_flag {break}
    }

    // 実質的な満足度
    // effective_locked_delight[i] := 缶切りと必要な缶の合計がi本の時の満足度の最大値
    let mut effective_locked_delight = vec![0; m+1];
    for num_both in 0..m+1 {
        effective_locked_delight[num_both] =  cum_locked_delight[effective_num_lock[num_both]];
    }
    // println!("cum_free_delight = {:?}", cum_free_delight);
    // println!("cum_locked_delight = {:?}", cum_locked_delight);
    // println!("cum_cutter = {:?}", cum_cutter);

    let mut ans = 0;
    for i in 0..m+1 {
        ans = max(ans, effective_locked_delight[m-i] + cum_free_delight[i]);
    }
    println!("{}", ans);

}
