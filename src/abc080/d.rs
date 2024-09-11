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
use rand::Rng;
fn main() {
    // 2024-09-10 20:29-21:13 (44min)
    input! {
        n: usize,
        c: usize,
    }
    // let mut set = BTreeSet::new();
    let mut movies: Vec<Vec<(usize, usize)>> = vec![vec![]; c];
    for i in 0..n {
        input! {
            si: usize,
            ti: usize,
            ci: usize,
        }
        // set.insert(si);
        // set.insert(ti);
        movies[ci-1].push((si, ti));
    }
    // println!("set = {:?}", set);

    // 同じチャンネルで、繋がっているやつは、マージする
    let mut movies2 = vec![vec![]; c];
    for ci in 0..c {
        let mut movie = vec![];
        if movies[ci].len() == 0 {continue}
        movies[ci].sort();
        movie.push(movies[ci][0]);
        for i in 1..movies[ci].len() {
            // マージ
            if movies[ci][i-1].1 == movies[ci][i].0 {
                let leng = movie.len();
                movie[leng-1].1 = movies[ci][i].1;
            }
            else {
                movie.push(movies[ci][i]);
            }
        }
        movies2[ci] = movie;
    }

    // かさなり
    // いもす
    let mut times = vec![0; 100_002 * 2];
    // let mut times: Vec<isize> = vec![0; 93];
    
    for ci in 0..c {
        for i in 0..movies2[ci].len() {
            let si = movies2[ci][i].0;
            let ti = movies2[ci][i].1;
            times[2*si-1] += 1;
            times[2*ti+1] -= 1;
        }
    }
    for ti in 1..times.len() {
        times[ti] += times[ti-1];
    }
    // println!("times = {:?}", times);
    // for ti in 0..times.len() {
    //     println!("times[{ti}] = {:?}", times[ti]);
    // }

    let mut ans = 0;
    for ti in 0..times.len() {
        ans = max(times[ti], ans);
    }
    println!("{}", ans);


}