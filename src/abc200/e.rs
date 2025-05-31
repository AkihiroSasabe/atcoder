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
    // 2025-05-28 20:15-21:05 (50min)
    // 2025-05-30 18:59-19:55 (56min)
    // 106min
    input! {
        n: usize,
        k: usize,
    }
    let mut cum = 0;

    let mut sum = 0;
    for s in 3..3*n+1 {
        let diff = if s <= n {
            // 和がSで、1<=x, 1<=y, 1<=z なときの、(x,y,z)の組み合わせの総数 (S-3個のボールに、仕切りを2個追加する問題に帰着可能)
            // (s-(1+1+1)+2)_C_2 = (s-1)_C_2
            (s-1) * (s-2) / 2
        } else {
            // 和がSで、1<=x, 1<=y, 1<=z なときの、(x,y,z)の組み合わせの総数 (S-3個のボールに、仕切りを2個追加する問題に帰着可能)
            // (s-(1+1+1)+2)_C_2 = (s-1)_C_2
            let s0 = (s-1) * (s-2) / 2;

            // 和がSで、n+1<=x, 1<=y, 1<=z なときの、(x,y,z)の組み合わせの総数 (S-n-3個のボールに、仕切りを2個追加する問題に帰着可能)
            // (s-((n+1)+1+1)+2)_C_2 = (s-n-1)_C_2
            let s1 = if (s > n + 2) {(s-n-1) * (s-n-2) / 2} else {0};

            // 和がSで、n+1<=x, n+1<=y, 1<=z なときの、(x,y,z)の組み合わせの総数 (S-2n-3個のボールに、仕切りを2個追加する問題に帰着可能)
            // (s-((n+1)+(n+1)+1)+2)_C_2 = (s-2n-1)_C_2
            let s2 = if (s > 2*n + 2) {(s-2*n-1) * (s-2*n-2) / 2} else {0};

            // 和がSで、n+1<=x, n+1<=y, n+1<=z なときの、(x,y,z)の組み合わせの総数 (S-3n-3個のボールに、仕切りを2個追加する問題に帰着可能)
            // (s-((n+1)+(n+1)+(n+1))+2)_C_2 = (s-3n-1)_C_2
            let s3 = if (s > 3*n + 2) {(s-3*n-1) * (s-3*n-2) / 2} else {0};

            let s_subtract = 3 * s1 - 3 * s2 + s3;
            s0 - s_subtract
        };
        // println!("(s, diff) = {:?}", (s, diff));
        cum += diff;
        if k <= cum {
            // x+y+z=s 平面上にあるパラメータが答え。
            sum = s;
            cum -= diff;
            break;
        }
    }
    // println!("sum = {:?}", sum);

    for x in 1..n+1 {
        let r = sum - x;
        if r > 2*n {continue}
        if r < 2 {continue} // ここは来ないけど。
        let y_min = if r <= n + 1 {
            1
        } else {
            r - n
        };
        let y_max = r - y_min;
        let diff = y_max + 1 - y_min;

        // let diff = if r <= n {sum - x - 1} else {2*n-r+1};
        cum += diff;
        if k <= cum {
            cum -= diff;
            let y = y_min + k - cum - 1;
            let z = sum-x-y;
            println!("{} {} {}", x, y, z);
            return;
        }
    }

}