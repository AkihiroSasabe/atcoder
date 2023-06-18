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
use std::collections::HashSet;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    input! {
        p: char,
        q: char,
    }
    // println!("{} {}", p, q);
    let mut hash = HashMap::new();
    hash.insert('A', 0_isize);
    hash.insert('B', 3);
    hash.insert('C', 4);
    hash.insert('D', 8);
    hash.insert('E', 9);
    hash.insert('F', 14);
    hash.insert('G', 23);
    println!("{}", (hash.get(&p).unwrap() - hash.get(&q).unwrap()).abs());

    // おまけ: char型は数字に変換できる。char型からアルファベットのindex(番号)を取得したければ、char型をisizeにcastして'A'からの距離を求めればいい。
    // println!("B-A={}", 'B' as isize - 'A' as isize); // 1
    // println!("A-A={}", 'A' as isize - 'A' as isize); // 0
    // println!("E-A={}", 'E' as isize - 'A' as isize); // 4
    // println!("A={}", 'A' as isize); // 65
    // println!("a={}", 'a' as isize); // 97
    // println!("A-B={}", 'A' as isize - 'B' as isize); // -1
    // println!("a-b={}", 'a' as isize - 'b' as isize); // -1
}