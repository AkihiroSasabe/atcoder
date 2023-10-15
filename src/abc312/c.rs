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
        n: usize,
        m: usize,
        mut a: [usize; n],
        mut b: [usize; m],
    }

    // let mut ab = vec![];
    let mut ab_tree: BTreeMap<usize, Vec<usize>> = BTreeMap::new();

    for i in 0..n {
        if ab_tree.contains_key(&a[i]) {
            ab_tree.get_mut(&a[i]).unwrap()[0] += 1;
        }
        else {
            ab_tree.insert(a[i], vec![1, 0]);
        }
        // ab.push(vec![a[i], 0]);
    }
    for i in 0..m {
        if ab_tree.contains_key(&(b[i] + 1)) {
            ab_tree.get_mut(&(b[i] + 1)).unwrap()[1] += 1;
        }
        else {
            ab_tree.insert(b[i] + 1, vec![0, 1]);
        }
        // ab.push(vec![b[i], 1]);
    }
    // ab.sort();

    let mut a_num = 0;
    let mut b_num = m;
    for (k, v) in ab_tree.iter() {
        a_num += v[0];
        b_num -= v[1];
        if a_num >= b_num {
            println!("{}", k);
            return
        }
    }
    // for i in 0..n+m {
    //     if ab[i][1] == 0 {
    //         a_num += 1;
    //     }
    //     else {
    //         b_num -= 1;
    //     }

    //     if a_num >= b_num {
    //         println!("{}", ab[i][0]);
    //         return
    //     }
    // }


}


// // B木に要素xを1個追加する関数
// fn add_to_btree<T: std::cmp::Ord>(b_tree: &mut BTreeMap<T, usize>, x: T) {
//     // b_tree := {key: 要素, value: 格納した要素の個数}
//     if b_tree.contains_key(&x) {
//         *b_tree.get_mut(&x).unwrap() += 1;
//     }
//     else {
//         b_tree.insert(x, 1);
//     }
// }
