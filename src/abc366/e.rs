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
    // 2024-08-13 12:02-13:15 (1h13min)
    input! {
        n: usize,
        d: usize,
    }
    let mut y = vec![];
    let mut x = vec![];
    for i in 0..n {
        input! {
            xi: isize,
            yi: isize,
        }
        x.push(xi);
        y.push(yi);
    }
    x.sort();
    y.sort();

    // 0 ~ xmax+d
    // xの下限は、min({xi}) - d >= - 2 * 10^6 
    // xの上限は、max({xi}) + d <= 2 * 10^6 

    // yの下限は、min({yi}) - d >= - 2 * 10^6 
    // yの上限は、max({yi}) + d <= 2 * 10^6 
    let xmin = *x.iter().min().unwrap();
    let ymin = *y.iter().min().unwrap();

    // x = 0 上に、何個あるか考える。
    // 全点を正の値にオフセットさせる
    for i in 0..n {
        x[i] = x[i] - xmin + d as isize;
        y[i] = y[i] - ymin + d as isize;
    }

    // オフセット後の、min, maxを求めておく。
    let xmin = *x.iter().min().unwrap();
    let xmax = *x.iter().max().unwrap();

    let ymin = *y.iter().min().unwrap();
    let ymax = *y.iter().max().unwrap();

    let mut y_list = vec![0; (ymax + d as isize + 1) as usize];
    let mut x_list = vec![0; (xmax + d as isize + 1) as usize];

    let mut y_sum = 0;
    let mut x_sum = 0;
    for i in 0..n {
        x_sum += x[i];
        y_sum += y[i];
    }

    // y = 0 のときの、全xの各頂点からの距離の和を求める
    let mut ans = 0;
    x_list[0] = x_sum + y_sum;
    for i in 0..x_list.len() {
        if x_list[i] <= d as isize{
            ans += 1;
        }

        if i == x_list.len() - 1 {break}
        
        let ind = x.upper_bound(&(i as isize));
        let dec = (n - ind) as isize;
        let inc = ind as isize;
        
        x_list[i+1] = x_list[i] - dec + inc;
    }

    let mut dists = vec![];
    for i in 0..x_list.len() {
        dists.push(x_list[i]);
    }
    dists.sort();

    let mut diff = 0;
    for i in 1..y_list.len() {
        let ind = y.upper_bound(&((i - 1) as isize));
        let dec = (n - ind) as isize;
        let inc = ind as isize;
        diff = diff - dec + inc;

        let ind2 = dists.upper_bound(&(d as isize - diff));
        let num_under_d = ind2;
        ans += num_under_d;
    }
    println!("{}", ans);

}