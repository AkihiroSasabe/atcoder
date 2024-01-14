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
    input! {
        n: usize,
        q: usize,
    }
    let mut kinds = vec![];
    let mut vals = vec![];
    for i in 0..q {
        input! {
            kind: usize,
        }
        kinds.push(kind);
        if kind == 1 {
            // 方向
            input! {
                cii: char
            }
            let ci: usize = match cii {
                'R' => 0,
                'L' => 1,
                'U' => 2,
                _ => 3
            };
            vals.push(ci);
        }
        else if kind == 2 {
            // print
            input! {
                pii: usize
            }
            let pi = pii - 1;
            vals.push(pi);
        }
    }
    let mut que = VecDeque::new();
    for i in 0..n {
        que.push_back([i as isize+1, 0]);
    }
    let dir_x = vec![1,-1,0,0];
    let dir_y = vec![0,0,1,-1];
    for i in 0..q{
        if kinds[i] == 1 {
            let d = vals[i];
            let mut q0 = que.pop_front().unwrap();
            let mut q1 = q0.clone();
            que.push_front(q0);
            q1[0] += dir_x[d];
            q1[1] += dir_y[d];
            que.push_front(q1);
            que.pop_back();
        }
        else {
            let v: usize = vals[i];
            let mut q0: &[isize; 2] = que.get(v).unwrap();
            println!("{} {}", q0[0], q0[1]);
        }
    }



    // // 現在の値(x, y, 次のindex)
    // let mut renketu = vec![];

    // // パーツiは、連結リストの何番目にいるか? taio[i] = index
    // let mut taio = vec![0; n];

    // // つぎがxなやつを誰が持っているか?
    // let mut tugi = vec![0; n];
    // for i in 0..n {
    //     renketu.push([i as isize + 1, 0 as isize, i as isize +1]);
    //     taio[i] = i;
    //     if i+1 == n {continue}
    //     tugi[i+1] = i;
    // }
    // let INF = 1 << 60;
    // renketu[n-1][2] = INF;

    // let dir_x = vec![1,-1,0,0];
    // let dir_y = vec![0,0,1,-1];

    // let mut next_ind = 0;
    // for i in 0..q {
    //     if kinds[i] == 1 {

    //         let d = vals[i];
            
            
            
    //         let next = renketu.len();
    //         // v == 0で注意
    //         let ind = taio[v-1];
    //         renketu[ind][2] = next as isize;
    //         renketu[ind][2] = next as isize;

    //         let ind2 = taio[v];
    //         renketu[ind2][0]
    //         renketu.push();
    //     }
    //     else {
    //         let ind = taio[vals[i]];
    //         let x = renketu[ind][0];
    //         let y =  renketu[ind][1];
    //         println!("{} {}", x, y);
    //     }
    // }






}