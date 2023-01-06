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
    // 2023-01-05 18:57-20:00 (1h3m)
    // 21:25-23:21 (1h56m)
    // TOTAL: 2h59min
    input! {
        a: usize,
        n: usize,
    }

    let INF = std::usize::MAX;
    let mut ans = INF;
    let mut seen = vec![INF; 1_000_001];
    dfs(n, &mut ans, 0, a, &mut seen);

    if ans == INF {
        println!("-1");
    }
    else {
        println!("{}", ans);
    }
}

fn dfs(mut n: usize, ans: &mut usize, depth: usize, a: usize, seen: &mut Vec<usize>) {
    // println!("n :{n}. depth: {depth}");
    seen[n] = depth;
    if n == 0 {
        return
    }
    else if n == 1 {
        *ans = min(*ans, depth);
        // if depth == 110 {
        //     std::thread::sleep(std::time::Duration::from_millis(1000000));
        // }
        return
    }
    let digit_num = get_digit_num(n);
    if is_changeable(n, digit_num) {
        // dbg!(n);
        if n % a == 0 && depth+1 < seen[n/a] {
            // dbg!(n);
            dfs(n/a, ans, depth+1, a, seen);
        }
        for i in 0..digit_num-1 {
            if !is_changeable(n, digit_num) {continue}
            n = change(n, digit_num);
            // dbg!(n, depth+1+i, seen[n]);
            if depth+1+i >= seen[n] {continue}
            dfs(n, ans, depth+1+i, a, seen);
            if n % a != 0 {continue}
            if depth+2+i >= seen[n/a] {continue}
            dfs(n/a, ans, depth+2+i, a, seen);
        }
    }
    else {
        if n % a != 0 {return}
        if depth + 1 >= seen[n/a] {return}
        dfs(n/a, ans, depth + 1, a, seen);
    }    
}

fn is_changeable(x: usize, digit_num: usize) -> bool {
    if digit_num < 2 {
        return false
    } 
    // 先頭から2桁目が0のケースは駄目
    // ex. 106 => 061 = 61 になってしまい、桁数が変わってしまうので。
    let scale = 10_usize.pow(digit_num as u32-1);
    let amari = x % scale;
    if amari / (scale / 10) == 0 {
        return false
    }
    else {
        return true
    }
    
}

fn get_digit_num(mut x: usize) -> usize {
    let mut digit = 0;
    while x > 0 {
        digit += 1;
        x /= 10;
    }
    return digit
}

// 先頭をケツに持っていく
fn change(x: usize, digit_num: usize) -> usize {
    let scale = 10_usize.pow(digit_num as u32-1);
    let top = x / scale;
    return (x - top * scale) * 10 + top
}

// 123 => 312 ケツを先頭に持っていく
// 312 => 123 先頭をケツに持っていく

