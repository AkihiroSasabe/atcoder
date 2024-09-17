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
use rand::Rng;
fn main() {
    // 2024-09-16 12:53-スニペット作り
    // 2024-09-16 13:12-13:51 (39min )
    input! {
        n: usize,
    }

    input! {
        k: usize
    }
    let mut x = vec![];
    let mut y = vec![];
    let mut x_min = 1_000_000_000;
    let mut y_min = 1_000_000_000;
    for i in 0..n {
        input!{
            xi: isize,
            yi: isize,
        }
        x.push(xi);
        y.push(yi);
        x_min = min(x_min, xi);
        y_min = min(y_min, yi);
    }
    for i in 0..n {
        x[i] -= x_min;
        y[i] -= y_min;
    }
    let x: Vec<usize> = x.iter()
        .map(|&xi| xi as usize)
        .collect();

    let y: Vec<usize> = y.iter()
        .map(|&xi| xi as usize)
        .collect();

    // println!("x = {:?}", x);
    // println!("y = {:?}", y);

    let (x_ranked_array, x_sorted_array, x_original_to_rank) = rank_array(&x);
    let (y_ranked_array, y_sorted_array, y_original_to_rank) = rank_array(&y);

    let mut img = vec![vec![0; n]; n];
    for i in 0..n {
        let xi = x_ranked_array[i];
        let yi = y_ranked_array[i];
        img[yi][xi] += 1;
    }
    let cum2d = get_2d_cumulative_sum(&img);
    // println!("img = {:?}", img);
    // println!("cum2d = {:?}", cum2d);

    let mut ans = 1 << 63;
    for ix in 0..n {
        for jx in ix+1..n {
            for iy in 0..n {
                for jy in iy+1..n {
                    // 
                    let min_x = x_sorted_array[ix];
                    let max_x =  x_sorted_array[jx];

                    let min_y = y_sorted_array[iy];
                    let max_y = y_sorted_array[jy];

                    let num = get_sum_from_2d_cum(&cum2d, iy, ix, jy, jx);

                    if num < k {continue}

                    let x_width= (max_x - min_x);
                    let y_width= (max_y - min_y);

                    let area = x_width * y_width;
                    // println!("x_width = {:?}", x_width);
                    // println!("y_width = {:?}", y_width);
                    // println!("area = {:?}", area);
                    ans = min(ans, area);
                }
            }
        }
    }
    println!("{}", ans);
}

fn rank_array<T: Ord + std::hash::Hash + Clone + Copy>(array: &Vec<T>) -> (Vec<usize>, Vec<T>, HashMap<T, usize>) {
    // 配列を順位変換する関数 O(NlogN)
    // 要素の値を圧縮することを、目的として使うことを想定している。(座標圧縮)
    // Input: 
    //     array: 配列
    // Output: 
    //    ranked_array:     順位変換済み配列
    //    sorted_array:     ソート済みの配列(順位から元の値をマップさせる)
    //    original_to_rank: 元の値から順位を対応させるマップ
    // Example:
    // let array = vec![333, 111, 444, 111, 555, 999];
    // let (ranked_array, _sorted_array, _original_to_rank) = rank_array(&array);
    // assert_eq!(ranked_array, vec![2, 0, 3, 0, 4, 5]);

    // 配列のサイズ
    let n = array.len();

    // B木<数列中に登場する値, 頻度>
    let mut btree: BTreeMap<T, usize> = BTreeMap::new();
    for i in 0..n {
        *(btree.entry(array[i]).or_insert(0)) += 1;
    }

    // 昇順ソート済みの、順位変換済み配列
    let mut sorted_rank_array = vec![];
    let mut rank = 0;
    for (k, frequency) in btree {
        for j in 0..frequency {
            sorted_rank_array.push(rank);
        }
        rank += frequency; // sorted_rank_array = [0, 0, 2, 3, 4, 5], 
        // ここを1にすると、隙間なくなる。
        // rank += 1; //sorted_rank_array = [0, 0, 1, 2, 3, 4], 
    }
    // println!("sorted_rank_array = {:?}, ", sorted_rank_array);

    // 順位から元の値をマップさせる
    let mut sorted_array = (*array).clone();
    sorted_array.sort();

    // 元の値から順位を対応させるマップ
    let mut original_to_rank: HashMap<T, usize> = HashMap::new();
    for i in 0..n {
        original_to_rank.insert(sorted_array[i], sorted_rank_array[i]);
    }

    // 元の順序の、順位変換済み配列
    let mut ranked_array: Vec<usize> = vec![];
    for i in 0..n {
        ranked_array.push(*(original_to_rank.get(&array[i]).unwrap()));
    }

    return (ranked_array, sorted_array, original_to_rank)
}


/// 2次元累積和 cum を計算する関数
/// cum[y][x] := 原点(0,0)から右下(y,x)までの画素値の累積和
fn get_2d_cumulative_sum<T>(img: &Vec<Vec<T>>) -> Vec<Vec<T>> 
    where T: std::ops::Add<Output = T> + std::ops::Sub<Output = T> + Copy + Default
{
    let h = img.len();
    let w = img[0].len();
    let zero: T = Default::default();

    // 累積和の初期化
    let mut cum: Vec<Vec<T>> = vec![vec![zero; w]; h];
    cum[0][0] = img[0][0];

    // 1行目(y=0)の計算
    for x in 1..w {
        cum[0][x] = img[0][x] + cum[0][x-1];
    }
    // 2行目以降(y>0)の計算
    for y in 1..h {
        // 1列目(x=0)の計算
        cum[y][0] = img[y][0] + cum[y-1][0];
        // 2列目以降(x>0)の計算
        for x in 1..w {
            cum[y][x] = img[y][x] + cum[y-1][x] + cum[y][x-1] - cum[y-1][x-1];
        }
    }

    return cum
}

/// 2次元累積和から、(ys, xs) ~ (yt, xt) で囲まれた矩形領域の和を求める関数
fn get_sum_from_2d_cum<T>(cum_2d: &Vec<Vec<T>>, ys: usize, xs: usize, yt: usize, xt: usize) -> T 
    where T: std::ops::Add<Output = T> + std::ops::Sub<Output = T> + Copy,
{
    if xs == 0 && ys == 0 {
        let sum = cum_2d[yt][xt];
        return sum
    }
    else if xs == 0 && ys != 0 {
        let sum = cum_2d[yt][xt] - cum_2d[ys-1][xt];
        return sum
    }
    else if xs != 0 && ys == 0 {
        let sum = cum_2d[yt][xt] - cum_2d[yt][xs-1];
        return sum
    }
    else {
        // xs != 0 && ys != 0
        let sum = cum_2d[yt][xt] + cum_2d[ys-1][xs-1] - cum_2d[yt][xs-1] - cum_2d[ys-1][xt];
        return sum
    }
}