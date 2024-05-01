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
    // 2024-05-01 12:54-13:47 (53min)
    input! {
        y0: isize,
        x0: isize,
        y1: isize,
        x1: isize,
    }

    if y0 == y1 && x0 == x1 {
        // 同一点
        println!("0");
        return;
    }
    if y0 + x0 == y1 + x1 {
        // ＼の直線方程式
        println!("1");
        return;
    }
    if y0 - x0 == y1 - x1 {
        // ／の直線方程式
        println!("1");
        return;
    }
    if ((y0 - y1).abs() + (x0 - x1).abs()) <= 3 {
        // 3点でいける距離
        println!("1");
        return;
    }


    // 点(y1,x1)と直線(ax+by+c=0)の距離の方程式
    // d = |a * x1 + b * y1 + c| / √(a^2+b^2)

    // ＼の直線方程式
    // y0 + x0 = y1 + x1 
    // <=> x1 + y1 - x0 - y0 = 0
    // <=> (a,b,c) = (1, 1, -x0-y0)

    // ／の直線方程式
    // y0 - x0 = y1 - x1 
    // <=> x1 - y1 - x0 + y0 = 0
    // <=> (a,b,c) = (1, -1, x0-y0)

    // 点と直線の距離に、√(a^2+b^2)=√2 をかけたもの
    let d0 = (x1 + y1 - x0 - y0).abs();
    let d1 = (x1 - y1 - x0 + y0).abs();

    // 2手でいける。
    if ((y0 - y1).abs() + (x0 - x1).abs()) % 2 == 0 {
        // 距離が偶数なら、／と＼の組み合わせでどこでも、2手でいける。
        println!("2");
        return;
    }
    else if d0 <= 4 || d1 <= 4 {
        // 距離が奇数でも、中心の5x5マス上からスターㇳした点なら、どこでも2手でいける。
        // d / √2 <= 2√2 
        // <=> d <= 4
        println!("2");
        return;
    }
    else if ((y0 - y1).abs() + (x0 - x1).abs()) <= 6 {
        // 6手で行ける距離
        println!("2");
        return;
    }
    else {
        println!("3");
        return;
    }


}