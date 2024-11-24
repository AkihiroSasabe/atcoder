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
        n: usize,
        a: [usize; n]
    }


    // 尺取法か？
    // 起点を偶数にするか、奇数にするか。。。
    let mut ans = 0;
    let mut set = BTreeSet::new(); // 尺取法の管理。index
    for i in 0..n {
        // 偶数起点にしよう
        if i % 2 != 0 {continue}
        if i+1 > n-1 {continue;}
        // if 2*i > n-1 {continue}
        // if 2*i+1 > n-1 {continue}
        if set.contains(&i) {continue}
        // 起点開始
        if a[i] == a[i+1] {
            let mut set2 = BTreeSet::new(); // 2個あるかの、管理
            set2.insert(a[i]);

            let mut queue = VecDeque::new();
            queue.push_back(a[i]);
            
            let mut index = i+2;
            ans = max(ans, queue.len()*2);

            while index+1 < n {
                // println!("index = {:?}", index);
                ans = max(ans, queue.len()*2);
                if a[index] == a[index+1] {
                    set.insert(index);

                    if !set2.contains(&a[index]) {
                        // 新規
                        set2.insert(a[index]);
                        queue.push_back(a[index]);
                        ans = max(ans, queue.len()*2);
                    }
                    else {
                        // すでにいる
                        while queue.len() != 0 {
                            let v = queue.pop_front().unwrap();
                            set2.remove(&v);
                            if v == a[index] {
                                set2.insert(a[index]);
                                queue.push_back(a[index]);
                                break
                            }
                        }
                    }
                }
                else {
                    break;
                }
                index += 2;
            }
        } 
    }


    let mut set = BTreeSet::new(); // 尺取法の管理。index
    for i in 0..n {
        // 奇数起点にしよう
        if i % 2 != 1 {continue}
        if i+1 > n-1 {continue;}
        // if 2*i > n-1 {continue}
        // if 2*i+1 > n-1 {continue}
        if set.contains(&i) {continue}
        // 起点開始
        if a[i] == a[i+1] {
            let mut set2 = BTreeSet::new(); // 2個あるかの、管理
            set2.insert(a[i]);

            let mut queue = VecDeque::new();
            queue.push_back(a[i]);
            
            let mut index = i+2;
            ans = max(ans, queue.len()*2);

            while index+1 < n {
                ans = max(ans, queue.len()*2);
                if a[index] == a[index+1] {
                    set.insert(index);

                    if !set2.contains(&a[index]) {
                        // 新規
                        set2.insert(a[index]);
                        queue.push_back(a[index]);
                        ans = max(ans, queue.len()*2);
                    }
                    else {
                        // すでにいる
                        while queue.len() != 0 {
                            let v = queue.pop_front().unwrap();
                            set2.remove(&v);
                            if v == a[index] {
                                set2.insert(a[index]);
                                queue.push_back(a[index]);
                                break
                            }
                        }
                    }
                }
                else {
                    break;
                }
                index += 2;
            }
        } 
    }

    println!("{}", ans);


}