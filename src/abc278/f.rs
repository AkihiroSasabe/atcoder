#![allow(dead_code, unused_imports)]
use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    // 2023-10-26 (Thu.) 21:24-22:00 (36min)
    // 2023-10-28 (Sat.) 12:20-13:30 (1h10m)競プロフレンズのツイートチラ見して自分で実装
    // 2023-10-28 (Sat.) 18:40-19:13 (33min)
    // total 2h13min = 133min
    input! {
        n: usize,
        s: [Chars; n]
    }
    let mut graph = vec![vec![]; n];
    for i in 0..n {
        for j in i+1..n {
            if s[i][s[i].len()-1] == s[j][0] {
                graph[i].push(j);
            }
            if s[i][0] == s[j][s[j].len()-1] {
                graph[j].push(i);
            }
        }
    }
    // println!("graph = {:?}", graph);

    // 候補1
    // BFSで全探索?
    // 16! = 20_922_789_888_000 <- 厳しい
    
    // 候補2
    // 各単語が使用済みかで、状態を遷移
    // 2^16 = 65_536            <- なんか行けそう。6.5 * 10^4

    // 各頂点に訪問したかどうかは、2^16 = 65,536 通りの状態しかない。
    // 0の状態で、勝敗判定していけばいいかと。
    // dp[使用済み][これから選ぶ単語v] := これから選ぶ単語がvのとき、0(負け), 1(勝ち), 2(わからない)
    let mut dp = vec![vec![2; n]; 1 << n];
    
    // 全体の計算量は、O((N^2)*(2^N)) <= O(16*16*2^16) = O(2^24) = 16_777_216 = 1.6 * 10^7で間に合う
    for state in (0..(1 << n)).rev() {
        // state := 使用済み
        // v: これから選ぶ単語
        // next_v: 次に相手が選ぶ単語
        for v in 0..n {
            // もう既にvを使っていたら調べない
            if state & (1 << v) != 0 {continue}
            // このstateでvを選んだら勝てるのか? -> 相手がnext_vを1個も選べない または、選んだdp[next_state][next_v]が1個も1(win)じゃないなら、勝てる
            let mut win_flag = true;
            for i in 0..graph[v].len() {
                let next_v = graph[v][i];
                if state & (1 << next_v) == 0 {
                    // 相手がnext_vを選べるとき
                    let next_state = state | (1 << v);
                    if dp[next_state][next_v] == 1 {
                        // 自分がvを選んで、相手がnext_vを選んだときに相手が勝てるケースが1個でもあれば、自分は確定で負け
                        win_flag = false;
                    }
                }
            }
            if win_flag {
                dp[state][v] = 1;
            }
            else {
                dp[state][v] = 0;
            }
        }
    }

    // dp.print_2d_array_with_name("dp");

    let mut flag = false;
    for i in 0..n {
        if dp[0][i] == 1 {
            flag = true;
            break;
        } 
    }
    if flag {
        println!("First");
    }
    else {
        println!("Second");
    }
}

// デバッグ用に2次元配列をprintするトレイト
pub trait Print2DArray {
    fn print_2d_array(&self);
    fn print_2d_array_with_name(&self, name: &str);
    fn print_2d_array_transposed(&self);
    fn print_2d_array_transposed_with_name(&self, name: &str);
}
impl<T: std::fmt::Display> Print2DArray for Vec<Vec<T>> {
    fn print_2d_array(&self) {
        for y in 0..self.len() {
            print!("{:06b}: ", y);
            for x in 0..self[y].len() {
                print!("{} ", self[y][x]);
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
                print!("{} ", self[y][x]);
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