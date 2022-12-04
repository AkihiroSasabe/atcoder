#![allow(dead_code, unused_imports)]
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
        n: usize,
        s: [Chars; n]
    }
    // let one = vec!["H", "D"]
    let two = vec!['A' ,'2' ,'3' ,'4' ,'5' ,'6' ,'7' ,'8' ,'9' ,'T' ,'J' ,'Q' ,'K'];
    let mut flag = true;
    for i in 0..n {
        if !(s[i][0] == 'H' || s[i][0] == 'D' || s[i][0] == 'C' || s[i][0] == 'S') {
            flag = false;
        }
        let mut temp_flag = false;
        for j in 0..two.len() {
            if s[i][1] == two[j] {
                temp_flag = true;
            }
        }
        if !temp_flag {
            flag = false;
        }
        for j in (i+1)..n {
            if s[i][0] == s[j][0] && s[i][1] == s[j][1] {
                flag = false;
            }
        }
    }
    if flag {
        println!("Yes");
    }
    else {
        println!("No");
    }

}