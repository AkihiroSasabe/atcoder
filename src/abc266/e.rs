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
        n: usize
    }

    // expect[i]には、iターン目にちょうどゲームが終了するときの期待値が格納されている。
    let mut expect: Vec<f64> = vec![0.0; n+1];
    expect[1] = 3.5;
    for i in 2..(n+1) {
        // .ceil()は切り上げ。
        let x = expect[i-1].ceil() as usize;
        let mut over = 0.0;
        for j in x..7 {
            over += (j as f64 / 6.0);
        }
        
        let mut under_x_probability = (x as f64 - 1.0 ) / 6.0;

        expect[i] = over + under_x_probability * expect[i-1];

    }
    println!("{}", expect[n]);
}