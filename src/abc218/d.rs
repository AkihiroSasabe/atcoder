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
    // 2024-01-31 21:04-21:57 (53min)
    input! {
        n: usize
    }
    let mut x = vec![];
    let mut y = vec![];
    for i in 0..n {
        input! {
            xi: usize,
            yi: usize,
        }
        x.push(xi);
        y.push(yi);
    }
    let (ranked_x, _, _) = rank_array(&x);
    let (ranked_y, _, _) = rank_array(&y);

    let mut exist = vec![vec![n; n]; n];
    let mut rank_yx = vec![];
    for i in 0..n {
        let yi = ranked_y[i];
        let xi = ranked_x[i];
        exist[yi][xi] = i;
        rank_yx.push(vec![yi, xi]);
    }
    // println!("rank_yx = {:?}", rank_yx);


    // 若いほど、左上にいる。
    let mut set = HashSet::new();
    for i in 0..n {
        for j in i+1..n {
            let yi = rank_yx[i][0];
            let xi = rank_yx[i][1];

            let yj = rank_yx[j][0];
            let xj = rank_yx[j][1];

            if !(xi !=xj && yi != yj) {continue}

            let v = exist[yj][xi];
            let w = exist[yi][xj];
            
            if v != n && w != n {
                let mut aaa = vec![i, j, v, w];
                aaa.sort();
                set.insert(aaa);
            }
        }
    }
    // println!("set = {:?}", set);
    println!("{}", set.len());


}

fn rank_array<T: Ord + std::hash::Hash + Clone + Copy>(array: &Vec<T>) -> (Vec<usize>, Vec<T>, HashMap<T, usize>) {
    // 配列を順位変換する関数 O(NlogN)
    // 要素の値を圧縮することを、目的として使うことを想定している。
    // Input: 
    //     array: 配列
    // Output: 
    //    ranked_array:     順位変換済み配列
    //    sorted_array:     ソート済みの配列(順位から元の値をマップさせる)
    //    original_to_rank: 元の値から順位を対応させるマップ
    // Example:
    // let array = vec![333, 111, 444, 111, 555, 999];
    // let (ranked_array, _, _) = rank_array(&array);
    // assert_eq!(ranked_array, vec![2, 0, 3, 0, 4, 5]);

    // 配列のサイズ
    let n = array.len();

    // B木<数列中に登場する値, 頻度>
    let mut btree: BTreeMap<T, usize> = BTreeMap::new();
    for i in 0..n {
        *(btree.entry(array[i]).or_insert(0)) += 1;
    }

    // 昇順ソート済みの、順位変換済み配列
    let mut sorted_rank_array = vec![];
    let mut rank = 0;
    for (k, frequency) in btree {
        for j in 0..frequency {
            sorted_rank_array.push(rank);
        }
        rank += frequency; // sorted_rank_array = [0, 0, 2, 3, 4, 5], 
        // ここを1にすると、隙間なくなる。
        // rank += 1; //sorted_rank_array = [0, 0, 1, 2, 3, 4], 
    }
    // println!("sorted_rank_array = {:?}, ", sorted_rank_array);

    // 順位から元の値をマップさせる
    let mut sorted_array = (*array).clone();
    sorted_array.sort();

    // 元の値から順位を対応させるマップ
    let mut original_to_rank: HashMap<T, usize> = HashMap::new();
    for i in 0..n {
        original_to_rank.insert(sorted_array[i], sorted_rank_array[i]);
    }

    // 元の順序の、順位変換済み配列
    let mut ranked_array: Vec<usize> = vec![];
    for i in 0..n {
        ranked_array.push(*(original_to_rank.get(&array[i]).unwrap()));
    }

    return (ranked_array, sorted_array, original_to_rank)
}
