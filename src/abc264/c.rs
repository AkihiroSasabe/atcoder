use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
fn main() {
    input! {
        h1: usize,
        w1: usize,
        a: [[usize; w1]; h1],
        h2: usize,
        w2: usize,
        b: [[usize; w2]; h2],
    }

    let mut hb = 0;
    let mut wb = 0;
    let mut match_x = vec![];
    // let mut break_flag = false;
    let mut candidate_x = vec![];

    for i in 0..h1 {
        wb = 0;
        match_x = vec![];
        for j in 0..w1 {
            if a[i][j] == b[hb][wb] {
                // hb += 1;
                wb += 1;
                match_x.push(j);
                if wb == w2 {
                    candidate_x.push(match_x);
                    break
                }
            }
        }
    }
    // println!("{:?}", candidate_x);

    let mut answer_flag = false;
    let mut continue_flag = false;

    for i in 0..candidate_x.len() {
        let mut y_b = 0;
        
        for y_a in 0..h1 {
            continue_flag = false;
            for (x_b, x_a) in candidate_x[i].iter().enumerate() {
                if a[y_a][*x_a] != b[y_b][x_b] {

                    continue_flag = true;
                    break
                }
            }
            if continue_flag {continue}
            y_b += 1;
            if y_b == h2 {
                answer_flag = true;
                break
            }
        }
    }

    if answer_flag {
        println!("Yes");
    }
    else {
        println!("No");
    }


}