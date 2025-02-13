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
        n: usize,
        w: usize,
    }

    let mut xs = vec![];
    let mut ys = vec![];
    let mut xx = vec![vec![]; w];
    for i in 0..n {
        input!{
            xsi: Usize1,
            ysi: Usize1,
        }
        xx[xsi].push((ysi, i));
        xs.push(xsi);
        ys.push(ysi);
    }
    input! {
        q: usize
    }
    let mut ts = vec![];
    let mut aas = vec![];
    // let mut tas = vec![];
    for i in 0..q {
        input!{
            tsi: usize,
            asi: Usize1,
        }
        ts.push(tsi);
        aas.push(asi);
        // tas.push((tsi, asi, i));
    }

    // 各ブロックが消える時刻は?



    // dans[i] := ブロックi の段位
    let mut dans = vec![0; n];


    // 段i が w列、全部揃っているかの判定もいる
    let mut is_complete_dan = vec![BTreeSet::new(); n];

    // hh[i] := 下からi段目の奴の初期位置の高さの最大
    let mut hh = vec![0; n];
    for x in 0..w {
        xx[x].sort();
        for i in 0..xx[x].len() {
            let (y, ind) = xx[x][i];
            hh[i] = max(hh[i], y);
            dans[ind] = i;
            is_complete_dan[i].insert(x);
        }
    }


    // ぜんぶ下の段のやつ...　考慮していない
    あああ　

    // println!("dans = {:?}", dans);

    for i in 0..q {
        let ti = ts[i];
        let ai = aas[i];

        // let x = xs[ai];
        // let y = ys[ai];

        let dan = dans[ai];
        let deth_time = hh[dan] + 1;

        if deth_time <= ti && is_complete_dan[dan].len() == w {
            println!("No");
        }
        else {
            println!("Yes");
        }
    }



}
