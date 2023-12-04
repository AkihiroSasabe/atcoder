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
    // 2023-12-04 19:59-20:40 (41min)
    input! {
        n: usize,
        c: [usize; 9]
    }

    let c_min = *c.iter().min().unwrap();
    // println!("c_min = {:?}", c_min);
    let max_keta = n / c_min;

    let mut c_min_ind = 0;
    for i in (0..9).rev() {
        if c[i] == c_min {
            c_min_ind = i;
            break
        }
    }
    // println!("c_min_ind = {:?}", c_min_ind);

    let mut x = vec![c_min_ind + 1; max_keta];
    // println!("init x = {:?}", x);
    let min_cost = max_keta * c_min;
    let mut amari = n - min_cost;

    // あとは貪欲に1ビットずつやっていけばいいかと。
    for i in 0..max_keta {
        for j in (0..9).rev() {
            if c[j] == c_min {break}
            if c[j] - c_min <= amari {
                x[i] = j + 1;
                amari -= c[j] - c_min;
                break
            }
        }
    }
    // println!("x = {:?}", x);

    if max_keta == 0 {
        println!("0");
        return;
    }
    let mut first_flag = true;
    for ans in x {
        if ans == 0 {
            if first_flag {
                continue
            }
        }
        else {
            first_flag = false;
        }
        print!("{}", ans);
    }

}