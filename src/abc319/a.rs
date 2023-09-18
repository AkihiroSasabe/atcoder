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
        s: String
    }
    let ss: &str = &s;
    let ans = match ss {
        "tourist" => 3858,
        "ksun48" => 3679,
        "Benq" => 3658,
        "Um_nik" => 3648,
        "apiad" => 3638,
        "Stonefeang" => 3630,
        "ecnerwala" => 3613,
        "mnbvmar" => 3555,
        "newbiedmy" => 3516,
        "semiexp" => 3481,
        _ => 0
    };
    println!("{}", ans);
    

}

