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
use std::collections::{HashSet, BTreeSet};
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    input! {
        mut n: usize
    }



    let mut arr = vec![];
    let mut ans = 0;
    arr.push(n);
    let mut set = HashMap::new();
    let ans = dfs(n, &mut set);
    // println!("set = {:?}", set);
    println!("{}", ans);

    // let mut count  = 0;
    // while arr.len() > 0 {
    //     // println!("arr = {:?}, ans = {ans}", arr);
    //     let x = arr.pop().unwrap();
    //     let x0 = x / 2;
    //     let mut x1 = x / 2;
    //     if x % 2 != 0 {
    //         x1 += 1;
    //     }
    //     if x0 >= 2 {
    //         arr.push(x0);
    //     }
    //     if x1 >= 2 {
    //         arr.push(x1);
    //     }
    //     // println!("x0 = {:?}, x1 = {:?}", x0, x1);
    //     ans += x;
    // }
    // println!("{}", ans);
}

fn dfs(x: usize, set: &mut HashMap<usize, usize>) -> usize {
    if set.contains_key(&x) {
        let ans = *set.get(&x).unwrap();
        return ans
    }
    if x == 2 {
        return 2
    }
    else if x < 2 {
        return 0
    }

    let x0 = x / 2;
    let mut x1 = x / 2;
    if x % 2 != 0 {
        x1 += 1;
    }
    let y0 = dfs(x0, set);
    let y1 = dfs(x1, set);

    set.insert(x, x + y0 + y1);    

    return x + y0 + y1
}