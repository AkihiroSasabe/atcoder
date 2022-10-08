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
        k: usize,
        q: usize,
    }

    let mut a = vec![];
    for i in 0..k {
        input! {
            a_i: usize
        }
        a.push(a_i - 1);
    }
    input! {
        l: [usize; q]
    }

    for i in 0..q {
        if a[l[i] - 1] == n - 1 {continue}
        if l[i] == k {
            a[l[i] - 1] += 1;
        }
        else {
            if a[l[i]] == a[l[i] - 1] + 1 {continue}
            a[l[i] - 1] += 1;
        }

    }

    for i in 0..k {
        print!("{} ", a[i] + 1);
    }
}