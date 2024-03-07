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
    // 2024-03-07 20:07-21:28 (1h21min)
    input! {
        s1: Chars,
        s2: Chars,
        s3: Chars,
    }

    // 全探索でいけそう
    let mut btree = BTreeSet::new();

    let n1 = s1.len();
    let n2 = s2.len();
    let n3 = s3.len();

    let mut ss1 = vec![];
    let mut ss2 = vec![];
    let mut ss3 = vec![];

    for i in 0..n1 {
        // btree.entry(s1[i]).or_insert(vec![]).push(i);
        let num = s1[i] as usize - 'a' as usize;
        ss1.push(num);
        btree.insert(num);
    }
    for i in 0..n2 {
        // btree.entry(s2[i]).or_insert(vec![]).push(i * 10);
        let num = s2[i] as usize - 'a' as usize;
        ss2.push(num);
        btree.insert(num);
    }
    for i in 0..n3 {
        // btree.entry(s3[i]).or_insert(vec![]).push(i * 100);
        let num = s3[i] as usize - 'a' as usize;
        ss3.push(num);
        btree.insert(num);
    }

    if btree.len() > 10 {
        println!("UNSOLVABLE");
        return
    }

    // apps[i] := 登場する文字のcast
    let mut apps = vec![];
    for &num in btree.iter() {
        apps.push(num);
    }

    let mut revs = vec![0; 26];
    for i in 0..apps.len() {
        revs[apps[i]] = i;
    }

    // println!("apps = {:?}", apps);
    // println!("revs = {:?}", revs);

    // どの文字に、どの数字を割り当てるか全探索
    // 順列全探索 計算量は10! = 362_880 = 4 * 10^5 なので余裕で間に合う
    for perm in (0..10).permutations(10) {
        // println!("perm = {:?}", perm);
        if let Some((n1, n2, n3)) = check(&apps, &revs, &ss1, &ss2, &ss3,  &perm) {
            // println!("----------");
            println!("{}", n1);
            println!("{}", n2);
            println!("{}", n3);
            return
        }
    }
    println!("UNSOLVABLE");


}


fn check(apps: &Vec<usize>, revs: &Vec<usize>, s1: &Vec<usize>, s2: &Vec<usize>, s3: &Vec<usize>, perm: &Vec<usize>) -> Option<(usize, usize, usize)> {
    // apps[i] := 登場する文字のcast
    // perm[i] := apps[i]の文字castに、対応させる数字
    // revs[c] := 文字c　がappsの何番目か?

    // 先頭が0じゃないか、チェック
    let ind1 = revs[s1[0]];
    let ind2 = revs[s2[0]];
    let ind3 = revs[s3[0]];
    if perm[ind1] == 0 || perm[ind2] == 0 || perm[ind3] == 0 {
        return None
    }

    let n1 = get(revs,  s1, perm);
    let n2 = get(revs,  s2, perm);
    let n3 = get(revs,  s3, perm);
    // println!("= = = = ");
    // println!("{}", n1);
    // println!("{}", n2);
    // println!("{}", n3);

    if n1 + n2 == n3 {
        Some((n1, n2, n3))
    }
    else {
        return None
    }


}

fn get(revs: &Vec<usize>, s1: &Vec<usize>, perm: &Vec<usize>) -> usize {
    let mut n1 = 0;
    let mut keta = 1;
    for i in (0..s1.len()).rev() {
        let ind = revs[s1[i]];
        // let ci = apps[ind];
        n1 += perm[ind] * keta;
        keta *= 10;
    }
    return n1
}

