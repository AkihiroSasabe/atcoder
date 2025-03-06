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
    // 2025-03-05 20:10-20:50 (40min)
    input! {
        n: usize,
        se: [Chars;n]
    }

    // let mut s = vec![];
    // let mut e = vec![];
    let mut starts = vec![vec![]; 24*60+1];
    let mut ends = vec![vec![]; 24*60+1];
    for i in 0..n {

        let sh = cast(se[i][0]) * 10 + cast(se[i][1]);
        let mut sm = cast(se[i][2]) * 10 + cast(se[i][3]);
        sm = sm + sh * 60;
        sm = (sm / 5) * 5;

        let eh = cast(se[i][5]) * 10 + cast(se[i][6]);
        let mut em = cast(se[i][7]) * 10 + cast(se[i][8]);
        em = em + eh * 60;
        if em % 5 != 0 {
            em = (em / 5 + 1) * 5;            
        }
        // s.push(sm);
        // e.push(em);
        // println!("sm = {}, em = {:?}", sm % 60, em % 60);
        starts[sm].push(i);
        ends[em].push(i);
    }
    // println!("starts = {:?}", starts);
    // println!("ends = {:?}", ends);

    let mut set = BTreeSet::new();
    let mut is_rains = vec![false; 24*60+1];
    for m in 0..24*60+1 {
        while let Some(ind) = starts[m].pop() {
            set.insert(ind);
        }
        while let Some(ind) = ends[m].pop() {
            set.remove(&ind);
        }
        is_rains[m] =  set.len() != 0;
    }

    let mut sss = vec![];
    let mut eee = vec![];
    let mut is_rain = false;
    let mut pre_start = 0;
    for m in 0..24*60+1 {
        if is_rain {
            if !is_rains[m] {
                // 雨が止む
                sss.push(pre_start);
                eee.push(m);
                is_rain = false;
            } 
        }
        else {
            // 晴れ
            if is_rains[m] {
                pre_start = m;
                is_rain = true;
            }
        }
    }
    for i in 0..sss.len() {
        let sh = sss[i] / 60;
        let sm = sss[i] % 60;

        let eh = eee[i] / 60;
        let em = eee[i] % 60;
        println!("{:02}{:02}-{:02}{:02}", sh, sm, eh, em);
    }
}

fn cast(ch: char) -> usize {
    ch as usize - '0' as usize
}