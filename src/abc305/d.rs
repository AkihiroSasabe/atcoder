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
        n: usize,
        a: [isize; n],
        q: usize,
    }
    // i個目を含めた睡眠時間の合計
    let mut cum_sleep_time = vec![0; n];
    // // i個目の開始時刻
    // let mut elapsed_time = vec![0; n];
    for i in 1..n {
        if i % 2 == 0 {
            cum_sleep_time[i] = cum_sleep_time[i-1] + a[i] - a[i-1];
        }
        else {
            cum_sleep_time[i] = cum_sleep_time[i-1];
        }
        // elapsed_time[i] += elapsed_time[i-1] + a[i-1];
    }
    let mut ll = vec![];
    let mut rr = vec![];
    for i in 0..q {
        input!{ 
            l_i: isize,
            r_i: isize,
        }
        ll.push(l_i);
        rr.push(r_i);
    }
    for i in 0..q {
        let l_i = ll[i];
        let r_i = rr[i];
        // 睡眠開始index
        let index_l = a.upper_bound(&l_i);
        // 睡眠終了index
        let index_r = a.upper_bound(&r_i);
        // println!("index_l={}, index_r={} ", index_l, index_r);
        
        // 短めに取っておく
        let mut ans = cum_sleep_time[index_r - 1] - cum_sleep_time[index_l];
        // println!("small {}", ans);
        // index_l - 1が寝ていたら
        if index_l % 2 == 0 {
            ans += a[index_l] - l_i;
            // println!("totyu: {} ", ans);
        }
        if index_r % 2 == 0 {
            ans += r_i - a[index_r-1];
        }

        println!("{} ", ans);
    }

}