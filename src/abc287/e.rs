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
    // 2023-10-15 18:41-19:25 (44min) 電話とかしたから実際はもう少し短時間でできたかも?
    input! {
        n: usize,
        mut s: [Chars; n]
    }

    let mut s_sorted = vec![];
    for i in 0..n {
        s_sorted.push((s[i].clone(), i));
    }
    s_sorted.sort();

    // debug
    // for i in 0..n {
    //     println!("{:?}", s_sorted[i]);
    // }

    let mut ans = vec![0; n];

    // 上か、下を比べればok
    for i in 1..n {
        let mut pre_i = 0;
        let mut now_i = 0;
        let mut count = 0;
        loop {
            if pre_i >= s_sorted[i-1].0.len() || now_i >= s_sorted[i].0.len() {break}
            if s_sorted[i-1].0[pre_i] != s_sorted[i].0[now_i] {break}
            count += 1;
            pre_i += 1;
            now_i += 1;
        }
        ans[s_sorted[i-1].1] = max(ans[s_sorted[i-1].1], count);
        ans[s_sorted[i].1] = max(ans[s_sorted[i].1], count);
    }
    for i in 0..n {
        println!("{}", ans[i]);
    }


    // 昔    
    // 2023-01-29 17:10-
    // input! {
    //     n: usize,
    //     mut s: [String; n]
    // }
    // let mut ss = vec![];
    // for i in 0..n {
    //     let i_string: String = i.to_string();
    //     ss.push(vec![s[i].clone(), i_string]);
    // }
    // ss.sort();
    // for i in 0..n {
    //     println!("ss[{}]: {:?}", i, ss[i]);
    // }

    // let mut counts = vec![0; n];
    // let mut pre=0;
    // let mut post=0;
    // for i in 0..n {
    //     // if i == 1 {
    //     //     pre = min(ss[i-1][0].len(), ss[i][0].len());
    //     //     for j in 0..min(ss[i-1][0].len(), ss[i][0].len()) {
    //     //         let c0 = ss[i-1][0].chars().nth(j).unwrap();
    //     //         let c1 = ss[i][0].chars().nth(j).unwrap();
    //     //         // println!("pre: i: {}, c0:{} c1:{}",i, c0, c1);
    //     //         if c0 != c1 {
    //     //             pre = min(pre, j);
    //     //             break
    //     //         }
    //     //     }
    //     // }

    //     if i+1 < n {
    //         post = min(ss[i+1][0].len(), ss[i][0].len());
    //         for j in 0..min(ss[i+1][0].len(), ss[i][0].len()) {
    //             let c0 = ss[i+1][0].chars().nth(j).unwrap();
    //             let c1 = ss[i][0].chars().nth(j).unwrap();
    //             // println!("post: i: {}, c0:{} c1:{}",i, c0, c1);
    //             if c0 != c1 {
    //                 post = min(post, j);
    //                 break
    //             }
    //         }
    //     }
    //     else {
    //         post = 0;
    //     }
    //     let c: usize = ss[i][1].parse().unwrap();
    //     // println!("i:{}, pre:{} post:{}, c:{}", i, pre, post, c);
    //     counts[c] = max(pre, post);
    //     pre = post;
    // }
    // for i in 0..n {
    //     println!("{}", counts[i]);
    // }
}