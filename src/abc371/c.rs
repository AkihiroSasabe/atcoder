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
    input! {
        n: usize,
        mg: usize,
    }
    // let mut g = vec![vec![]; n];
    // let mut g = vec![BTreeSet::new(); n];
    let mut g = vec![vec![0; n]; n];
    for i in 0..mg {
        input! {
            ui: usize,
            vi: usize,
        }
        // g[ui-1].push(vi-1);
        // g[vi-1].push(ui-1);

        g[ui-1][vi-1] += 1;
        g[vi-1][ui-1] += 1;
        // g[ui-1].insert(vi-1);
        // g[vi-1].insert(ui-1);

    }
    input! {
        mh: usize,
    }
    let mut h = vec![vec![0; n]; n];
    
    // let mut h = vec![vec![]; n];
    // let mut h = vec![BTreeSet::new(); n];
    for i in 0..mh {
        input! {
            ai: usize,
            bi: usize,
        }
        h[ai-1][bi-1] += 1;
        h[bi-1][ai-1] += 1;
        // h[ai-1].push(bi-1);
        // h[bi-1].push(ai-1);
        // h[ai-1].insert(bi-1);
        // h[bi-1].insert(ai-1);
    }

    let mut a = vec![];
    // println!("a = {:?}", a);
    for i in 0..n-1 {
        input! {
            ai: [usize; n-i-1]
        }
        a.push(ai);
    }
    // println!("a = {:?}", a);

    let mut ans = 1 << 60;
    for perm in (0..n).permutations(n) {
        let mut temp = 0;
        for i in 0..n {
            for j in i+1..n {
                let ui = min(perm[i], perm[j]);
                let vi = max(perm[i], perm[j]);
                if g[i][j] != h[ui][vi] {
                    // println!("ui = {}, vi = {:?}", ui, vi);
                    temp += a[ui][vi-ui-1];
                }
            }
        }
        ans = min(ans, temp);
    }
    println!("{}", ans);

    

    // // 同型か判定
    // for mask in 0..1<< (n*(n-1)/2) {

    //     let mut h2 = h.clone();
        
    //     for perm in (0..n).permutations(n) {
            
    //     }
    // }

    // 2_68_435_456

    // 6_289_920

}

fn is_dokei(g: &Vec<BTreeSet<usize>>, h: &Vec<BTreeSet<usize>>, perm: Vec<usize>, n: usize) -> bool {

    for v in 0..n {
        if g[v].len() != h[v].len() {
            return false
        }
        for hv in h[v].iter() {
            let hv2 = perm[*hv];
            if !g[v].contains(&hv2) {
                return false
            }
        }
    }
    return true
}