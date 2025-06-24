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
    // 2025-06-16 12:32-12:49
    // 2025-06-16 23:33-
    input! {
        t: usize,
    }

    let mut h = vec![];
    let mut w = vec![];
    let mut s = vec![];
    for i in 0..t {
        input! {
            hi: usize,
            wi: usize,
            si: [Chars; hi], 
        }
        h.push(hi);
        w.push(wi);
        s.push(si);
    }

    // 公式解説通り、長方形領域の上辺と底辺を全探索すると、O(h^2 * w) の計算量で解ける。
    // h*w<=3*10^5 ということは、
    // h<=w のとき、
    // h*h <= h*w <= 3*10^5
    // h <= 547.722.. < 548
    // h^2*w <= 548 * 3*10^5 = 1.644 * 10^8 これならギリ間に合う。
    // 長方形領域の上辺y=ysと底辺y=ytを固定した場合、
    // 区間[xs, xt]の中の'#'と'.'の数が等しいような、(xs,xt)の組が何個存在するかをO(W)で解けばいい。
    // それは、累積和(cum)と、累積和の値が何個存在するかカウントするための配列(nums)を使うと解ける。
    // 具体的には、xs=0からxs=w-1までの寄与を、numsを動的に更新しながら、シミュレートすることで解ける。
    for i in 0..t {
        // println!("i = {:?} ---- ---- ----", i);
        let mut hi = h[i];
        let mut wi = w[i];
        // '#' を +1 に、 '.' を -1 に置換した2次元配列を作成。
        let mut si: Vec<Vec<isize>> = convert_2dmap(&s[i]);
        // hi < wi にする。
        if wi < hi {
            si = rotate_left_90deg(&si);
            swap(&mut hi, &mut wi);
        }
        // debug
        // println!("si");
        // for y in 0..hi {
        //     for x in 0..wi {
        //         print!("{:2} ", si[y][x]);
        //     }
        //     println!("");
        // }

        // 2次元累積和
        let cum = get_2d_cumulative_sum(&si);
        // println!("si = {:?}", si);
        // println!("cum = {:?}", cum);
        // println!("cum");
        // for y in 0..hi {
        //     for x in 0..wi {
        //         print!("{:2} ", get_sum_from_2d_cum(&cum, 0, 0, y, x));
        //     }
        //     println!("");
        // }

        // O(h^2w)
        let mut ys = 0;
        let mut yt = 0;
        // nums[sum] := [0..=x]の区間の累積和をx=(0..w)まで計算した後、累積和が sum となる累積和が何個あるかを動的に記録
        // ただし、 sum は負の数もあり得るので、実際はh*wをオフセットしておく必要がある。
        let mut nums = vec![0; 2*hi*wi+1];

        let mut ans: usize = 0;
        for ys in 0..hi {
            for yt in ys..hi {
                // println!("bef nums = {:?}", nums);
                for x in 0..wi {
                    let sum = get_sum_from_2d_cum(&cum, ys, 0, yt, x);
                    // println!("(ys, yt, x, sum) = {:?}", (ys, yt, x, sum));
                    let index = ((hi*wi) as isize + sum) as usize;
                    nums[index] += 1;
                }
                
                // println!("mid nums = {:?}", nums);
                // x=0のときだけ、寄与とnumsの更新は別途計算
                ans += nums[hi*wi];
                let sum = get_sum_from_2d_cum(&cum, ys, 0, yt, 0);
                let index = ((hi*wi) as isize + sum) as usize;
                nums[index] -= 1;

                // x = 1..wiのときの寄与とnumsの更新を計算
                for x in 1..wi {
                    let sum = get_sum_from_2d_cum(&cum, ys, 0, yt, x-1);
                    let index = ((hi*wi) as isize + sum) as usize;
                    ans += nums[index];

                    let sum = get_sum_from_2d_cum(&cum, ys, 0, yt, x);
                    let index = ((hi*wi) as isize + sum) as usize;
                    nums[index] -= 1;
                }
                // println!("aft nums = {:?}", nums);
            }
        }
        println!("{}", ans);
        // return;
    }
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

fn convert_2dmap(s: &Vec<Vec<char>>) -> Vec<Vec<isize>> {
    let h = s.len();
    let w = s[0].len();
    let mut ans = vec![vec![0; w]; h];
    for y in 0..h {
        for x in 0..w {
            if s[y][x] == '#' {
                ans[y][x] = 1;
            }
            else {
                ans[y][x] = -1;
            }
        }
    }
    return ans
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

/// 2次元累積和から、閉区間(ys, xs) ~ (yt, xt) で囲まれた矩形領域の和を求める関数
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