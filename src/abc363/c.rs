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
    
    // let mut s: Vec<char> = vec!['b', 'a', 'a', 'c'];
    // // next_permutationで、全順列を探索したいなら、ソートする必要があることに注意
    // s.sort();
    // let mut count = 0;
    // loop {
    //     count += 1;
    //     // next_permutationは、辞書的に1個後ろの配列を返す。そのような配列が存在しなければfalseを返し、sを順列の先頭にする。
    //     // 事前にsort()しておけば、全順列を網羅できる。
    //     // https://docs.rs/superslice/latest/superslice/trait.Ext.html#tymethod.next_permutation
    //     let is_first = s.next_permutation();
    //     println!("is_first = {:?}, s = {:?}, count = {}", is_first, s, count);
    //     if !is_first {break}
    // }
    // return;

    // 'b', 'a', 'a', 'c'の順列は、4! / 2 = 4*3*2/2 = 12個ある。
    // is_first = true, s = ['a', 'a', 'c', 'b'], count = 1
    // is_first = true, s = ['a', 'b', 'a', 'c'], count = 2
    // is_first = true, s = ['a', 'b', 'c', 'a'], count = 3
    // is_first = true, s = ['a', 'c', 'a', 'b'], count = 4
    // is_first = true, s = ['a', 'c', 'b', 'a'], count = 5
    // is_first = true, s = ['b', 'a', 'a', 'c'], count = 6
    // is_first = true, s = ['b', 'a', 'c', 'a'], count = 7
    // is_first = true, s = ['b', 'c', 'a', 'a'], count = 8
    // is_first = true, s = ['c', 'a', 'a', 'b'], count = 9
    // is_first = true, s = ['c', 'a', 'b', 'a'], count = 10
    // is_first = true, s = ['c', 'b', 'a', 'a'], count = 11
    // is_first = false, s = ['a', 'a', 'b', 'c'], count = 12

    input! {
        n: usize,
        k: usize,
        mut s: Chars,
    }

    let mut ans: usize = 0;

    // next_permutationで、全順列を探索したいなら、ソートする必要があることに注意
    s.sort();
    loop {
        // println!("s = {:?}", s);

        let mut is_ok = true;
        for st in 0..n-k+1 {
            let mut is_kaibun = true;
            for i in 0..k/2 {
                if s[st + i] != s[st + k - 1 - i] {
                    is_kaibun = false;
                    break
                }
            }
            // 回分判定にスライスを使うと、早期break出来ないのと、cloneする必要があるので、遅い(842ms)
            // 回分判定にインデックスを使った場合は、55msec
            // let aaa: Vec<char> = s[st..st+k].to_vec();
            // let bbb: Vec<char> = s[st..st+k].iter().rev().cloned().collect();
            // let is_kaibun =  aaa == bbb;
            if is_kaibun {
                is_ok = false;
                break
            }
        }
        if is_ok {
            ans += 1;
        }
        let is_first = s.next_permutation();
        if !is_first {break}
    }
    println!("{}", ans);

    // コンテスト中の解法。next_permutationを使わなかったので、
    // HashSetを使って重複を判定した。ギリギリTLEせずにACできた。運が良かった。(1937ms)
    // // 10! = 3_628_800 = 3 * 10^6
    // let mut set = HashSet::new();
    // for perm in (0..n).permutations(n) {
    //     let mut is_ok = true;
    //     for st in 0..n-k+1 {
    //         let mut is_kaibun = true;
    //         for i in 0..k {
    //             if s[perm[st + i]] != s[perm[st + k - 1 - i]] {
    //                 is_kaibun = false;
    //                 break
    //             }
    //         }
    //         if is_kaibun {
    //             is_ok = false;
    //             break
    //         }
    //     }
    //     if is_ok {
    //         let mut temp = vec![];
    //         for pp in perm {
    //             temp.push(s[pp]);
    //         }
    //         set.insert(temp);
    //     }
    // }
    // println!("{}", set.len());



}