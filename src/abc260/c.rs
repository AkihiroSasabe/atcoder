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
        x: usize,
        y: usize
    }

    // dpの中身は、レベル1の青い宝石の個数。添字はレベル
    let mut dp_r = vec![0; 11];
    let mut dp_b = vec![0; 11];
    
    dp_r[1] = 0;
    dp_b[1] = 1;

    dp_r[2] = x*y;
    dp_b[2] = y;

    // // 解法1: 動的計画法
    // for i in 2..n+1 {
    //     dp_b[i] = dp_r[i-1] + dp_b[i-1] * y;
    //     dp_r[i] = dp_r[i-1] + dp_b[i] * x;
    // }
    // let answer = dp_r[n];

    // 解法2: 再帰関数 + 動的計画法　（相互再帰している: 2つの別々の再帰関数を使っている）
    let answer = get_r(n, &mut dp_r, &mut dp_b, x, y);

    // println!("{}", dp_r[n]);
    println!("{}", answer);


}


fn get_r(i: usize, dp_r: &mut Vec<usize>, dp_b: &mut Vec<usize>, x: usize, y: usize) -> usize {
    if i == 1 {
        return 0
    }
    else if dp_r[i] != 0 {
        return dp_r[i]
    }
    else {
        dp_r[i] = get_r(i - 1, dp_r, dp_b, x, y) + x * get_b(i, dp_r, dp_b, x, y);
        return dp_r[i]
    }
}

fn get_b(i: usize, dp_r: &mut Vec<usize>, dp_b: &mut Vec<usize>, x: usize, y: usize) -> usize {
    if i == 1 {
        return 1
    }
    else if dp_b[i] != 0 {
        return dp_b[i]
    }
    else {
        dp_b[i] = get_r(i - 1, dp_r, dp_b, x, y) + y * get_b(i-1, dp_r, dp_b, x, y);
        return dp_b[i]
    }

}





