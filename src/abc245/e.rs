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
    // 2023-12-08 11:30-12:12 (42min) 自力で考察してgive up
    // 2023-12-08 12:12-12:36 (24min) 公式の解説を見てac
    // total 66min
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        b: [usize; n],
        c: [usize; m],
        d: [usize; m],
    }

    if n > m {
        println!("No");
        return;
    }

    let mut hwk = vec![];
    for i in 0..n {
        hwk.push([a[i], b[i], 0]);
    }
    for i in 0..m {
        hwk.push([c[i], d[i], 1]);
    }
    hwk.sort();
    hwk.reverse();

    let mut num_match = 0;
    let mut set = BTreeSet::new();
    for i in 0..n+m {
        let h = hwk[i][0];
        let w = hwk[i][1];
        let k = hwk[i][2];

        // 箱なら追加
        if k == 1 {
            // index を付与することで、multiset (同一キーを識別した集合) として扱える。
            set.insert((w, i));
        }
        // チョコなら、現在使用可能な箱の中で、一番幅が小さいものを選択。
        else {
            let mut exist_flag = false;
            let mut v_temp = (0, 0);
            if let Some(v) = set.range((w,0)..).next() {
                num_match += 1;
                exist_flag = true;
                v_temp = *v;
            }
            if exist_flag {
                set.remove(&v_temp);
            }
        }
    }

    if num_match == n {
        println!("Yes");
    }
    else {
        println!("No");
    }

}