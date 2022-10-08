use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use proconio::marker::Chars;
use std::f64::consts::PI;
fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut p = 0;
    let mut masume = vec![0; 4];
    for i in 0..n {
        masume[0] += 1;
        let mut next_masume = vec![0; 4];
        for j in 0..4 {
            if j + a[i] >= 4 {
                p += masume[j];
            }
            else {
                next_masume[j + a[i]] = masume[j];
            }
        }
        masume = next_masume.clone();
    }
    println!("{}", p);
}