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
    let mut s = vec![];
    for _ in 0..n {
        input! {
            s_i: String
        }
        s.push(s_i);
    }

    // println!("==========-----");
    let mut hash_map = HashMap::new();
    for i in 0..n {
        // 初
        if !hash_map.contains_key(&s[i]) {
            println!("{}", s[i]);
            hash_map.insert(&s[i], 1);
        }
        else {
            println!("{}({})", s[i], hash_map[&s[i]]);
            *hash_map.get_mut(&s[i]).unwrap() += 1;
        }
        // println!("{}", s[i]);

    }


}