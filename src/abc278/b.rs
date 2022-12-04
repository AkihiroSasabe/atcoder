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
        mut h: usize,
        mut m: usize
    }
 
    loop {
        let a = h / 10;
        let b = h % 10;
        let c = m / 10;
        let d = m % 10;
 
        // 右上 c
        // 左下 b
        // 入れ替え後の時刻
        let h_new = a * 10 + c;
        let m_new = b * 10 + d;
        // println!("h_new {}, m_new {}", h_new, m_new);
        if 0 <= h_new && h_new < 24 && 0 <= m_new && m_new < 60 {
            println!("{} {}", h, m);
            break;
        } else {
            if m != 59 {
                m += 1;
            } else {
                m = 0;
                if h == 23 {
                    h = 0;
                } else {
                    h += 1;
                }
            }
        }
    }
}
