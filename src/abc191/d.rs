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
use num::Integer;

fn main() {
    // 10^4 の世界で考える。
    // 2025-04-25 21:18-21:21 (3min)
    // 2025-04-26 13:35-15:35 (2h)
    // Total: 2h3min

    // let t: isize = (-81) / 10;
    // println!("t = {:?}", t);

    // let t: isize = (-80) / 10;
    // println!("t = {:?}", t);

    // let t: isize = (-79) / 10;
    // println!("t = {:?}", t);

    // 除算後に床関数 (⌊x⌋ := x∈R 以下の最大の整数) を適用
    assert!(0 == (4).div_floor(&5)); // 4.0 / 5.0 = 0.2
    assert!(-1 == (-4).div_floor(&5)); // -4.0 / 5.0 = -0.2
    assert!(-1 == (-5).div_floor(&5)); // -5.0 / 5.0 = -1.0
    assert!(-2 == (-6).div_floor(&5)); // -6 / 5.0 = -1.2
    assert!(-2.0 ==( -1.2_f64).floor());

    // 除算後に天井関数(⌈x⌉ := x∈R 以上の最小の整数) を 適用
    assert!(1 == (4).div_ceil(&5)); // 4.0 / 5.0 = 0.2
    assert!(0 == (-4).div_ceil(&5)); // -4.0 / 5.0 = -0.2
    assert!(-1 == (-5).div_ceil(&5)); // -5.0 / 5.0 = -1.0
    assert!(-1 == (-6).div_ceil(&5)); // -6 / 5.0 = -1.2
    assert!(-1.0 ==(-1.2_f64).ceil());

    assert!(12345 == decimal_to_offset_isize("123.45", 2));
    assert!(-12345 == decimal_to_offset_isize("-123.45", 2));
    assert!(12345 == decimal_to_offset_isize("+123.45", 2));

    // RustやC++の除算は、ゼロに近い方に丸める仕様 (Integer division rounds towards zero.)
    // つまり、商が負なら天井関数、商が正なら床関数がかかる。これは使いにくいので、numクレートのdiv_floor()を使うのがおすすめ。
    // 参考: https://doc.rust-lang.org/reference/expressions/operator-expr.html#:~:text=Integer%20division%20rounds%20towards%20zero.
    // Pythonは、符号によらず、常に床関数が掛かる。 (例: -6 // 5 == -2)
    assert!(0 == 4 / 5); // 4.0 / 5.0 = 0.2
    assert!(0 == (-4) / 5); // -4.0 / 5.0 = -0.2
    assert!(-1 == (-5) /5); // -5.0 / 5.0 = -1.0
    assert!(-1 == (-6) /5); // -6 / 5.0 = -1.2

    // return;

    input! {
        xs: String,
        ys: String,
        rs: String,
    }
    let mut xc = decimal_to_offset_isize(&xs, 4);
    let mut yc = decimal_to_offset_isize(&ys, 4);
    let mut r = decimal_to_offset_isize(&rs, 4);
    // println!("(xc, yc, r) = {:?}", (xc, yc, r));

    let offset = 10_000;

    let mut x_min_ind = (xc - r).div_ceil(&offset);
    let mut x_max_ind = (xc + r).div_floor(&offset);
    // println!("(x_min_ind, x_max_ind) = {:?}", (x_min_ind, x_max_ind));

    let mut ans = 0;
    for x_ind in x_min_ind..x_max_ind+1 {

        let x = x_ind * offset;
        let root = usize_floor_sqrt((r*r - (x - xc) * (x - xc)) as usize);
        let y_max = yc + root as isize;
        let y_min = yc - root as isize;

        // println!("x, y_min, y_max = {:?}", (x, y_min, y_max));

        let y_max_ind = y_max.div_floor(&offset);
        let y_min_ind = y_min.div_ceil(&offset);;
        // println!("y_min_ind, y_max_ind = {:?}", (y_min_ind, y_max_ind));
        let diff = y_max_ind - y_min_ind + 1;
        ans += diff;
    }
    println!("{}", ans);
    
}

fn usize_floor_sqrt(n: usize) -> usize {
    // エビちゃん ( @rsk0315 ) が作った、nの平方根 (= √n) 以下で最大の整数を取得する関数 
    // 二分探索で真面目に書いても良いかも。 (この関数は、u64だと動くけど、u128とかだと動くかは未検討なので注意)
    // https://atcoder.jp/contests/abc400/editorial/12642

    let tmp = (n as f64).sqrt() as usize;
    let tmp_m1 = tmp.saturating_sub(1);
    if tmp_m1 * (tmp_m1 + 2) < n { tmp } else { tmp_m1 }
}

/// 小数の文字列をオフセットして、整数に変換する関数 
/// 例: assert!(12345 == decimal_to_offset_isize("123.45", 2)); 
/// 例: assert!(12345 == decimal_to_offset_isize("+123.45", 2)); 
/// 例: assert!(-12345 == decimal_to_offset_isize("-123.45", 2)); 
/// (使用例: ABC191D https://atcoder.jp/contests/abc191/tasks/abc191_d)
/// s: 小数の文字列
/// fraction_digits: 小数部分の桁の個数。この数ぶけ全体的に各桁をオフセットする。
fn decimal_to_offset_isize(s: &str, fraction_digits: usize) -> isize {
    // 先頭の符号を確認
    let (sign, number_without_sign) = if let Some(rest) = s.strip_prefix('-') {
        (-1, rest)
    }
    else if let Some(rest) = s.strip_prefix('+') {
        (1, rest)
    }
    else {
        (1, s)
    };

    // 整数部分と小数部分に分ける
    let (integer_part_str, decimal_part_str) = match number_without_sign.split_once('.') {
        Some((int_str, dec_str)) => (int_str, dec_str),
        None => (number_without_sign, "0")
    };
    
    // 整数部分の取得
    let integer_part: isize = integer_part_str.parse::<isize>().unwrap_or(0);

    // 指定された小数点部分の桁数まで、不足がないように0埋め
    let mut decimal_part_vec: Vec<char> = decimal_part_str.chars().collect::<Vec<char>>();
    for _ in 0..(fraction_digits-decimal_part_vec.len()) {
        decimal_part_vec.push('0');
    }
    // 小数部分の取得
    let mut decimal_part: isize = 0;
    for c in decimal_part_vec {
        decimal_part *= 10;
        decimal_part += c.to_digit(10).unwrap() as isize;
    }    

    // オフセット量の計算
    let mut offset: isize = 1;
    for _ in 0..fraction_digits {
        offset *= 10;
    }
    
    // 小数をオフセットして整数で表現
    let offset_number = sign * (integer_part * offset + decimal_part);
    return offset_number
}