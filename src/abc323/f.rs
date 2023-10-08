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
    // 2023-10-08 11:01-11:35 (34min)
    input! {
        xa: isize, // 高橋の位置
        ya: isize,
        xb: isize, // 荷物の現在位置
        yb: isize,
        xc: isize, // 荷物の目的位置
        yc: isize,
    }

    // 荷物の移動量
    let work_dist = (xc - xb).abs() + (yc - yb).abs();

    let mut anss: Vec<isize> = vec![];
    // 左から始める場合
    if xc - xb > 0 {
        let init_xt = xb - 1;
        let init_yt = yb;
        // 
        let init_t_move = (init_xt - xa).abs() + (init_yt - ya).abs();
        let mut total_move = init_t_move + work_dist;
        if (yc - yb).abs() > 0 {
            total_move += 2;
        }

        // 高橋と荷物の初期位置が、一直線上にある場合
        if ya == yb && xa > xb {
            total_move += 2;
        }

        anss.push(total_move);
    }
    // 右から始める場合
    if xc - xb < 0 {
        let init_xt = xb + 1;
        let init_yt = yb;
        let init_t_move = (init_xt - xa).abs() + (init_yt - ya).abs();
        let mut total_move = init_t_move + work_dist;
        if (yc - yb).abs() > 0 {
            total_move += 2;
        }

        // 高橋と荷物の初期位置が、一直線上にある場合
        if ya == yb && xa < xb {
            total_move += 2;
        }


        anss.push(total_move);
    }

    // 上から始める場合
    if yc - yb > 0 {
        let init_xt = xb;
        let init_yt = yb - 1;
        let init_t_move = (init_xt - xa).abs() + (init_yt - ya).abs();
        let mut total_move = init_t_move + work_dist;
        if (xc - xb).abs() > 0 {
            total_move += 2;
        }

        // 高橋と荷物の初期位置が、一直線上にある場合
        if xa == xb && ya > yb {
            total_move += 2;
        }

        anss.push(total_move);
    }

    // 下から始める場合
    if yc - yb < 0 {
        let init_xt = xb;
        let init_yt = yb + 1;
        let init_t_move = (init_xt - xa).abs() + (init_yt - ya).abs();
        let mut total_move = init_t_move + work_dist;
        if (xc - xb).abs() > 0 {
            total_move += 2;
        }
        // 高橋と荷物の初期位置が、一直線上にある場合
        if xa == xb && ya < yb {
            total_move += 2;
        }
        anss.push(total_move);
    }

    // 上記のどれにも該当しないケース
    if yc == yb && xc == xb {
        anss.push(0);
    }
    anss.sort();
    println!("{}", anss[0]);



}