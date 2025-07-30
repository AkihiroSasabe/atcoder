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
        mut k: usize,
    }

    let mut dp = vec![vec![0; 10]; k+1];
    let mut count = 0;
    let mut cum =vec![ vec![0; 10]; k+1];
    for i in 0..10 {
        dp[0][i] = 1;
        if i != 0 {
            count += 1;
            cum[0][i] = count;
        }
        if count == k {
            println!("{}", i);
            return;
        }
    }

    let mut keta = 1;
    let mut top = 1;
    let mut is_break = false;
    for i in 1..k {
        for head in 0..10 {
            if head != 0 {
                dp[i][head] += dp[i-1][head-1];
            }            
            dp[i][head] += dp[i-1][head];
            if head != 9 {
                dp[i][head] += dp[i-1][head+1];
            }
            if head != 0 {
                count += dp[i][head];
            }
            if head != 0 {
                cum[i][head] = cum[i][head-1] + count;
            }
            if count >= k {
                count = count - k;
                keta = i;
                top = head;
                print!("{}", head);
                is_break = true;
                break;
            }
        }
        if is_break {
            break;
        }
    }

    // debug
    // for i in 0..keta+1 {
    //     println!("dp[{i}] = {:?}", dp[i]);
    // }
    
    // println!("(keta, top) = {:?}", (keta, top));
    // println!("count = {:?}", count);
    let mut pre_head = top;

    // for head in 1..top {
    //     count -= dp[keta][head];
    // }

    for i in (0..keta).rev() {
        let mut heads = vec![];
        if pre_head != 9 {
            heads.push(pre_head+1);
        }
        heads.push(pre_head);
        if pre_head != 0 {
            heads.push(pre_head-1);
        }

        for head in heads {
            // println!("(i, head, count) = {:?}", (i, head, count));
            if count >= dp[i][head] {
                count -= dp[i][head];
            }
            else if count < dp[i][head] {
                print!("{}", head);
                pre_head = head;
                break
            }
            // else if count == dp[i][head] {
            //     count -= dp[i][head];
            //     pre_head = head;
            //     print!("{}", head);
            //     break
            // }

        }
    }




}