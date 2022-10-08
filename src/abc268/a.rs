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
        a: usize,
        b: usize,
        c: usize,
        d: usize,
        e: usize,
    }
    let mut list = vec![0; 200];
    list[a] = 1;
    list[b] = 1;
    list[c] = 1;
    list[d] = 1;
    list[e] = 1;

    let mut ans = 0;
    for i in 0..list.len() {
        ans += list[i];
    }
    println!("{}", ans);


    // let mut hash_map = HashMap::new();
    // hash_map.insert(a, 1);
    // hash_map.insert(b, 1);
    // hash_map.insert(c, 1);
    // hash_map.insert(d, 1);
    // hash_map.insert(e, 1);



}