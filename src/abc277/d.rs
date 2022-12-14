#![allow(dead_code, unused_imports)]
use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; n],
    }

    a.sort();
    // println!("{:?}", a);

    let mut sum_list = vec![];
    let mut sum = a[0];
    if a.len() == 1 {
        sum_list.push(a[0])
    }
    else {
        for i in 1..a.len() {
            if a[i] == a[i-1] || a[i] == (a[i-1] + 1) % m {
                sum += a[i];
            }
            else {
                sum_list.push(sum);
                sum = a[i];
            }
            // 数列最後の処理
            if i == a.len() - 1 {
                if a[0] == (a[a.len()-1] + 1) % m  && sum_list.len() > 0 {
                    sum_list[0] += sum;
                }
                else {
                    sum_list.push(sum);
                }
            }
        }    
    }
    // println!("sum_list: {:?}", sum_list);
    let a_sum: usize = a.iter().sum();
    let ans = a_sum - *sum_list.iter().max().unwrap();
    println!("{}", ans);
}
