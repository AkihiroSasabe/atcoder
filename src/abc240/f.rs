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
    // 2023-12-12 18:41-19:22 (41min)ねむい
    // 2023-12-12 23:16-24:00 (44min)
    // 2023-12-13 19:33-20:50 (1h17min)
    // total: 162 min
    input! {
        t: usize,
    }
    let mut n = vec![];
    let mut m = vec![];

    let mut x = vec![];
    let mut y = vec![];
    for i in 0..t {
        input! {
            ni: usize,
            mi: usize,
            xyi: [(isize, isize); ni],
        }
        n.push(ni);
        m.push(mi);
        let mut xi = vec![];
        let mut yi = vec![];
        for j in 0..ni {
            xi.push(xyi[j].0);
            yi.push(xyi[j].1);
        }
        x.push(xi);
        y.push(yi);
    }

    // debug用
    // for i in 0..t {
    //     let mut real_a = vec![];
    //     let mut real_b = vec![];
    //     let mut real_c = vec![];
    //     let mut calc_a = vec![0];
    //     let mut calc_b = vec![0];
    //     let mut calc_c = vec![0];
    //     for j in 0..n[i] {
    //         for k in 0..y[i][j] {
    //             real_c.push(x[i][j]);
    //             if real_b.len() == 0 {
    //                 real_b.push(x[i][j]);
    //                 real_a.push(x[i][j]);
    //             }
    //             else {
    //                 let temp_b = real_b[real_b.len()-1] + x[i][j];
    //                 real_b.push(temp_b);
    //                 let temp_a = real_a[real_a.len()-1] + real_b[real_b.len()-1];
    //                 real_a.push(temp_a);
    //             }
    //             if k == y[i][j] - 1 {
    //                 calc_a.push(real_a[real_a.len()-1]);
    //                 calc_b.push(real_b[real_b.len()-1]);
    //                 calc_c.push(real_c[real_c.len()-1]);
    //             }
    //         }
    //     }
    //     println!("real_c = {:?}", real_c);
    //     println!("real_b = {:?}", real_b);
    //     println!("real_a = {:?}", real_a);
    //     println!("calc_c = {:?}", calc_c);
    //     println!("calc_b = {:?}", calc_b);
    //     println!("calc_a = {:?}", calc_a);
    // }

    // let mut ans: isize = - (1 << 60);
    for i in 0..t {

        let mut ans_i = x[i][0];
        

        // bの末尾を入れていく！
        // let mut c = vec![];
        let mut b = vec![];
        let mut a = vec![];
        // 累積和
        for j in 0..n[i] {
            if j == 0 {
                // b.push(x[i][j] * y[i][j]);    
                b.push(0);
                a.push(0);
            }
            else {
                b.push(b[j-1] + x[i][j-1] * y[i][j-1]);
                let mut temp = (b[j] + (b[j-1] + x[i][j-1])) * y[i][j-1] / 2;
                a.push(a[j-1] + temp);
            }
        }
        // println!("b = {:?}", b);
        // println!("a = {:?}", a);

        let mut diff_num = 0;
        for j in 0..n[i] {
            if x[i][j] < 0 {
                if b[j] + x[i][j] < 0 {
                    // println!("cond 0");
                    diff_num = 1;
                }
                else {
                    // println!("cond 1");
                    // bがプラスである限り足す
                    diff_num = min((b[j] / (- x[i][j])),  y[i][j]);
                    diff_num = max(1, diff_num);
                }
            }
            else {
                // println!("cond 2");
                diff_num = y[i][j];
            }
            // println!("diff_num = {:?}", diff_num);
            let temp_ans = a[j] + ((b[j] + x[i][j]) + (b[j] + diff_num * x[i][j])) * diff_num / 2;
            // println!("temp_ans = {:?}", temp_ans);
            ans_i = max(ans_i, temp_ans);
        }
        println!("{}", ans_i);
    }
}