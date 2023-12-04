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
    // 2023-12-03 9:41-10:03 (22min)
    input! {
        n: usize,
        m: usize,
        l: usize,
        a: [usize; n],
        b: [usize; m],
        cd: [[usize; 2]; l]
    }

    let mut b_sort = vec![];
    for i in 0..m {
        b_sort.push((b[i], i));
    }
    b_sort.sort();
    b_sort.reverse();
    
    let mut graph = vec![HashSet::new(); n];

    for i in 0..l {
        let ci = cd[i][0] - 1;
        let di = cd[i][1] - 1;
        graph[ci].insert(di);
    }
    // println!("graph = {:?}", graph);

    let mut ans = 0;
    for i in 0..n {
        let a_p = a[i];
        let a_ind = i;

        for j in 0..m {
            let b_p = b_sort[j].0;
            let b_ind = b_sort[j].1;
            if graph[a_ind].contains(&b_ind) {continue}
            ans = max(ans, a_p + b_p);
            break
        }
    }
    println!("{}", ans);
    

}