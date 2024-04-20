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
    // 2024-04-16 10:52-11:22 (30min)
    // 思いつき -11:14 (~22min)
    // 2024-04-17 19:34-22:00 (2h26m)
    // 2024-04-18 19:10-19:40 (30min)
    // 2024-04-19 AC
    // 3 h 48 min
    input! {
        n: usize,
        q: usize,
    }
    let mut a = vec![];
    let mut b = vec![]; 
    for i in 0..n {
        input!{
            ai: usize,
            bi: usize,
        }
        a.push(ai);
        b.push(bi-1); // 所属 b[園児] = 幼稚園
    }
    let mut c = vec![];
    let mut d = vec![];
    for i in 0..q {
        input!{
            ci: usize,
            di: usize,
        }
        c.push(ci-1);
        d.push(di-1);
    }

    let num_max_school = 200_001;
    // let num_max_school = 5; // Debug用
    let mut btrees = vec![BTreeMap::new(); num_max_school];
    for hito in 0..n {
        let rate = a[hito];
        let pos = b[hito];
        btrees[pos].entry(rate).or_insert(HashSet::new()).insert(hito);
    }

    // btree := 全幼稚園の最大レートを格納
    // key: rate, val: HashSet<pos>
    let mut btree = BTreeMap::new();

    // max_rates[幼稚園] = その幼稚園の最大レート
    let INF = 1 << 60;
    let mut max_rates = vec![INF; num_max_school];
    let mut ans = INF;
    for pos in 0..num_max_school {
        if let Some((&max_rate, humans)) = btrees[pos].iter().rev().next() {
            let hito = humans.iter().next().unwrap();
            max_rates[pos] = max_rate;
            btree.entry(max_rate).or_insert(HashSet::new()).insert(pos);
            ans = min(ans, max_rate);
        }
    }

    // let mut shozoku = vec![0; n];
    for i in 0..q {
        // println!("btrees = {:?}", btrees);
        // println!("btree = {:?}", btree);
        // println!("b = {:?}", b);

        let hito = c[i];
        let rate = a[hito];
        let pos_s = b[hito];
        let pos_t = d[i];
        // let pre_min_rate = btree.iter().next().unwrap();
        // println!("hito = {:?}", hito);
        // println!("rate = {:?}", rate);
        // println!("pos_s = {:?}", pos_s);

        // (1)btrees, (2)b, (3)btree を更新する
        // 移動元から削除する。
        btrees[pos_s].get_mut(&rate).unwrap().remove(&hito);

        let mut is_src_deleted = false;
        // btree @ pos_s が更新されるのはどんなときか?
        if btree.contains_key(&rate) {
            if btree.get(&rate).unwrap().contains(&pos_s) {
                btree.get_mut(&rate).unwrap().remove(&pos_s);
                is_src_deleted = true;
                // println!("deleted!!! rate={rate}, pos_s={pos_s}, hito={hito}");
                if btree.get_mut(&rate).unwrap().len() == 0 {
                    btree.remove(&rate);
                }
            }

        }        
        if btrees[pos_s].get(&rate).unwrap().len() == 0 {
            btrees[pos_s].remove(&rate);
        }
        if is_src_deleted {
            // btreeを削除したら、次点を入れないといけない。
            if let Some((n_rate, enjis)) = btrees[pos_s].iter().rev().next() {
                // println!("ready to add, n_rate={n_rate}, enjis={:?}", enjis);
                if enjis.len() != 0 {
                    let n_hito = enjis.iter().next().unwrap();
                    btree.entry(*n_rate).or_insert(HashSet::new()).insert(b[*n_hito]);
                    // println!("added!! n_rate={n_rate}, pos_s={pos_s}, n_hito={n_hito}");
                }
            }
        }
        // 移動先のbtreeを修正
        if btrees[pos_t].len() != 0 {
            let (max_t_rate, _enjis) = btrees[pos_t].iter().rev().next().unwrap();
            if *max_t_rate < rate {
                btree.entry(rate).or_insert(HashSet::new()).insert(pos_t);
                btree.get_mut(max_t_rate).unwrap().remove(&pos_t);
                if btree.get(max_t_rate).unwrap().len() == 0 {
                    btree.remove(max_t_rate);
                }
            }
        }
        else {
            btree.entry(rate).or_insert(HashSet::new()).insert(pos_t);
        }

        btrees[pos_t].entry(rate).or_insert(HashSet::new()).insert(hito);

        let (&ans, _) = btree.iter().next().unwrap();
        println!("{}", ans);

        // 移動先に追加
        b[hito] = pos_t;

    }
    // println!("btrees = {:?}", btrees);
    // println!("btree = {:?}", btree);
    // println!("b = {:?}", b);
}

