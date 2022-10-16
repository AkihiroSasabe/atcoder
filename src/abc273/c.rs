use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    input! {
        n: usize,
        mut a: [usize; n]
    }
    a.sort();
    a.reverse();

    // let mut hash_map = HashMap::new();

    let mut b = vec![];
    b.push(vec![a[0], 1]);

    let mut i_b = 0;
    for i in 1..n {
        if a[i-1] == a[i] {
            b[i_b][1] += 1;
        }
        else {
            b.push(vec![a[i], 1]);
            i_b += 1;
        }
    }
    for i in 0..n {
        if b.len() - 1 >= i {
            println!("{}", b[i][1]);
        }
        else {
            println!("0");
        }
    }


    // hash_map.insert(a[0], 1);
    // for i in 1..n {
    //     if hash_map.contains_key(&a[i]) {
    //         *hash_map.get_mut(&a[i]).unwrap() += 1;
    //     }
    // }





    // let mut b = vec![];
    // for i in 0..n {
    //     b.push(vec![a[i], i]);
    // }
    // // let mut b = a.clone();
    // b.sort();
    // b.reverse();

    // // k種類になるaiの数
    // let mut count = vec![0; n];

    // count[0] = 1;
    // let mut k = 0;
    // for i in 1..n {
    //     if b[i][0] == b[i-1][0] {
    //         count[k] += 1;
    //     }
    //     else {
    //         k += 1;
    //     }
    // }

    // for i in 0..n {
        
    // }


}