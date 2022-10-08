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
    }
    let mut graph = vec![vec![false; n]; n];
    for i in 0..m {
        input! {
            u_i: usize,
            v_i: usize,
        }
        graph[u_i - 1][v_i - 1] = true;
        graph[v_i - 1][u_i - 1] = true;
        // graph[u_i - 1].push(v_i - 1);
        // graph[v_i - 1].push(u_i - 1);
    }

    let mut ans = 0;

    for i in 0..n {
        for j in (i+1)..n {
            for k in (j+1)..n {
                // a-b
                if graph[i][j] && graph[j][k] && graph[k][i] {
                    ans += 1;
                    // dbg!(i, j, k);
                }
            }
        }
    }
    println!("{}", ans);



}