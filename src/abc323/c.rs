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
        m: usize,
        a: [usize; m],
        s: [Chars; n]
    }
    // n人、m問
    let mut cums = vec![vec![0; m]; n];
    let mut mikaitos = vec![vec![]; n];

    let mut tokutens = vec![0; n];
    for i in 0..n {
        tokutens[i] += i + 1;
        for j in 0..m {
            if s[i][j] == 'o' {
                tokutens[i] += a[j];
            }
            else {
                mikaitos[i].push(a[j]);
            }
        }
        mikaitos[i].sort();
        mikaitos[i].reverse();
    }
    // println!("tokutens = {:?}", tokutens);
    // println!("mikaitos = {:?}", mikaitos);
    let mut cum_mikaitos = vec![];
    for i in 0..n {
        cum_mikaitos.push(vec![0; mikaitos[i].len()]);
        for j in 0..mikaitos[i].len() {
            if j == 0 {
                cum_mikaitos[i][0] = mikaitos[i][0];
            }
            else {
                cum_mikaitos[i][j] = cum_mikaitos[i][j-1] + mikaitos[i][j];
            }
        }
    }


    // let mut diff = vec![0 as isize; n];
    let mut btree: BTreeSet<Vec<isize>> = BTreeSet::new();
    for i in 0..n {
        btree.insert(vec![- (tokutens[i] as isize), i as isize]);
    }

    for i in 0..n {
        btree.remove(&vec![- (tokutens[i] as isize), i as isize]);

        let min_of_btree = btree.iter().next().unwrap();

        let max_point = - min_of_btree[0];
        let max_index = min_of_btree[1];

        if tokutens[i] as isize > max_point {
            println!("0");
        }
        else {
            let diff = (1 + max_point - (tokutens[i] as isize)) as usize;
            // println!("diff = {:?}", diff);
            let ind = cum_mikaitos[i].lower_bound(&diff);
            println!("{}", ind + 1);
        }
        btree.insert(vec![- (tokutens[i] as isize), i as isize]);
    }

    

}