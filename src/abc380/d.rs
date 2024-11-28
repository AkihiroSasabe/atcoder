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
    input! {
        s: Chars,
        q: usize,
        k: [usize; q]
    }

    for i in 0..q {
        let n = s.len();
        let mut ki = k[i] - 1;
        let mut d = ki / n;
        let mut r = ki % n;
        // 1_152_921_504_606_846_976
        // dブロック目
        let mut count = 0;
        for j in (0..64).rev() {
            if d >= 1 << j {
                count += 1;
            }
            d = d % (1 << j);
        }
        let ans;
        if count % 2 == 0 {
            // そのまま
            ans = s[r];
        }
        else {
            // 小文字を大文字に、大文字を小文字にする操作 https://x.com/aplysiaSheep/status/1857782849841279145
            ans = ((s[r] as u8) ^ 32) as char;

            // if s[r].is_lowercase() {
            //     ans = ((s[r] as u8) - 32) as char;
            // }
            // else {
            //     ans = ((s[r] as u8) + 32) as char;
            // }
        }
        print!("{} ", ans);
    }


}