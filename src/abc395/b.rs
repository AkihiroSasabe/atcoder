#![allow(dead_code, unused_imports)]
use proconio::{input, marker::{Usize1, Isize1}};
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
use num::{BigUint, ToPrimitive};
use num_bigint::ToBigUint;
fn main() {
    input! {
        n: usize
    }

    let mut s = vec![vec!['#'; n]; n];
    for i in 0..n {
        let j = n-1-i;
        // println!("(i,j) = {:?}", (i,j));
        if i > j {continue}

        if i % 2 == 0 {
            draw(&mut s, i,j,'#');
        }
        else {
            draw(&mut s, i,j,'.');
        }
    }
    for i in 0..n {
        for j in 0..n {
            print!("{}", s[i][j]);
        }
        println!("");
    }

}

fn draw(s: &mut Vec<Vec<char>>, i: usize, j: usize, ch: char) {
    for y in i..j+1 {
        for x in i..j+1 {
            s[y][x] = ch;
        }
    }
}