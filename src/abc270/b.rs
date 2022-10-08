use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    input! {
        x: isize,
        y: isize,
        z: isize,
    }

    let mut flag = true;
    let mut ans = 0;
    // 無理なケース
    if y > 0 && y < z && y < x {
        flag = false;
    }
    // 無理なケース
    if y < 0 && y > z && y > x {
        flag = false;
    }

    // 壁を超えなくていいケース
    if y < 0 && y < x {
        ans = x.abs();
    }
    // 壁を超えなくていいケース
    else if y > 0 && x < y {
        ans = x.abs();
    }
    // 壁を超えるケース
    else if y * z < 0 {
        ans = z.abs() + (x - z).abs();
    }
    // 壁を超えるケース
    else {
        ans = x.abs();
    }


    if flag {
        println!("{}", ans);
    }
    else {
        println!("-1");
    }
}