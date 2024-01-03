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
    // 2023-12-19 20:17-21:33 (1h16min)
    input! {
        x: Chars
    }

    let keta = x.len();
    let mut x_nums = vec![];
    let mut x_num = 0;
    let mut power = 1;
    for i in (0..keta).rev() {
        let digit =  x[i] as isize - '0' as isize;
        x_nums.push(digit);
        x_num += digit * power;
        power *= 10;
    }
    x_nums.reverse();

    // 2桁以下なら、自身が等差数
    if x.len() <= 2 {
        println!("{}", x_num);
        return;
    }

    let diff = x_nums[0] - x_nums[1];

    let mut is_arithmetic_num = true;
    for i in 0..keta-1 {
        if x_nums[i] - x_nums[i+1] != diff {
            is_arithmetic_num = false;
            break
        }
    }

    if is_arithmetic_num {
        println!("{}", x_num);
        return
    }

    const INF: isize = 1_000_000_000_000_000_000; // 19桁の数
    let ans = get_arithmetic_num(x_num, &x_nums);

    // println!("ans = {:?}", ans);

    if ans != INF {
        println!("{}", ans);
        return;
    }
    else {    
        loop {
            // 桁を繰り上げ
            if x_nums[0] == 9 {
                x_num = 1;
                x_nums.insert(0, 1);
                for i in 1..x_nums.len() {
                    x_nums[i] = 0;
                    x_num *= 10;
                }
            }
            else {
                // 
                // println!("x_num = {:?}", x_num);
                // println!("x_nums = {:?}", x_nums);
                x_nums[0] += 1;
                x_num = 1;
                for i in 1..x_nums.len() {
                    x_num *= 10;
                }
            }
            // println!("x_num = {:?}", x_num);
            // println!("x_nums = {:?}", x_nums);

            let ans = get_arithmetic_num(x_num, &x_nums);
            // println!("ans = {:?}", ans);
            // println!("x_num = {:?}", x_num);
            if ans != INF {
                println!("{}", ans);
                return;
            }
        }
    }
    // 8_989_898_989

}


fn get_arithmetic_num(x_num: isize, x_nums: &Vec<isize>) -> isize {
    // println!("check x_num = {:?}", x_num);
    const INF: isize = 1_000_000_000_000_000_000; // 19桁の数
    let mut ans = INF;
    let keta = get_digit_num(x_num);
    let head_num = x_nums[0];

    for diff in (-9..10) {
        // println!("diff = {:?}", diff);
        let tail_num = head_num + (keta as isize - 1) * diff;
        // println!("tail_num = {:?}", tail_num);

        // 末尾の桁 が 0 か 9 なら終了
        if tail_num < 0 || 9 < tail_num {
            continue
        }

        // 実際に等差数 num を計算
        let mut num = 0;
        let mut digit = tail_num;
        let mut power = 1;
        for _ in 0..keta {
            num += digit * power;
            digit = digit - diff;
            power *= 10;
        }
        // println!("num = {:?}", num);
        if num < x_num {continue}
        ans = min(ans, num);
    }

    return ans
    
}

fn get_digit_num(mut x: isize) -> isize {
    let mut digit = 0;
    while x > 0 {
        digit += 1;
        x /= 10;
    }
    return digit
}