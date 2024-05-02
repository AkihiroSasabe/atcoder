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
    // 10min
    input! {
        n: usize,
        x0: f64,
        y0: f64,
        xn: f64,
        yn: f64,
    }
    // n % 2 == 0;
    let mut xc = (x0 + xn) as f64 / 2.0;
    let mut yc = (y0 + yn) as f64 / 2.0;
    let r = (((xc - x0) * (xc - x0) + (yc - y0) * (yc - y0)) as f64).sqrt();
    let x = (x0 as f64- xc);
    let y = (y0 as f64- yc);
    let angle = PI * 2.0 / n as f64;
    let x1 = angle.cos() * x - angle.sin() * y + xc;
    let y1 = angle.sin() * x + angle.cos() * y + yc;
    println!("{} {}", x1, y1);

}