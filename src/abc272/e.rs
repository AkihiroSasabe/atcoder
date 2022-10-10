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
        mut a: [isize; n]
    }

    // t = [1, m] のm個分のsetを用意
    // 各set毎に0 <= a[i][t] <= n - 1となる数列の項を入れる
    // 計算量O(N*logN)
    let mut set: HashSet<isize> = HashSet::new();
    let mut set_list = vec![set; m+1];
    for i in 0..n {
        // a[i][t] = a[i][0] + (i+1)*t となる。
        //     0 <= a[i][t] <= n - 1 となるtを見つけ出す
        // <=> -a[i][0]/(i+1) <= t <= (n-1-a[i][0])/(i+1)
        let mut start_time = - a[i] / (i+1) as isize;
        let mut end_time =(n  as isize - 1 - a[i]) / (i+1) as isize;
        // println!("i:{}, start_time:{}, end_time:{}", i, start_time, end_time);
        if start_time <= 0 {
            start_time = 1;
        }
        if end_time > m as isize {
            end_time = m as isize ;
        }
        // println!("==== i:{}, start_time:{}, end_time:{}", i, start_time, end_time);
        for t in start_time..(end_time+1) {
            let ai_t = a[i] + (i+1) as isize * t;
            set_list[t as usize].insert(ai_t);
        }
    }

    for i in 1..(m+1) {
        for j in 0..(n+1) {
            if !set_list[i].contains(&(j as isize)) {
                println!("{}", j);
                break
            }
        }
    }
    
}