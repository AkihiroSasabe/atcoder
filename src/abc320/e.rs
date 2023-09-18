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
    // 2023-09-18 11:05-11:35
    input! {
        n: usize,
        m: usize,
    }
    // 素麺が来る時刻
    let mut t = vec![];
    // 素麺の量
    let mut w = vec![];
    // 退場時間
    let mut s = vec![];
    for i in 0..m {
        input!{
            t_i: usize,
            w_i: usize,
            s_i: usize,
        }
        t.push(t_i);
        w.push(w_i);
        s.push(s_i);
    }

    // ヒープで片付けるべきでは?
    // (- 食べる人)を要素とするヒープ (番号が若い順にpopされる)
    let mut heap = BinaryHeap::new();
    for i in 0..n {
        heap.push((-1) *  i as isize);
    }

    // ans[i] := 人iが素麺を食べた量
    let mut ans = vec![0; n];

    // 退場者リスト (- 復活時刻、- 食べた人)
    let mut outer = BinaryHeap::new();

    for i in 0..m {
        // 復活させる処理
        loop {
            if let Some((m_hukkatu_time, m_eater)) = outer.pop() {
                let hukkatu_time = ((-1) * m_hukkatu_time) as usize;
                if hukkatu_time <= t[i] {
                    // 復活
                    heap.push(m_eater);
                }
                else {
                    // 復活しないのでもとに戻す
                    outer.push((m_hukkatu_time, m_eater));
                    break
                }
            }
            else {
                break
            }
        }
        if let Some(m_eater) = heap.pop() {
            let eater = ((-1) * m_eater) as usize;
            ans[eater] += w[i];
            let hukkatu_time = t[i] + s[i];
            let m_hukkatu_time = (-1) * hukkatu_time as isize;
            outer.push((m_hukkatu_time, m_eater));
        }
    }

    for i in 0..n {
        println!("{}", ans[i]);
    }

}