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
        s: Chars
    }
    let uppercase: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    let mut flag = true;
    if s.len() != 8 {
        flag = false
    }
    let mut first_flag = false;
    for i in 0..uppercase.len() {
        if s[0] == uppercase[i] {
            first_flag = true;
        }
    }
    if !first_flag {
        flag = false;
    }
    let mut final_flag = false;
    if flag {
        for i in 0..uppercase.len() {
            if s[7] == uppercase[i] {
                final_flag = true;
            }
        }    
    }
    if !final_flag {
        flag = false;
    }

    let num_case: Vec<char> = "1234567890".chars().collect();
    if flag {
        let mut n_flag = false;
        for i in 1..7 {
            n_flag = false;
            for j in 0..num_case.len() {
                if s[i] == num_case[j] {
                    n_flag = true;
                    break
                }
            }
            if !n_flag {
                break
            }
        }
        if !n_flag {
            flag = false;
        }
    }
    if flag && s[1] != '0' {
        println!("Yes");
    }
    else {
        println!("No");
    }


    


    
}