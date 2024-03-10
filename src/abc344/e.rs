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
        n: usize,
        a: [usize; n],
        q: usize,
    }
    let mut k = vec![];
    let mut x = vec![];
    let mut y = vec![];
    for i in 0..q {
        input! {
            ki: usize,
        }
        k.push(ki);
        if ki == 1 {
            input!{
                xi: usize,
                yi: usize,
            }
            x.push(xi);
            y.push(yi);
        }
        else {
            input!{
                xi: usize,
            }
            x.push(xi);
            y.push(0);
        }
    }
    let mut hash = HashMap::new();

    let INF = 1 << 62;
    for i in 0..n {
        if i == 0 {
            if a.len() > 1 {
                hash.insert(a[i], [INF, a[i+1]]);
            }
            else {
                hash.insert(a[i], [INF, INF]);
            }
        }
        else if i == n-1 {
            hash.insert(a[i], [a[i-1], INF]);
        }
        else {
            hash.insert(a[i], [a[i-1], a[i+1]]);
        }
    }
    for i in 0..q {
        if k[i] == 1 {
            // 直後に挿入

            let xi = x[i];
            let yi = y[i];

            let mut now = hash.get_mut(&xi).unwrap();
            let pre_next = now[1];
            now[1] = yi;

            if pre_next != INF {
                let mut pre_ind = hash.get_mut(&pre_next).unwrap();
                pre_ind[0] = yi;
            }
            hash.insert(yi, [xi, pre_next]);
            // ケツのとき
        }
        else if k[i] == 2{
            // 消す

            // 先頭のとき注意
            let xi = x[i];
            let mut now_ind = hash.get_mut(&xi).unwrap();
            let pre = now_ind[0];
            let next = now_ind[1];

            // 先頭のとき
            if pre != INF {
                let mut pre_ind = hash.get_mut(&pre).unwrap();
                pre_ind[1] = next;
            }

            // けつのとき
            if next != INF {
                let mut next_ind = hash.get_mut(&next).unwrap();
                next_ind[0] = pre;
            }
            hash.remove(&xi).unwrap();
        }
    }
    let mut ans = vec![];
    let mut head = INF;
    for (h, ind) in hash.iter() {
        if ind[0] == INF {
            head = *h;
        }
    }
    // if head == INF {
    //     println!("");
    //     return
    // }
    ans.push(head);
    loop {
        let next = hash.get(&head).unwrap()[1];
        if next == INF {break}
        head = next;
        ans.push(head);
    }
    for i in 0..ans.len() {
        print!("{} ", ans[i]);
    }


}