#![allow(dead_code, unused_imports)]
use proconio::{input, marker::Usize1};
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
    // 2024-11-28 19:26-19:36 (10min)
    input! {
        s: Chars
    }
    // 貪欲に、勝てるときに勝っとくか?
    // あえて引き分けのときは引き分けでためておくか?

    // 条件：(今までにパーを出した回数)≦(今までにグーを出した回数)

    let n = s.len();
    let mut ans = 0;
    let mut num_g = 0;
    let mut num_p = 0;

    // gp
    // gp
    // ppp
    // gpg

    for i in 0..n {
        if s[i] == 'g' {
            if num_p + 1 <= num_g {
                ans += 1;
                num_p += 1;
            }
            else {
                num_g += 1;
            }
        }
        else {
            if num_p + 1 <= num_g {
                num_p += 1;
            }
            else {
                ans -= 1;
                num_g += 1;
            }
        }
    }
    println!("{}", ans);



    
}