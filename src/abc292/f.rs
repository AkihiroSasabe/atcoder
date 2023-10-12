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
    // 2023-10-11 12:00-12:42 (42min)
    // 2023-10-11 20:00-20:52 (52min)
    // total 94 min

    // 浮動小数点数の計算についての参考
    // 浮動小数点数は、(s, e, [b])の3つの要素で表される。
    // (-1)^s * 2^(e-1023) * (1.b51 b50 ... b1 b0)
    // 1.0 <=> s=0, e=1023, b_all=0
    // 0.0 <=> s=0. e=0, b_all=0
    // https://rsk0315.hatenablog.com/entry/2022/06/04/190255
    // https://bartaz.github.io/ieee754-visualization/

    // 13 15 の入力例(04_border_02.txt)で誤差が大きくなる
    // 15.000012336536473
    // 15.000007843660276 <- 10^-6以上の誤差が出てしまう。
    // 大きい桁と、小さい桁の足し算などに注意しましょう。
    input! {
        a: usize,
        b: usize
    }

    // 短辺を縦、長辺を横、と置き換える。
    let short = min(a,b) as f64;
    let long = max(a,b) as f64;
    
    // 10^(-12)
    // deltaを小さくしすぎると、浮動小数点の演算の都合上、桁あまり（？）して、答えが合わなくなる
    // let delta = 0.000_000_000_000_1;
    // let min_error = 0.000_000_000_000_1;

    // ACする精度の大きさ
    // 上に凸な関数の極大値を求める処理(二分探索で導関数の符号が変わる位置を取得する)
    let delta: f64     = 0.000_000_000_01; // 10^(-10) 小さくしすぎないように注意
    let min_error: f64 = 0.000_000_000_01;
    let mut ng: f64 = 30.0;             // 導関数が負
    let mut ok: f64 = 15.0 - min_error; // 導関数が正
    while (ng - ok).abs() > min_error {
        let mid = (ng + ok) / 2.0;
        let fx = get_fx(mid, short, long);
        let fx_plus = get_fx(mid + delta, short, long);
        // let diff = x_plus - x;
        // println!("ok = {ok}, ng = {ng}, mid = {mid}, diff = {diff}");

        if fx_plus > fx {
            // midの導関数が正だったとき
            ok = mid;
        }
        else {
            // midの導関数が負だったとき
            ng = mid;
        }
    }
    let fx = get_fx(ok, short, long);
    println!("{}", fx);

}

fn get_fx(mid: f64, short: f64, long: f64) -> f64 {
    
    let theta = PI * mid / 180.0;

    let mid_r = 30.0 - mid;
    let theta_r = PI * mid_r / 180.0;

    let x1 = short / theta.cos();
    let x2 = long / theta_r.cos();

    // println!("mid = {mid}, theta = {theta}, theta_r = {theta_r}, x1 = {x1}, x2 = {x2}");
    let x = x1.min(x2);

    return x

}