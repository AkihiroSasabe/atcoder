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
    // 2024-04-14 18:48-19:56 (1h8min)
    // 2024-04-14 21:22-22:30 (1h8min)
    // 2024-04-15 20:25-21:12 (47min)
    // Total: 3h3min
    input! {
        a: [[isize; 3]; 3]
    }
    // 勝敗が決する譜面を順列全探索で求めてから、DFSで勝敗判定すればよい。

    // 全部塗ったときの、終了状態は9C5 = 126通り
    // 塗る過程も含めた塗り方は、順列全探索
    // 9! = 40,320 = 4*10^4 通り

    // 3連続塗って、途中で終了する8パターンを、毎回のループでチェック。3ターン目~9ターン目の全てでチェックすると、最悪
    // (8 * 7) * 9! ≒ 4*10^6
    // ただ、途中で早期打ち切りが可能なので、実際はもっと少ないでしょう。

    // 終了状態(126個)から、DFSで遷移していけばよさそう。

    // 3連続塗って勝敗が決する場合の塗った位置
    let r0 = vec![0,1,2];
    let r1 = vec![3,4,5];
    let r2 = vec![6,7,8];

    let c0 = vec![0,3,6];
    let c1 = vec![1,4,7];
    let c2 = vec![2,5,8];

    let x0 = vec![0,4,8];
    let x1 = vec![2,4,6];

    // ゲーム終了判定を 9 bits で表現
    let r0_mask = get_bit_mask(r0);
    let r1_mask = get_bit_mask(r1);
    let r2_mask = get_bit_mask(r2);
    let c0_mask = get_bit_mask(c0);
    let c1_mask = get_bit_mask(c1);
    let c2_mask = get_bit_mask(c2);
    let x0_mask = get_bit_mask(x0);
    let x1_mask = get_bit_mask(x1);
    // decode_bit_mask_to_2d_map(r0_mask);
    // decode_bit_mask_to_2d_map(r1_mask);
    // decode_bit_mask_to_2d_map(r2_mask);
    // decode_bit_mask_to_2d_map(c0_mask);
    // decode_bit_mask_to_2d_map(c1_mask);
    // decode_bit_mask_to_2d_map(c2_mask);
    // decode_bit_mask_to_2d_map(x0_mask);
    // decode_bit_mask_to_2d_map(x1_mask);
    // return;

    // 最終状態を記録する。(key: 高橋と青木のビットマスク、value: 高橋が勝ったか?)
    let mut fin_states = HashMap::new();
    for perm in (0..9).permutations(9) {
        // どっちが勝つか?
        // 塗ったマスク
        let mut mask_t = 0;
        let mut mask_a = 0;
        // 点数
        let mut point_t = 0;
        let mut point_a = 0;
        // 3連続塗って決着がつくか?
        let mut determined = false;
        for (i,&v) in perm.iter().enumerate() {
            let mut mask_selected = 0;
            if i % 2 == 0 {
                // 高橋の塗るターン
                mask_t |= (1 << v);
                mask_selected = mask_t;
                point_t += a[v / 3][v % 3];
            }
            else {
                // 青木の塗るターン
                mask_a |= (1 << v);
                mask_selected = mask_a;
                point_a += a[v / 3][v % 3];
            }
            if i < 2 {continue}

            // 途中終了条件
            if ((mask_selected & r0_mask) == r0_mask) || 
                ((mask_selected & r1_mask) == r1_mask) || 
                ((mask_selected & r2_mask) == r2_mask) || 
                ((mask_selected & c0_mask) == c0_mask) || 
                ((mask_selected & c1_mask) == c1_mask) || 
                ((mask_selected & c2_mask) == c2_mask) || 
                ((mask_selected & x0_mask) == x0_mask) || 
                ((mask_selected & x1_mask) == x1_mask) {

                fin_states.insert(mask_t + (mask_a << 9), (i % 2) == 0);
                determined = true;
                break
            }
        }
        
        // if mask & (1 << 9) - 1 != 0 {
        if !determined {
            // 全部塗って、ポイントで勝敗を決する場合
            let is_tkhs_winner = point_t > point_a;
            fin_states.insert(mask_t + (mask_a << 9), is_tkhs_winner);
        }
    }

    if dfs(0, 0, 0, 0, &fin_states) {
        println!("Takahashi");
    }
    else {
        println!("Aoki");
    }
}

fn dfs(mask: usize, depth: usize, mask_t: usize, mask_a: usize, fin_states:& HashMap<usize, bool>) -> bool {
    if let Some(is_tkhs_winner) = fin_states.get(&(mask_t + (mask_a << 9))) {
        return *is_tkhs_winner
    }

    for i in 0..9 {
        // すでに塗ってあったらスキップ
        if mask & (1 << i) != 0 {continue}

        let mut n_mask_t = mask_t;
        let mut n_mask_a = mask_a;
        if depth % 2 == 0 {
            n_mask_t |= (1 << i); 
        }
        else {
            n_mask_a |= (1 << i); 
        }

        let is_tkhs_winner = dfs(mask ^ (1 << i), depth + 1, n_mask_t, n_mask_a, fin_states);
        if depth % 2 == 0 {
            // 高橋のターンで、次の1手で高橋の勝ちがあったら、その時点で高橋の勝ち
            if is_tkhs_winner {
                return true
            }
        }
        else {
            // 青木のターンで、次の1手で青木の勝ちがあったら、その時点で青木の勝ち
            if !is_tkhs_winner {
                return false
            }
        }
    }

    if (depth % 2) == 0 {
        // 現在が高橋のターンで、次にどんな手を打っても高橋が勝てなければ青木の勝ち(false)
        return false
    }
    else {
        // 現在が青木のターンで、次にどんな手を打っても青木が勝てなければ高橋の勝ち(true)
        return true
    }
}


fn get_bit_mask(list: Vec<usize>) -> usize {
    // 9マスのうち、塗られたマスのインデックスをリストに格納されると、9ビットで返す

    let mut mask = 0;
    for i in list {
        mask |= (1 << i);
    }
    // println!("fin_mask = {:09b}", mask);

    return mask
}

fn decode_bit_mask_to_2d_map(mask: usize) {
    // デバッグ用関数。bit列をデコードして、2次元map上に描画
    let mut map = vec![vec![0; 3]; 3];
    for i in 0..9 {
        let y = i / 3;
        let x = i % 3;
        if mask & (1 << i) != 0 {
            map[y][x] = 1;
        }
    }
    println!("----");
    map.print_2d_array();
}

// デバッグ用に2次元配列をprintするトレイト
pub trait Print2DArray {
    fn print_2d_array(&self);
    fn print_2d_array_with_name(&self, name: &str);
    fn print_2d_array_transposed(&self);
    fn print_2d_array_transposed_with_name(&self, name: &str);
}
impl<T: std::fmt::Debug> Print2DArray for Vec<Vec<T>> {
    fn print_2d_array(&self) {
        for y in 0..self.len() {
            for x in 0..self[y].len() {
                print!("{:?} ", self[y][x]);
            }
            println!("");
        }
    }
    fn print_2d_array_with_name(&self, name: &str) {
        println!("-=-=-=-= {} -=-=-=-=", name);
        self.print_2d_array();
        println!("-=-=-=-=-=-=-=-=");
    }
    fn print_2d_array_transposed(&self) {
        for x in 0..self[0].len() {
            print!("{:02}: ", x);
            for y in 0..self.len() {
                print!("{:?} ", self[y][x]);
            }
            println!("");
        }
    }
    fn print_2d_array_transposed_with_name(&self, name: &str) {
        println!("-=-=-=-= transposed {} -=-=-=-=", name);
        self.print_2d_array_transposed();
        println!("-=-=-=-=-=-=-=-=");
    }
}