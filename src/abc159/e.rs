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
    // 2024-04-29 10:21-11:47 (1h26min)
    input! {
        h: usize,
        w: usize,
        k: usize,
        ss: [Chars; h]
    }
    let mut s = vec![vec![0; w]; h];
    for y in 0..h {
        for x in 0..w {
            if ss[y][x] == '1' {
                s[y][x] = 1;
            }
        }
    }
    // 10!=3_628_800
    // タテの切り方 2^(10-1) = 512 通り

    // 1 が白

    let mut cums = vec![vec![0; w]; h];
    for y in 0..h {
        for x in 0..w {
            if x == 0 {
                cums[y][x] = s[y][x];
            }
            else {
                cums[y][x] = cums[y][x-1] + s[y][x];
            }
        }
        // println!("cums[y] = {:?}", cums[y]);
    }
    let INF = 1 << 60;
    let mut ans = INF;
    for mask_y in 0_usize..1<<(h-1) {
        let num_y_cut = mask_y.count_ones() as usize;
        // println!("mask_y = {:03b}", mask_y);
        let mut merges = vec![vec![0; w]; num_y_cut+1];

        for x in 0..w {
            let mut count = 0;
            let mut temp = 0;
            for y in 0..h {
                temp += cums[y][x];
                if mask_y & (1 << y) != 0 {
                    // yで切る
                    merges[count][x] = temp;
                    temp = 0;
                    count += 1;
                }
                if y == h - 1 {
                    merges[count][x] = temp;
                }
                
            }
        }
        // println!("merges.len() = {:?}", merges.len());
        // println!("merges = {:?}", merges);
        // for y in 0..merges.len() {
        //     println!("merges[y] = {:?}", merges[y]);
        // }


        // x=0から詰んでいる場合は終了
        let mut is_break = false;
        for y in 0..merges.len() {
            if merges[y][0] > k {
                is_break = true;
                break
            }
            if w >= 2 && merges[y][w-1] - merges[y][w-2]  > k {
                is_break = true;
                break
            }
        }
        if is_break {continue}

        let mut num_x_cut = 0;
        let mut pre_cut = INF;
        for x in 1..w {
            for y in 0..merges.len() {
                if pre_cut == INF {
                    if merges[y][x] > k {
                        num_x_cut += 1;
                        pre_cut = x-1;
                        if merges[y][x] - merges[y][pre_cut] > k {
                            is_break = true;
                            break
                        }
                    }    
                }
                else {
                    if merges[y][x] - merges[y][pre_cut] > k {
                        num_x_cut += 1;
                        pre_cut = x-1;
                        if merges[y][x] - merges[y][pre_cut] > k {
                            is_break = true;
                            break
                        }
                    }
                }
            }
            if is_break {
                break
            }
        }
        if is_break {continue}
        let temp_ans = num_x_cut + num_y_cut;
        // println!("temp_ans = {:?}-------------------------", temp_ans);
        ans = min(ans, temp_ans);
    }
    println!("{}", ans);


}
