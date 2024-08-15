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
    // 2024-08-15 09:51-10:12 (21min)
    // 
    // 【XOR, 二分探索（尺取法）】
    // a[L] xor a[L+1] xor ... xor a[R] = a[L] + a[L+1] + ... + a[R] となる、
    // [L,R]の組み合わせの個数を答える問題。
    // 等号不成立するとき、特定のbitが1になっている個数が2以上になる。
    // Lを固定すると、R=Lのときは必ず等号成立、Rが大きくなっていくと単調増加で厳しくなるので、
    // 2分探索でその境界を見つけるだけ。
    // 尺取法の実装は自分は苦手だけど、多くの場合は、二分探索で誤魔化せることが多そう。
    input! {
        n: usize,
        a: [usize; n]
    }
    // DPの匂いがする

    // 桁が被っていない範囲を探せば良い
    // その桁が1であるとまずい。
    // 尺取か?

    // 各ビットの個数を全部みていく
    // 二分探索していく
    let mut cum: Vec<Vec<usize>> = vec![vec![0; 20]; n];
    for i in 0..n {
        for j in 0..20 {
            if a[i] & (1 << j) != 0 {
                cum[i][j] += 1;
            }
        }
    }
    for i in 1..n {
        for j in 0..20 {
            cum[i][j] += cum[i-1][j];
        }
    }

    let mut ans = 0;
    for le in 0..n {

        
        // めぐる式二分探索
        let mut ok = le;
        let mut ng = n-1;

        if judge(ng, le, &cum) {
            ok = ng;
        }

        while (ng as i128 - ok as i128).abs() > 1 {
            let mid = (ng + ok) / 2;
            let is_ok = judge(mid, le, &cum);
            if is_ok {
                ok = mid;
            }
            else {
                ng = mid;
            }
        }
        ans += 1 + ok - le;
    }
    println!("{}", ans);
}

fn judge(mid: usize, le: usize, cum: &Vec<Vec<usize>>) -> bool {

    for i in 0..20 {
        if le != 0 {
            if cum[mid][i] - cum[le-1][i] > 1 {return false}
        }
        else {
            if cum[mid][i] > 1 {return false}
        }
    }
    return true
}