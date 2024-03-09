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
    // 2024-03-08 23:29-23:38 (9min)
    // 2024-03-09 13:52-14:53 (61min)
    // 70min

    // 2024-03-09 (Sat.) 15:29-
    // 水色はいま112問解いた。全体で356問で、半分は178問
    // 178 - 112 = 66問

    // 毎日2問ペースで解けば、
    // 22日 * 2 = 44問
    // 112 + 44 = 156問合計で解ける    

    // 毎日3問ペースで解けば、水色は半分は終わる。
    // 22日 * 3 = 66問
    // ちょうど、水色の半分は解いたことになる。

    input! {
        x_chars: Chars,
        // m_chars: Chars,
        m: i128
    }
    // 0-9
    let mut x = vec![];
    let mut d = 0;
    for i in x_chars {
        let num = i as usize - '0' as usize;
        let num = num as i128;
        d = max(d, num);
        x.push(num);
    }

    // 底
    let n = d + 1;
    if let Some(x_10) = convert_x_from_basen_to_base10(n, &x) {
        if x_10 > m {
            println!("0");
            return
        }
    }
    else {
        println!("0");
        return
    }

    if x.len() == 1 {
        println!("1");
        return
    }

    // めぐる式二分探索
    let mut ng = 1_000_000_000_000_000_003; // 10^18 + 1
    let mut ok = n;
    while (ng - ok).abs() > 1 {
        let mid = (ng + ok) / 2;

        if judge(mid, &x, m) {
            ok = mid;
        }
        else {
            ng = mid;
        }
    }
    let ans = ok - n + 1;
    println!("{}", ans);
}

fn judge(n: i128, x: &Vec<i128>, m: i128) -> bool {
    if let Some(x_10) = convert_x_from_basen_to_base10(n, x) {
        return x_10 <= m 
    }
    else {
        return false
    }
}


fn convert_x_from_basen_to_base10(n: i128, x: &Vec<i128>) -> Option<i128> {
    // n進法の x を 10進法に変換する関数

    // x[0] は、最上位の桁の数が格納されているので注意
    // 例えばn進法の 167 は、配列上で
    // x = vec![1, 6, 7] としているので注意。

    let x_len = x.len();
    let mut x_10 = 0;
    let mut temp = 1;
    
    let mut max_kakerareru = i128::MAX / n;
    for i in (0..x_len).rev() {
        if max_kakerareru < temp {
            return None
        }
        x_10 += temp * x[i];
        temp *= n;
    }
    return Some(x_10)    
}
