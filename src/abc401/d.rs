#![allow(dead_code, unused_imports)]
use proconio::{input, marker::{Usize1, Isize1, Chars}};
use itertools::Itertools;
use std::cmp::{max, min, Ordering, Reverse};
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::BTreeMap;
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::collections::{HashSet, BTreeSet};
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
use rand::Rng;
use num::{BigUint, ToPrimitive};
use num_bigint::ToBigUint;
fn main() {
    // 2025-04-12 21:18-22:30 (1h12min)
    input! {
        n: usize,
        mut k: usize,
        mut s: Chars,
    }
    // 'o' の隣にあるやつは、事前に'.'にしておく。
    for i in 0..n {
        if s[i] == 'o' {
            k -= 1;
            if 0 < i {
                s[i-1] = '.';
            }
            if i < n - 1 {
                s[i+1] = '.';
            }
        }
    }

    // renzoku[i] := i は、oの連結成分の何個目か? 後ろから。
    let mut renzoku = vec![0; n];
    if s[n-1] == '?' {
        renzoku[n-1] = 1;
    }
    for i in (0..n-1).rev() {
        if s[i] == '?' {
            if s[i+1] == '?' {
                renzoku[i] = renzoku[i+1] + 1;
            }
            else {
                renzoku[i] = 1;
            }
        }
        else {
            renzoku[i] = 0;
        }
    }

    // f_renzoku[i] := 前から数えた連続
    let mut f_renzoku = vec![0; n];
    if s[0] == '?' {
        f_renzoku[0] = 1;
    }
    for i in 1..n {
        if s[i] == '?' {
            if s[i-1] == '?' {
                f_renzoku[i] = f_renzoku[i-1] + 1;
            }
            else {
                f_renzoku[i] = 1;
            }
        }
        else {
            f_renzoku[i] = 0;
        }
    }

    // can[i] := i以降で置ける'o'の個数。後ろから。
    let mut can = vec![0; n];
    if s[n-1] == '?' {
        can[n-1] = 1;
    }

    for i in (0..n-1).rev() {
        if renzoku[i] == 0 {
            can[i] = can[i+1];
        }
        else {
            if renzoku[i] % 2 == 1 {
                can[i] = can[i+1] + 1;
            }
            else {
                can[i] = can[i+1];
            }
        }
    }

    // f_can[i] := i以前で置ける'o'の個数。後ろから。
    let mut f_can: Vec<usize> = vec![0; n];
    if s[0] == '?' {
        f_can[0] = 1;
    }
    for i in 1..n {
        if f_renzoku[i] == 0 {
            f_can[i] = f_can[i-1];
        }
        else {
            if f_renzoku[i] % 2 == 1 {
                f_can[i] = f_can[i-1] + 1;
            }
            else {
                f_can[i] = f_can[i-1];
            }
        }
    }

    let mut anss = s.clone();
    // println!("s = {:?}", s);

    for i in 0..n {
        if s[i] == '?' {
            let mut pre = 0;
            let mut post = 0;
            let mut pre2 = 0;
            let mut post2 = 0;
            if i != 0 {
                pre = f_can[i-1];
            }
            if i != n-1 {
                post = can[i+1];
            }
            if i > 1 {
                pre2 = f_can[i-2];
            }
            if i+2 < n {
                post2 = can[i+2];
            }
            
            // 置いちゃ駄目
            if k == 0 {
                anss[i] = '.';
                continue
            }

            // 初期化
            let mut is_ok_for_without_o = false;
            let mut is_ok_for_with_o= false;
            // 置かなくていい
            if pre + post >= k  {
                is_ok_for_without_o = true;
            }
            // 置いていい
            if pre2 + post2 + 1 >= k {
                is_ok_for_with_o = true;
            }

            if is_ok_for_with_o && is_ok_for_without_o {
                anss[i] = '?';
            }
            else if is_ok_for_with_o {
                anss[i] = 'o';
            }
            else if is_ok_for_without_o {
                anss[i] = '.';
            }
        }
    }
    for ans in anss {
        print!("{}", ans);
    }
    println!("");
}