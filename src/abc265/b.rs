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
        m: usize,
        mut t: isize,
        a: [isize; n-1],
        mut xy: [[isize; 2]; m]
    }
    xy.sort();

    let mut flag = true;
    let mut bonus_index = 0;
    for i in 0..(n-1) {
        t -= a[i];
        // dbg!(t);
        if t <= 0 {
            flag = false;
            break
        }
        if bonus_index == m {continue}
        if xy[bonus_index][0] == i as isize + 2 {
            t += xy[bonus_index][1];
            // println!("t up :{}", t);
            bonus_index += 1;
        }
    }

    if flag {
        println!("Yes");
    }
    else {
        println!("No");
    }









    // let mut x = vec![];
    // let mut y = vec![];
    // for i in 0..m {
    //     input! {
    //         x_i: usize,
    //         y_i: usize,
    //     }
    //     x.push(x_i);
    //     y.push(y_i);
    // }




}