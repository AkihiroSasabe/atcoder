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
    input! {
        mut n: usize
    }
    let list = vec![2,4,6,8];
    let list2 = vec![0,2,4,6,8];

    if n <= 5 {
        println!("{}", list2[n-1]);
        return
    }
    let mut cum: Vec<usize> = vec![0; 26];
    // 95_367_431_640_625
    // 476_837_158_203_125
    // 298_023_223_876_953_125

    cum[1] = 5;
    let mut pow5 = 1;
    let mut pow5s = vec![];
    pow5s.push(pow5);
    for i in 2..26 {
        pow5 *= 5;
        pow5s.push(pow5);
        cum[i] = cum[i-1] + pow5 * 4;
    }
    let keta = cum.lower_bound(&n);

    // println!("cum = {:?}", cum);
    // println!("cum[{keta}] = {}", cum[keta]);

    // ind桁
    let r = n - cum[keta-1] - 1;
    // println!("r = {:?}", r);


    let top_ind = r / pow5s[keta-1];
    let top = list[top_ind];

    let mut r_ind = r % pow5s[keta-1];
    let mut ans = vec![0; keta];
    ans[keta - 1] = top;
    let mut count = 0;
    while r_ind != 0 {
        let ti = r_ind % 5;
        ans[count] = list2[ti];
        count += 1;
        r_ind /= 5;
    }
    ans.reverse();
    for i in 0..ans.len() {
        print!("{}", ans[i]);
    }


    // let mut nn = n;
    // let mut ans = vec![];
    // while nn != 0 {
    //     let ind = nn % 5;
    //     ans.push(ind);
    //     nn /= 5;
    // }



    // ans.reverse();
    // print!("{}", list[ans[0]]);
    // for i in 1..ans.len() {
    //     print!("{}", list2[ans[i]]);
    // }



}