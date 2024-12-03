#![allow(dead_code, unused_imports)]
use proconio::{input, marker::{Usize1, Isize1}};
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
        m: usize,
        a: [usize; n], // 人
        b: [usize; m], // すし
    }

    let INF = 1 << 60;
    let mut mins = vec![INF; n];
    mins[0] = a[0];
    for i in 1..n {
        mins[i] = min(mins[i-1], a[i]);
    }

    // めぐる式二分探索

    // 関数じゃなくて、クロージャーを使うと、引数を少なく出来る。
    let judge  = |mid: usize, susi: usize| -> bool {
        return mins[mid] <= susi
    };


    // fn judge(mid: usize, susi: usize, mins: &Vec<usize>) -> bool {
    //     return mins[mid] <= susi
    // }

    // println!("mins = {:?}", mins);


    for i in 0..m {
        let mut ng = 0;
        let mut ok = n-1;
        let susi = b[i];
        if judge(ng, susi) {
            ok = ng;
        }
        if !judge(ok, susi) {
            println!("-1");
            continue
        }
        
        while (ng as i128 - ok as i128).abs() > 1 {
            let mid = (ng + ok) / 2;
            let is_ok = judge(mid, susi);
            if is_ok {
                ok = mid;
            }
            else {
                ng = mid;
            }
        }
        println!("{}", ok + 1);
    }





}

