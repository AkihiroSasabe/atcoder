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
    // 2024-03-03 15:13-16:47 (1h34min)
    // 2024-03-04 19:09-19:41 (32min)
    // 2h6min
    input! {
        v1: isize,
        v2: isize,
        v3: isize,
    }

    // 40_353_607
    // 0,0,0で固定で良い。
    // 0-14, 0-14, 0-14

    let x0 = 0;
    let y0 = 0;
    let z0 = 0;

    // テストケース1で、Debug
    // let x1 = 0;
    // let y1 = 6;
    // let z1 = 0;

    // let x2 = 6;
    // let y2 = 0;
    // let z2 = 0;

    // let s012 = get_intersection_for_3boxes(x1, y1, z1, x2, y2, z2);
    // // 0 かつ 1
    // let s01 = get_intersection_for_2boxes(x0, y0, z0, x1, y1, z1);
    // // 1 かつ 2
    // let s12 = get_intersection_for_2boxes(x1, y1, z1, x2, y2, z2);
    // // 2 かつ 0
    // let s20 = get_intersection_for_2boxes(x2,y2,z2, x0, y0, z0);

    // println!("s012 = {:?}", s012);
    // println!("s01 = {:?}", s01);
    // println!("s12 = {:?}", s12);
    // println!("s20 = {:?}", s20);

    // let sum3 = s012;
    // let sum2 = s01 + s12 + s20 - 3 * s012;
    // let sum1 = 3 * 7 * 7 * 7 - 2 * sum2 - 3 * sum3;
    // println!("sum1 = {:?}", sum1);
    // println!("sum2 = {:?}", sum2);
    // println!("sum3 = {:?}", sum3);
    // return;

    for x1 in -14..15 {
        for y1 in -14..15 {
            for z1 in -14..15 {
                for x2 in -14..15 {
                    for y2 in -14..15 {
                        for z2 in -14..15 {
                            // 0 かつ 1 かつ 2 の体積
                            let s012 = get_intersection_for_3boxes(x1, y1, z1, x2, y2, z2);
                            if s012 != v3 {continue}
                            
                            // 0 かつ 1 の体積
                            let s01 = get_intersection_for_2boxes(x0,y0,z0,x1,y1,z1);
                            // 1 かつ 2 の体積
                            let s12 = get_intersection_for_2boxes(x1,y1,z1, x2,y2,z2);
                            // 2 かつ 0 の体積
                            let s20 = get_intersection_for_2boxes(x2,y2,z2, x0, y0, z0);

                            // 包除原理
                            let sum3 = s012;
                            let sum2 = s01 + s12 + s20 - 3 * s012;
                            let sum1 = 3 * 7 * 7 * 7 - 2 * sum2 - 3 * sum3;
                            
                            if sum2 != v2 {continue}
                            if sum1 != v1 {continue}
                            println!("Yes");
                            println!("{} {} {} {} {} {} {} {} {}", x0, y0, z0, x1, y1, z1, x2, y2, z2);
                            return
                        }
                    }
                }
            }
        }
    }
    println!("No");
    
}

fn get_intersection_for_2boxes(x0: isize, y0: isize, z0: isize, x1: isize, y1: isize, z1: isize) -> isize {
    // 2つの立方体のオーバーラップしている領域を計算
    let xlen = get_overlap_length(x0, x0+7, x1, x1+7);
    let ylen = get_overlap_length(y0, y0+7, y1, y1+7);
    let zlen = get_overlap_length(z0, z0+7, z1, z1+7);

    let v = xlen * ylen * zlen;
    return v
}

fn get_intersection_for_3boxes(x1: isize, y1: isize, z1: isize, x2: isize, y2: isize, z2: isize) -> isize {
    // 3つの立方体のオーバーラップしている領域を計算

    let xmin1;
    let xmax1;
    let ymin1;
    let ymax1;
    let zmin1;
    let zmax1;

    let xmin2;
    let xmax2;
    let ymin2;
    let ymax2;
    let zmin2;
    let zmax2;

    // 立方体0と1のオーバーラップした立方体は?
    if let Some((_xmin1, _xmax1)) = get_corner(x1) {
        xmin1 = _xmin1;
        xmax1 = _xmax1;
    }
    else {
        return 0
    }
    if let Some((_ymin1, _ymax1)) = get_corner(y1) {
        ymin1 = _ymin1;
        ymax1 = _ymax1;
    }
    else {
        return 0
    }
    if let Some((_zmin1, _zmax1)) = get_corner(z1) {
        zmin1 = _zmin1;
        zmax1 = _zmax1;
    }
    else {
        return 0
    }


    // 立方体0と2のオーバーラップした立方体は?
    if let Some((_xmin2, _xmax2)) = get_corner(x2) {
        xmin2 = _xmin2;
        xmax2 = _xmax2;
    }
    else {
        return 0
    }
    if let Some((_ymin2, _ymax2)) = get_corner(y2) {
        ymin2 = _ymin2;
        ymax2 = _ymax2;
    }
    else {
        return 0
    }
    if let Some((_zmin2, _zmax2)) = get_corner(z2) {
        zmin2 = _zmin2;
        zmax2 = _zmax2;
    }
    else {
        return 0
    }
    // println!("(xmin1, ymin1, zmin1) =({}, {}, {})", xmin1, ymin1, zmin1);
    // println!("(xmax1, ymax1, zmax1) =({}, {}, {})", xmax1, ymax1, zmax1);
    // println!("(xmin2, ymin2, zmin2) =({}, {}, {})", xmin2, ymin2, zmin2);
    // println!("(xmax2, ymax2, zmax2) =({}, {}, {})", xmax2, ymax2, zmax2);

    let xlen = get_overlap_length(xmin1, xmax1, xmin2, xmax2);
    let ylen = get_overlap_length(ymin1, ymax1, ymin2, ymax2);
    let zlen = get_overlap_length(zmin1, zmax1, zmin2, zmax2);


    return xlen * ylen * zlen
}

fn get_overlap_length(min0: isize, max0: isize, min1: isize, max1: isize) -> isize {
    // 2つの区間 [min0, max0] と、[min1, max1] のオーバーラップしている領域の長さを求める。
    if min1 <= min0 {
        if max1 <= min0 {
            return 0
        }
        else if min0 <= max1 && max1 <= max0{
            return max1 - min0
        }
        else {
            max0 - min0
        }
    }
    else if min0 <= min1 && min1 <= max0 {
        if max1 <= max0 {
            return max1 - min1
        }
        else {
            return max0 - min1
        }
    }
    else {
        return 0
    }
}

fn get_corner(x: isize) -> Option<(isize, isize)> {

    if x <= -7 {
        return None
    }
    else if x <= 0 {
        return Some((0, x+7))
    }
    else if x <= 7 {
        return Some((x, 7))
    }
    else {
        return None
    }

}