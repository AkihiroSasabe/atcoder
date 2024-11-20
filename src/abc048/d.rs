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
    // 2024-11-19 20:33-21:11 (38min, 解説ac)
    // 参考: https://drken1215.hatenablog.com/entry/2019/02/02/181600
    input! {
        s: Chars
    }
    // 高橋君が先手

    // sから両端以外の文字をひとつ取り除く。 
    // ただし、その文字を取り除くことで、 s の中に同一の文字が隣り合う箇所ができる場合、その文字を取り除くことはできない。

    // s の中に同一の文字が隣り合う箇所はない。

    // 状態が多すぎる。。

    // パリティ（偶奇性）に注目
    // abcab
    // abab <- 両端が異なれば、偶数で回されると負け
    //
    // adefa
    // afa <- 両端が等しければ、奇数で回されると負け
    let tail = s.len()-1;
    if (s[0] == s[tail]) ^ (s.len() % 2 == 0) {
        println!("Second");
    }
    else {
        println!("First");
    }


}