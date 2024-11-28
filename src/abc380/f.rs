#![allow(dead_code, unused_imports)]
use proconio::{input, marker::Usize1};
use itertools::{all, Itertools};
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
    // 2024-11-17 Sun. 19:58-21:00 (1h2m)
    // 2024-11-18 Mon. 12:38-12:56 (18min. 解説見た。)
    // 2024-11-18 Mon. 19:00-20:02 (1h2min, Debug)
    // Total 2h22min
    input! {
        n: usize,
        m: usize,
        l: usize,
        a: [usize; n], // tkhs
        b: [usize; m], // aoki
        c: [usize; l], // ba
    }

    let sum = n + m + l;

    let mut pow4: Vec<usize> = vec![1; sum+1];
    for i in 1..sum+1 {
        pow4[i] = 4 * pow4[i-1];
    }
    
    let mut values = vec![];
    let mut state = 0;
    for i in 0..n {
        values.push(a[i]);
        state += 1 * pow4[i];
    }
    for i in 0..m {
        values.push(b[i]);
        state += 2 * pow4[n+i];
    }
    for i in 0..l {
        values.push(c[i]);
        state += 3 * pow4[n+m+i];
    }
    let mut seen = BTreeMap::new();
    let ans = dfs(true, state, &mut seen, &values, sum, &pow4);
    if ans {
        println!("Takahashi");
    }
    else {
        println!("Aoki");
    }

    // let mut a_set: BTreeMap<usize, usize> = BTreeMap::new();
    // for i in 0..n {
    //     a_set.add_value_by_1(a[i]);
    // }

    // let mut b_set = BTreeMap::new();
    // for i in 0..m {
    //     b_set.add_value_by_1(b[i]);
    // }

    // let mut c_set = BTreeMap::new();
    // for i in 0..l {
    //     c_set.add_value_by_1(c[i]);
    // }

    // // tkhs -> aoki
    // let ans = dfs_my_tle(true, &mut a_set, &mut b_set, &mut c_set);
    // if ans {
    //     println!("Takahashi");
    // }
    // else {
    //     println!("Aoki");
    // }
}

fn dfs(is_tkhs: bool, state: usize, seen: &mut BTreeMap<(bool, usize), bool>, values: &Vec<usize>, sum: usize, pow4: &Vec<usize>) -> bool {
    // 状態数: 2 * 3^(N+M+L) // 高橋と自分のターンの2通り x N+M+L枚のカードの居場所
    // 遷移: (N+M+L)^2 (手札から何を出すか x 山札から何を引くか)
    // 2 * 3^(N+M+L) * (N+M+L)^2 <= 2 * (3^12) * (12^2) = 153_055_008 ギリ間に合いそう。

    let mut is_win = false;
    for i in 0..sum {
        for j in 0..sum {
            if i == j {continue}
            // i を送る
            // j をもらう
            let src_pos = (state / pow4[i]) % 4;

            let trg_pos = (state / pow4[j]) % 4;

            if trg_pos != 3 {continue}
            if is_tkhs {
                if src_pos != 1 {continue;}
            }
            else {
                if src_pos != 2 {continue;}
            }

            let src = values[i];
            let trg = values[j];
            // println!("src = {src}, src_pos = {}, trg = {trg}, trg_pos = {:?}", src_pos, trg_pos);

            let mut next_state;
            if src > trg {
                // 貰える
                if is_tkhs {
                    // next_state = state + (3 - 1) * pow4[i] + (1 - 3) * pow4[j];
                    next_state = state + (3 - 1) * pow4[i] - 2 * pow4[j];
                }
                else {
                    // next_state = state + (3 - 2) * pow4[i] + (2 - 3) * pow4[j];
                    next_state = state + (3 - 2) * pow4[i] - pow4[j];
                }
            }
            else {
                // 貰えない、
                if is_tkhs {
                    next_state = state + (3 - 1) * pow4[i];
                }
                else {
                    next_state = state + (3 - 2) * pow4[i];
                }
            }
            let res;
            if let Some(&val) = seen.get(&(!is_tkhs, next_state)) {
                res = val;
            }
            else {
                res = dfs(!is_tkhs, next_state, seen, values, sum, pow4);
            }
            if !res {
                // 相手が負けなら、自分は勝ち
                is_win = true;
                seen.insert((is_tkhs, state), is_win);
                return is_win
            }
        }
    }

    seen.insert((is_tkhs, state), is_win);
    return is_win

}


fn dfs_my_tle(is_tkhs: bool, a_set: &mut BTreeMap<usize, usize>, b_set: &mut BTreeMap<usize, usize>, c_set: &mut BTreeMap<usize, usize>) -> bool {

    if is_tkhs {
        let mut is_tkhs_win = false;
        let a_set_clone: BTreeMap<usize, usize> = a_set.clone();
        for (ai, _) in a_set_clone {
            if let Some((&ci, _)) = c_set.range(..ai).rev().next() {
                a_set.subtract_value_by_1(ai);
                a_set.add_value_by_1(ci);
                
                c_set.add_value_by_1(ai);
                c_set.subtract_value_by_1(ci);

                let temp_res = dfs_my_tle(!is_tkhs, a_set, b_set, c_set);

                a_set.add_value_by_1(ai);
                a_set.subtract_value_by_1(ci);

                c_set.subtract_value_by_1(ai);
                c_set.add_value_by_1(ci);
                if temp_res {
                    is_tkhs_win = true;
                    println!("Takahashi----");
                    println!("a_set = {:?}", a_set);
                    println!("c_set = {:?}", c_set);
                    println!("a: {ai} <-> c: {ci}");
                    break
                }
            }
            else {
                a_set.subtract_value_by_1(ai);
                // a_set.add_value_by_1(ci);
                
                c_set.add_value_by_1(ai);
                // c_set.subtract_value_by_1(ci);

                let temp_res = dfs_my_tle(!is_tkhs, a_set, b_set, c_set);

                a_set.add_value_by_1(ai);
                // a_set.subtract_value_by_1(ci);

                c_set.subtract_value_by_1(ai);
                // c_set.add_value_by_1(ci);
                if temp_res {
                    is_tkhs_win = true;
                    println!("Takahashi----");
                    println!("a_set = {:?}", a_set);
                    println!("c_set = {:?}", c_set);
                    println!("a: {ai} -> c");
                    break
                }
            }
        }
        return is_tkhs_win
    }
    else {
        let mut is_tkhs_win = true;
        let b_set_clone = b_set.clone();
        for (ai, _) in b_set_clone {
            if let Some((&ci, _)) = c_set.range(..ai).rev().next() {
                b_set.subtract_value_by_1(ai);
                b_set.add_value_by_1(ci);
                
                c_set.add_value_by_1(ai);
                c_set.subtract_value_by_1(ci);

                let temp_res = dfs_my_tle(!is_tkhs, a_set, b_set, c_set);

                b_set.add_value_by_1(ai);
                b_set.subtract_value_by_1(ci);

                c_set.subtract_value_by_1(ai);
                c_set.add_value_by_1(ci);
                if !temp_res {
                    is_tkhs_win = false;
                    println!("Aoki----");
                    println!("b_set = {:?}", b_set);
                    println!("c_set = {:?}", c_set);
                    println!("b: {ai} <-> c: {ci}");
                    break
                }
            }
            else {
                b_set.subtract_value_by_1(ai);
                // b_set.add_value_by_1(ci);
                
                c_set.add_value_by_1(ai);
                // c_set.subtract_value_by_1(ci);

                let temp_res = dfs_my_tle(!is_tkhs, a_set, b_set, c_set);

                b_set.add_value_by_1(ai);
                // b_set.subtract_value_by_1(ci);

                c_set.subtract_value_by_1(ai);
                // c_set.add_value_by_1(ci);
                if !temp_res {
                    is_tkhs_win = false;
                    println!("Aoki----");
                    println!("b_set = {:?}", b_set);
                    println!("c_set = {:?}", c_set);
                    println!("b: {ai} -> c");
                    break
                }
            }
        }
        return is_tkhs_win
    }
}


/// BTreeMapの value を操作するトレイト
/// b_tree := {key: 要素, value: 格納した要素の個数} をイメージ
pub trait ValueManipulator {
    fn subtract_value(&mut self, key: Self::Key, value: usize);
    fn subtract_value_by_1(&mut self, key: Self::Key);
    fn add_value(&mut self, key: Self::Key, value: usize);
    fn add_value_by_1(&mut self, key: Self::Key);

    type Key; // 関連型（ Associated Type ）を宣言
}
impl<T: std::cmp::Ord> ValueManipulator for BTreeMap<T, usize> {
    fn subtract_value(&mut self, key: T, value: usize) {
        // B木からkeyに対応する値を減算する

        // B木に key が存在しないなら何もしない
        if !self.contains_key(&key) {return}
        if self[&key] <= value {
            // B木のkeyの個数がvalue個以下なら key ごと消去する
            self.remove(&key);
        }
        else {
            // B木のkeyの個数をvalue個減らす
            *self.get_mut(&key).unwrap() -= value;
        }
    }
    fn subtract_value_by_1(&mut self, key: T) {
        // B木からkeyに対応する値を1だけ減算する
        self.subtract_value(key, 1);
    }
    fn add_value(&mut self, key: T, value: usize) {
        // keyに対応する値を加算する
        *self.entry(key).or_insert(0_usize) += value;
    }
    fn add_value_by_1(&mut self, key: T) {
        // keyに対応する値を1だけ加算する
        self.add_value(key, 1);
    }
    // たぶん型エイリアスを定義 https://rs.nkmk.me/rust-type-alias/
    type Key = T; // トレイト ValueManipulator 内での Key という関連型を具体的な型 T に紐付けるための宣言
}