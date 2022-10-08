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
    input! {
        n: usize,
        k: usize,
    }
    let mut a = vec![];
    let mut b = vec![vec![]; k];
    for i in 0..n {
        input! {
            a_i: usize,
        }
        a.push(a_i);
        b[i%k].push(a_i);
    }
    a.sort();

    // 計算量O(Nlog(N))
    for i in 0..k {
        b[i].sort();
    }

    for i in 0..n {
        // removeを使うとメチャクチャ遅くなるのでNG。RustのVecはケツは得意だが、頭の挿入・取り出しは不得手
        // if a[i] != c[i%k].remove(0) {
        if a[i] != b[i%k][i/k] {
            println!("No");
            return
        }
    }

    println!("Yes");

}