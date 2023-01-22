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
    // 2023-01-18 22:48-23:16 (28min)
    input! {
        n: usize,
    }
    let mut events = vec![];
    for i in 0..n {
        input! {
            a_i: usize,
            b_i: usize,
        }
        // イベントの始点は0, 終点は1とする
        events.push(vec![a_i, 0]);
        events.push(vec![a_i+b_i, 1]);
    }
    // イベントのタイミング順にソート
    events.sort();

    // 現在ログインしているユーザー数
    let mut active_num = 0;

    // k人が同時にログインしている総日数
    let mut login_time = vec![0; n+1];
    
    let mut pre = 1;
    for i in 0..events.len() {
        let now = events[i][0];
        login_time[active_num] += now - pre;
        // イベントが始点のとき
        if events[i][1] == 0 {
            active_num += 1;
        }
        // イベントが終点のとき
        else {
            active_num -= 1;
        }
        pre = now;
    }
    for k in 1..n+1 {
        print!("{} ", login_time[k]);
    }

}