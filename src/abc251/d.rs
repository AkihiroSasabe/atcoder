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
    input! {
        w: usize
    }
    // 1 <= N <= 300
    // 1 <= a[i] <= 10^6
    // 1 <= W <= 10^6
    // 300個の数字で1-10^6まで全部埋める
    // nC3 = n * n-1 * n-2 / 6 = 300*299*298/6= 4,455,100
    // 1,2,3,4,5,6,7,8,9
    // 10,20,30,40,50,60,70,80,90
    // 20個の数字で100までは表せる

    // 6桁の数字を3つの数でなんとか表現する
    // 11 22 33
    // 100 * 99 * 99
    // 300個ある！
    // println!("300");
    
    let mut ans = vec![];
    for i in 1..100 {
        ans.push(i);
        ans.push(i*100);
        ans.push(i*10000);
    }
    ans.push(1000000);
    println!("{}", ans.len());
    for i in 0..ans.len() {
        print!("{} ", ans[i]);
    }



    


}