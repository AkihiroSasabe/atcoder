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
    // 2024-08-29 20:49-21:40 (51min)
    // 2024-08-30 20:16-20:52 (36min, 解説ac https://nitrone7.hatenablog.com/entry/ABC083_D )
    // total: 87min
    input! {
        s: Chars
    }

    // https://nitrone7.hatenablog.com/entry/ABC083_D 参考
    // 「全部白（全部黒）」という状態
    // <=> 「各隣り合うコマにおいて色の異なっているものが 1 つもない」
    // <=> ，条件を達成する手順は、「隣り合うコマで色の異なっている部分を flip によって同じ色にする手順」
    // たとえば長さがピッタリ K=6 の区間で flip する場合は，以下の図で仕切りを入れた前後の色の異なりを操作によって解消することができます

    // | の前後で色の反転が可能
    // K=6
    // ○|○○○○○|○
    // K=5
    // 01 234 56
    // ○○|○○○|○○
    // K>=5
    // ○|○|○○○|○|○

    // つまり、仕切りの内側に、色の反転が無ければ、そのKは合格
    // Kについて、決め打ち二分探索

    // めぐる式二分探索
    fn judge(mid: usize, s: &Vec<char>) -> bool {
        if mid == 1 {
            return true
        }

        let n = s.len();
        let left = n - mid;
        let right = mid - 2;
        // println!("mid = {:?}", mid);
        // println!("left = {}, right = {:?}", left, right);
        for i in left..right+1 {
            if s[i] != s[i+1] {
                return false
            }
        }
        return true
    }
    let mut ng = s.len();
    let mut ok = 1;
    if judge(ng, &s) {
        // println!("ng = {:?}", ng);
        ok = ng;
    }
    while (ng as i128 - ok as i128).abs() > 1 {
        let mid = (ng + ok) / 2;
        let is_ok = judge(mid, &s);
        // println!("mid = {mid}, is_ok = {:?}", is_ok);
        if is_ok {
            ok = mid;
        }
        else {
            ng = mid;
        }
    }
    println!("{}", ok);

}

