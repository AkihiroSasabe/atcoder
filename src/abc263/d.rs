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
        n: usize,
        l: isize,
        r: isize,
        mut a: [isize; n]
    }
    let mut left_sum = cummulative_sum(&a);

    let mut ans = left_sum[n-1];
    let mut min_x = n;
    let mut min_y = n;
    let mut x = 0;
    let mut y = n-1;
    loop {
        if l <= a[x] {
            min_x = x;
            x += 1;
        }
        if r <= a[y] {
            min_y = y;
            y -= 1;
        }
    }
    

    let mut x = 0;
    let mut y = n-1;

    // for x in 0..n {
    //     let sum = left_sum[n-1] - left_sum[x] + (x+1) as isize * l;
    //     if sum < ans {
    //         min_x = x;
    //         ans = sum;
    //     }
    // }
    // if min_x != n {
    //     for i in 0..min_x+1 {
    //         a[i] = l;
    //     }
    // }
    // let mut left_sum = cummulative_sum(&a);
    // let mut min_y = n;
    // for y in 0..n {
    //     let sum = left_sum[y] + (n - 1 - y) as isize * r;
    //     if sum < ans {
    //         ans = sum;
    //     }
    // }
    // ans = min(ans, n as isize * r);
    println!("{}", ans);

}

fn cummulative_sum(a: &Vec<isize>) -> Vec<isize> {
    let mut left_sum = vec![0; a.len()];
    left_sum[0] = a[0];
    for i in 1..a.len() {
        left_sum[i] = left_sum[i-1] + a[i];
    }
    return left_sum
}