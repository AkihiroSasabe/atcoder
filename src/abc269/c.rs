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
use superslice::*;
fn main() {
    input! {
        n: usize
    }
    // 1_152_921_504_606_846_976
    // 10^18
    // let n_2 = ten_to_base(n, 2);
    let mut digit_list: Vec<usize> = vec![];
    for i in 0..61 {
        if (1 << i & n) != 0 {
            digit_list.push(i);
        }
    }
    // println!("{:?}", digit_list);

    let mut ans_list = vec![];
    ans_list.push(0);
    for bit in 0..(1 << digit_list.len()) {
        let bit_2 = ten_to_base(bit, 2);
        // println!("bit: {}, bit_2:{}", bit, bit_2);
        let mut elem: usize = 0;
        for j in 0..digit_list.len() {
            if (bit & 1 << j) != 0 {
                // println!("+{}", 1 << digit_list[j]);
                elem += (1 << digit_list[j]);
            }
        }
        if elem != 0 {
            ans_list.push(elem);
        }
    }
    ans_list.sort();
    for i in 0..ans_list.len() {
        println!("{}", ans_list[i]);
    }

}





// 10進法のxをn(base)進法に変換
fn ten_to_base(mut x: usize, base: usize) -> usize {
    // ==== How to use ====
    // let answer = ten_to_base(11, 8);
    // println!("{}", answer); // 13
    // let answer = ten_to_base(2021, 9);
    // println!("{}", answer); // 2685   
    // ==== Theory ====
    // The number x can be expressed in n-decimal notation as follows:
    // (where 0 <= ai <= n-1)
    // x   = a0 * (n^0) + a1 * (n^1) * a2 * (n^2) + ... + amax * (n^max)
    // ==== Example ====
    // 11  = 2*(8**0) + 3*(8**1) 
    // a0 = x % 8
    // a1 = (x / 8) % 8
    // a2 = (x / 8^2) % 8 but (x / 8^2) == 0 => max = 1
    // ... 
    let mut answer = 0;
    let mut digit: u32 = 0;
    while x != 0 {
        let amari = x % base;
        answer += amari * 10_usize.pow(digit);
        x /= base;
        digit += 1;
    }
    return answer;
}