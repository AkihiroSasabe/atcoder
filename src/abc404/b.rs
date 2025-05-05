#![allow(dead_code, unused_imports)]
use proconio::{input, marker::{Usize1, Isize1, Chars}};
use itertools::Itertools;
use std::cmp::{max, min, Ordering, Reverse};
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::collections::{VecDeque, BinaryHeap, HashMap, BTreeMap, HashSet, BTreeSet};
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
use rand::Rng;
use num::{BigUint, ToPrimitive, Integer};
use num_bigint::ToBigUint;
fn main() {
    input! {
        n: usize,
        s: [Chars; n],
        t: [Chars; n],
    }

    // let s3 = rotate(&s);
    // let s2 = rotate(&s3);
    // let s1 = rotate(&s2);

    let s1 = rotate_right_90deg(&s);
    let s2: Vec<Vec<char>> = rotate_right_90deg(&s1);
    let s3: Vec<Vec<char>> = rotate_right_90deg(&s2);

    let mut ans = 1 << 60;
    ans = min(ans, count(&s, &t, n));
    ans = min(ans, count(&s1, &t, n) + 1);
    ans = min(ans, count(&s2, &t, n) + 2);
    ans = min(ans, count(&s3, &t, n) + 3);

    println!("{}", ans);

}

fn count(s: &Vec<Vec<char>>, t: &Vec<Vec<char>>, n: usize) -> usize {
    let mut ans = 0;
    for y in 0..n {
        for x in 0..n {
            if s[y][x] != t[y][x] {
                ans += 1;
            }
        }
    }
    return ans
}


/// 画像を左回り（反時計周り）に90度回転させる
fn rotate_left_90deg<T>(image: &Vec<Vec<T>>) -> Vec<Vec<T>> 
    where T: 
        Default + 
        Copy 
{
    let next_h = image[0].len();
    let next_w = image.len();

    let pre_h = next_w;
    let pre_w = next_h;
    
    let zero: T = Default::default();
    let mut rotated: Vec<Vec<T>> = vec![vec![zero; next_w]; next_h];
    
    for pre_y in 0..pre_h {
        for pre_x in 0..pre_w {
            let next_y = pre_w - 1 - pre_x;
            let next_x = pre_y;
            rotated[next_y][next_x] = image[pre_y][pre_x];
        }
    }
    return rotated
}

/// 画像を右回り（時計周り）に90度回転させる
fn rotate_right_90deg<T>(image: &Vec<Vec<T>>) -> Vec<Vec<T>> 
    where T: 
        Default + 
        Copy 
{
    let next_h = image[0].len();
    let next_w = image.len();

    let pre_h = next_w;
    let pre_w = next_h;
    
    let zero: T = Default::default();
    let mut rotated: Vec<Vec<T>> = vec![vec![zero; next_w]; next_h];
    
    for pre_y in 0..pre_h {
        for pre_x in 0..pre_w {
            let next_y = pre_x;
            let next_x = pre_h - 1 - pre_y;
            rotated[next_y][next_x] = image[pre_y][pre_x];
        }
    }
    return rotated
}
