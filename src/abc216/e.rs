#![allow(dead_code, unused_imports)]
use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::BTreeMap;
use std::hash;
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::collections::HashSet;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    // 2024-02-08 19:14-21:12 (1h58min)
    
    input! {
        n: usize,
        mut k: usize,
        a: [usize; n],
    }

    // k回乗れる
    let (ranked_array,sorted_array, original_to_rank) = rank_array(&a);

    let mut rev_array = ranked_array.clone();
    rev_array.sort();
    rev_array.reverse();


    let mut counts = vec![0; n];
    for i in 0..n {
        counts[ranked_array[i]] += 1;
    }
    let mut revs = vec![];
    let mut seen = HashSet::new();
    for r in rev_array {
        if seen.contains(&r) {
            continue
        }
        else {
            seen.insert(r);
            revs.push(r);
        }
    }

    let mut ans = 0;
    let mut num = 0;

    for i in 0..revs.len()-1 {
        // println!("k = {:?}, ans = {ans}", k);
        
        let r = revs[i];
        let nr = revs[i+1];

        // 同順位の個数
        let count = counts[r];
        num += count;

        let val = sorted_array[r];
        let nval = sorted_array[nr];
        
        // 100, 99, 98, 97,    nv=96
        if (val - nval) * num <= k {
            // k が余っているとき

            k -= (val - nval) * num;
            // 等差数列の和
            ans += (val + nval + 1) * (val - nval) / 2 * num;
        }
        else {
            // k が 0になってしまうとき
            // k = 9
            // num = 4
            // down = 2

            // 100,99,     98

            let down = k / num;
            if val > down {
                // kを全部消費できるとき
                let nval = val - down;
                ans += (val + nval + 1) * down / 2 * num;
                ans += (k % num)* nval;
                k = 0;
                // println!("1 nval = {:?}", nval);
                println!("{}", ans);
                return
            }
            // else {
            //     // kを全部消費できないときは、考えなくて良い気がする。
            //     // k = 9
            //     // num = 4
            //     // down = 2
                
            //     // kを全部消費できないかもしれないとき
            //     let nval = 1;
            //     // println!("2 nval = {:?}", nval);
            //     let down = val - nval;
            //     ans += (val + 1 + nval) * down / 2 * num;
            //     ans += n * nval;
            //     println!("{}", ans);
            //     return
            // }
        }
    }
    // println!("--k = {:?}, ans = {ans}", k);

    
    if k != 0 {
        let r = revs[revs.len()-1];

        let count = counts[r];
        num += count;

        let val = sorted_array[r];
        let down = k / num;
        if val >= down {
            // k を全部消費できるとき
            let last_val = 1 + val - down;
            ans += (val + last_val) * down / 2 * num;
            // ans += (val + 1 + val - down) * down / 2 * num;
            k -= down * num;
            ans += (val - down) * k;
        }
        else {
            // k を全部消費できないとき
            // 0,1,2,3
            // 0,1,2,3,4
            ans += val * (val + 1) / 2 * num;
        }
    }
    println!("{}", ans);



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
