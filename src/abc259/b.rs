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
        a: f64,
        b: f64,
        d: f64
    }


    let x: f64 = a * (d / 180.0 * PI).cos() - b * (d / 180.0 * PI).sin();
    let y: f64 = a * (d / 180.0 * PI).sin() + b * (d / 180.0 * PI).cos();

    println!("{} {}", x, y);

}