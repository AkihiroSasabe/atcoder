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
        a: [usize; n]
    }

    let mut index_value_same_num = 0;
    let mut swap_num = 0;
    for i in 0..n {
        if i + 1 == a[i] {
            index_value_same_num += 1;
            continue
        }
        if a[a[i] - 1] == i + 1 {
            swap_num += 1;
        }
    }
    // dbg!(index_value_same_num, swap_num);
    let ans = index_value_same_num as isize * (index_value_same_num as isize - 1) / 2 + swap_num as isize / 2;
    println!("{}", ans);


}