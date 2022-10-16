use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    input! {
        // mut x: Chars,
        mut x: usize,
        k: usize,
    }

    let mut waru = 1;
    for i in 0..k {
        let mut xx = x.clone();
        xx /= waru;
        let digit = xx % 10;
        if digit <= 4 {
            x -= digit * waru;
        }
        else {
            x -= digit * waru;
            x += 1 * 10 * waru;
        }
        waru *= 10;
    }

    println!("{}", x);

    // for i in 0..k {
    //     if x.len() >= i {
    //         continue
    //     }
    //     if x[k-1-i] as i32 - 48 <= 4 {
    //         x[k-1-i] = '0';
    //     }
    //     else {
    //         x[k-1-i] = '0';
    //         x[k-i] = x[k-i] as i32 - 48 + 1
    //     }
    // }



}