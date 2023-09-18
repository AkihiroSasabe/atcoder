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
    // 18:24-
    input! {
        n: usize,
        x: usize,
        y: usize,
    }
    let mut p = vec![];
    let mut t = vec![];
    for i in 0..(n-1) {
        input! {
            p_i: usize,
            t_i: usize,
        }
        p.push(p_i);
        t.push(t_i);
    }
    input! {
        num_q: usize,
        q: [usize; num_q],
    }

    // 1<=Pi<=8 なことに注目
    // バス停1からバス停Nまでの所要時間には、バス停1の発射時刻に対して周期性がある。
    // (1,2,3,4,5,6,7,8)の最小公倍数は840であり、バス停1からバス停Nまでの所要時間は、
    // バス停1の発射時刻に対する周期840である。
    let lcm = 840; // 最小公倍数

    let mut times_from_1_to_n = vec![];
    for arrival_time_1 in 0..lcm {
        let arrival_time_n: usize = dfs(arrival_time_1, 0, n, &p, &t);
        times_from_1_to_n.push(arrival_time_n - arrival_time_1);
    }
    for i in 0..num_q {
        let time_to_arrive_station1 = q[i] + x;
        let time_on_bus = times_from_1_to_n[time_to_arrive_station1 % lcm];
        let ans = time_to_arrive_station1 + time_on_bus + y;
        println!("{}", ans);
    }
}

// バス停1への到着時刻から、バス停Nに到着する時刻を求める。
fn dfs(arrival_time: usize, current_station: usize, n: usize, p: &Vec<usize>, t: &Vec<usize>) -> usize {
    // println!("current_station = {}, arrival_time = {}", current_station, arrival_time);
    if current_station == n - 1 {
        return arrival_time
    }
    let mut departure_time = (arrival_time / p[current_station]) * p[current_station];
    if arrival_time % p[current_station] != 0 {
        departure_time += p[current_station];
    }
    // println!("departure_time = {}", departure_time);
    let arrival_time_for_next_station = departure_time + t[current_station];
    
    return dfs(arrival_time_for_next_station, current_station + 1, n, p, t)
}