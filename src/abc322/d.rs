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
    // let mask: Vec<Vec<usize>> = vec![
    //     vec![0,0,0,0],
    //     vec![1,1,1,0],
    //     vec![0,1,0,0],
    //     vec![0,0,0,0]
    // ];

    // let bbox = get_bbox(&mask);
    // println!("bbox = {:?}", bbox);
    // return;
    input! {
        p0: [Chars; 4],
        p1: [Chars; 4],
        p2: [Chars; 4],
    }
    let c0 = trim_mask_special(&p0, '#');
    let c1 = trim_mask_special(&p1, '#');
    let c2 = trim_mask_special(&p2, '#');
    // c0.print_2d_array_with_name("p0");
    // c1.print_2d_array_with_name("p1");
    // c2.print_2d_array_with_name("p2");

    let mut c0s = vec![c0];
    let mut c1s = vec![c1];
    let mut c2s = vec![c2];

    for i in 0..3 {
        let c0_rotate = rotate(&c0s[c0s.len()-1]);
        let c1_rotate = rotate(&c1s[c1s.len()-1]);
        let c2_rotate = rotate(&c2s[c2s.len()-1]);
        // c0_rotate.print_2d_array_with_name("i");
        c0s.push(c0_rotate);
        c1s.push(c1_rotate);
        c2s.push(c2_rotate);
    }


    for pose0 in 0..4 {
        for pose1 in 0..4 {
            for pose2 in 0..4 {
                let cc0 = &c0s[pose0];
                let cc1 = &c1s[pose1];
                let cc2 = &c2s[pose2];
                for y0 in 0..(5-cc0.len()) {
                    for x0 in 0..(5-cc0[0].len()) {
                        for y1 in 0..(5-cc1.len()) {
                            for x1 in 0..(5-cc1[0].len()) {
                                for y2 in 0..(5-cc2.len()) {
                                    for x2 in 0..(5-cc2[0].len()) {
                                        let flag = check(y0, x0, y1, x1, y2, x2, cc0, cc1, cc2);
                                        if flag {
                                            println!("Yes");
                                            return;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    println!("No");


    
}


fn check(y0: usize, x0: usize, y1: usize, x1: usize, y2: usize, x2: usize, c0: &Vec<Vec<usize>>, c1: &Vec<Vec<usize>>, c2: &Vec<Vec<usize>>) -> bool {
    let mut base = vec![vec![0; 4]; 4];
    for y in 0..c0.len() {
        for x in 0..c0[0].len() {
            base[y+y0][x+x0] += c0[y][x];
        }
    }
    for y in 0..c1.len() {
        for x in 0..c1[0].len() {
            base[y+y1][x+x1] += c1[y][x];
        }
    }
    for y in 0..c2.len() {
        for x in 0..c2[0].len() {
            base[y+y2][x+x2] += c2[y][x];
        }
    }

    for y in 0..4 {
        for x in 0..4 {
            if base[y][x] != 1 {
                return false;
            }
        }
    }
    return true;
}


/// 画像を90度回転させる
fn rotate<T>(image: &Vec<Vec<T>>) -> Vec<Vec<T>> 
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

/// maskのbbox [y_min, x_min, y_max, x_max] を取得する関数
fn get_bbox<T>(mask: &Vec<Vec<T>>) -> Vec<usize> 
    where T: 
        Default + 
        Copy + 
        std::fmt::Display + 
        std::cmp::PartialEq + 
        std::ops::Not<Output = T> + 
        std::ops::Div<Output = T>,
{
    let zero: T = Default::default();
    let not_zero: T = !zero;
    let one: T = not_zero / not_zero; 

    let ans = get_bbox_special(mask, one);

    return ans
}

/// maskのbbox [y_min, x_min, y_max, x_max] を取得する関数. mask領域に格納された値を指定する必要がある。
fn get_bbox_special<T>(mask: &Vec<Vec<T>>, mask_value: T) -> Vec<usize> 
    where T: std::cmp::PartialEq
{
    let h = mask.len();
    let w = mask[0].len();

    let mut x_min = w-1;
    let mut x_max = 0;
    let mut y_min = h-1;
    let mut y_max = 0;
    for y in 0..h {
        for x in 0..w {
            if mask[y][x] == mask_value {
                x_min = min(x_min, x);
                y_min = min(y_min, y);
                x_max = max(x_max, x);
                y_max = max(y_max, y);
            }
        }
    }

    return vec![y_min, x_min, y_max, x_max]
}

/// maskの余白部分(bboxの外)を削って、サイズを小さくする
fn trim_mask<T>(mask: &Vec<Vec<T>>) -> Vec<Vec<usize>>
    where T: 
        Default + 
        Copy + 
        std::fmt::Display + 
        std::cmp::PartialEq + 
        std::ops::Not<Output = T> + 
        std::ops::Div<Output = T>,
{
    let zero: T = Default::default();
    let not_zero: T = !zero;
    let one: T = not_zero / not_zero; 
    
    let trimmed = trim_mask_special(mask, one);

    return trimmed
}

/// maskの余白部分(bboxの外)を削って、サイズを小さくする。mask領域に格納された値を指定する必要がある。
fn trim_mask_special<T>(mask: &Vec<Vec<T>>, mask_value: T) -> Vec<Vec<usize>>
    where T: std::cmp::PartialEq + Copy
{
    let bbox = get_bbox_special(mask, mask_value);

    let y_min = bbox[0];
    let x_min = bbox[1];
    let y_max = bbox[2];
    let x_max = bbox[3];

    let h = 1 + y_max - y_min;
    let w = 1 + x_max - x_min;
    let mut trimmed = vec![vec![0; w]; h];

    for y in 0..h {
        for x in 0..w {
            if mask[y_min + y][x_min + x] == mask_value {
                trimmed[y][x] = 1;
            }
        }
    }

    return trimmed
}


// デバッグ用に2次元配列をprintするトレイト
pub trait Print2DArray {
    fn print_2d_array(&self);
    fn print_2d_array_with_name(&self, name: &str);
    fn print_2d_array_transposed(&self);
    fn print_2d_array_transposed_with_name(&self, name: &str);
}
impl<T: std::fmt::Display> Print2DArray for Vec<Vec<T>> {
    fn print_2d_array(&self) {
        for y in 0..self.len() {
            for x in 0..self[y].len() {
                print!("{} ", self[y][x]);
            }
            println!("");
        }
    }
    fn print_2d_array_with_name(&self, name: &str) {
        println!("-=-=-=-= {} -=-=-=-=", name);
        self.print_2d_array();
        println!("-=-=-=-=-=-=-=-=");
    }
    fn print_2d_array_transposed(&self) {
        for x in 0..self[0].len() {
            print!("{:02}: ", x);
            for y in 0..self.len() {
                print!("{} ", self[y][x]);
            }
            println!("");
        }
    }
    fn print_2d_array_transposed_with_name(&self, name: &str) {
        println!("-=-=-=-= transposed {} -=-=-=-=", name);
        self.print_2d_array_transposed();
        println!("-=-=-=-=-=-=-=-=");
    }
}