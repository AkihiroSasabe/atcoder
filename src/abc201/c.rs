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
        s: Chars
    }
    let mut count_o = 0;
    let mut count_x = 0;
    let mut count_h = 0;

    for si in s {
        if si == 'o' {
            count_o += 1;
        }
        else if si == 'x' {
            count_x += 1;
        }
        else {
            count_h += 1;
        }
    }

    // 全部はてなだったときは?

    if count_o >= 5 {
        // ユニークな数が5個以上
        println!("0");
        return;
    }
    if count_x == 10 {
        // 全部x
        println!("0");
        return;
    }

    let mut ans = 0;
    for num_unique in count_o..1+min(4, count_o + count_h) {
        let diff = match num_unique {
            1 => comb(count_h, 1-count_o),
            2 => (2*2*2*2-2) * comb(count_h, (2-count_o)), // 2色の順列 x ユニークなやつを選出する方法
            3 => 3 * 2 * comb(4,2) * comb(count_h, (3-count_o)), // 2個あるやつを3個の中から1個選ぶ(3通り) x 4箇所から2箇所選ぶ(4C2) x 残りの2個の順列(2通り)
            4 => (4*3*2*1) * comb(count_h, (4-count_o)), // 4色の順列 x ユニークなやつを選出する方法(4! * h_C_(4-o))
            _ => 0
        };
        // println!("num_unique = {num_unique}, diff = {:?}", diff);

        ans += diff;
    }
    println!("{}", ans);

}


fn comb(n: usize, r: usize) -> usize {
    // nCr
    let mut top = 1;
    for i in 0..r {
        top *= (n-i);
    }
    let mut bottom = 1;
    for i in 1..r+1 {
        bottom *= i;
    }
    return top / bottom
}