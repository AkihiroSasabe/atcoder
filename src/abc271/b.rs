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
use superslice::*;
fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut a = vec![];
    for i in 0..n {
        input! {
            l_i: usize,
        }
        input! {
            a_i: [usize; l_i]
        }
        a.push(a_i);
    }
    let mut s = vec![];
    let mut t = vec![];
    for i in 0..q {
        input! {
            s_i: usize,
            t_i: usize,
        }
        s.push(s_i);
        t.push(t_i);
    }

    for i in 0..q {
        println!("{}", a[s[i] - 1][t[i] - 1]);
    }


}