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
    input! {
        n: usize,
        s: Chars,
        c: [usize; n]
    }

    let mut temp = 0;
    // 0から始まる場合
    if s[0] != '0' {
        temp += c[0];
    }
    for i in 1..n {
        if i % 2 == 1 {
            if s[i] != '0' {
                temp += c[i];
            }
        }
        else {
            if s[i] != '1' {
                temp += c[i];
            }
        }
    }

    let mut ans = temp;
    for i in 1..n-1 {
        if i % 2 == 1 {
            // 1 じゃないと駄目
            if s[i] == '0' {
                temp += c[i];
            }
            else if s[i] == '1' {
                temp -= c[i];
            }
        }
        else if i % 2 == 0 {
            // 0じゃないと駄目
            if s[i] == '0' {
                temp -= c[i];
            }
            else if s[i] == '1' {
                temp += c[i];
            }
        }
        ans = min(ans, temp);
    }

    // 1から始まる場合
    temp = 0;
    if s[0] == '0' {
        temp += c[0];
    }
    for i in 1..n {
        // 1じゃないと駄目
        if i % 2 == 1 {
            if s[i] == '0' {
                temp += c[i];
            }
        }
        else if i % 2 == 0 {
            if s[i] == '1' {
                temp += c[i];
            }
        }
    }
    ans = min(ans, temp);


    for i in 1..n-1 {
        if i % 2 == 1 {
            // 0じゃないと駄目
            if s[i] == '0' {
                temp -= c[i];
            }
            else if s[i] == '1' {
                temp += c[i];
            }
        }
        else if i % 2 == 0 {
            // 1 じゃないと駄目
            if s[i] == '0' {
                temp += c[i];
            }
            else if s[i] == '1' {
                temp -= c[i];
            }
        }
        ans = min(ans, temp);
    }
    println!("{}", ans);
    



}