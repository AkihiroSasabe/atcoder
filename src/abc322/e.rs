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
    // 2023-10-01 10:03-13:23 (3h20min = 200min)
    // 2023-10-01 19:10-19:50 (40min)
    // total: (240min = 4h)
    input! {
        n: usize,
        k: usize,
        p: usize,
    }
    let mut c = vec![];
    let mut a = vec![];
    for i in 0..n {
        input! {
            c_i: usize,
            a_i: [usize; k]
        }
        c.push(c_i);
        a.push(a_i);
    }

    // dp[i番目までの施策を実行可能][各パラメータのレベルの状態] = 最低コスト
    let base = p + 1;
    let init_cost: usize = 100_000_000_000_000; // C * N <= 10^9 * 100 = 10^11
    let mut dp = vec![vec![init_cost; base.pow(k as u32) as usize]; n];
    dp[0][0] = 0;
    let state_vector_0 = a[0].clone();
    let state_num_0 = get_state_num(&state_vector_0, base);

    // どのパラメータのレベルも上がらないのに、コストが掛かるケースに注意
    dp[0][state_num_0] = min(dp[0][state_num_0], dp[0][0] + c[0]);

    for action in 1..n {
        // println!("============== new action {action} ================");

        // 前の状態の引継ぎ
        dp[action] = dp[action-1].clone();
        for state_num in 0..(base.pow(k as u32) as usize) {
            // println!("----new state_num: {state_num} --------------------");
            // println!("action = {action}, state_num = {state_num}");
            let state_vector = get_state_vector(state_num, base, k);
            let mut next_state_vector = vec![];
            for param in 0..k {
                // 各パラメータは、5以上は全部同じだとみなして良い。なぜなら目標水準P<=5なので、5以上は全部合格だから。
                let level = min(p, state_vector[param] + a[action][param]);
                next_state_vector.push(level);
            }
            // println!("state_vector = {:?}", state_vector);
            // println!("a[{action}] = {:?}", a[action]);
            // println!("next_state_vector = {:?}", next_state_vector);
            // println!("dp[{state_num}] = {}", dp[action-1][state_num]);

            let next_state_num = get_state_num(&next_state_vector, base);
            dp[action][next_state_num] = min(dp[action][next_state_num], dp[action-1][state_num] + c[action]);


            // println!("dp[{next_state_num}] = {}", dp[action][next_state_num]);
            // if next_state_vector[2] == 4 && dp[next_state_num] != init_cost {
            //     println!("dp = {:?}", dp);
            //     return;}
            // if next_state_vector[2] == 5 && dp[next_state_num] != init_cost {return;}
            // if next_state_num == 209 && dp[next_state_num] != init_cost {return;}
            // if next_state_num == 215 && dp[next_state_num] != init_cost {return;}
            // if state_num > 6*6 {return;}
        }
    }

    let criteria_vector = vec![p; k];
    let criteria_num = get_state_num(&criteria_vector, base);
    // println!("criteria_vector = {:?}", criteria_vector);
    // println!("criteria_num = {criteria_num}");

    let ans = dp[n-1][criteria_num];
    if ans == init_cost {
        println!("-1");
    }
    else {
        println!("{}", ans);
    }

}

/// p進数の数字を、sizeビット分のベクトルに変換する
fn get_state_vector(mut state_num: usize, base: usize, size: usize) -> Vec<usize> {
    // state_num: 変換したい数値
    // base: p進数
    // size: ビット数
    
    let mut state = vec![];
    // while state_num != 0 {
    while state.len() < size {
        let coefficient = state_num % base; // 各ビットに格納される[0,p-1]の値
        state.push(coefficient);
        state_num /= base;
    }

    return state
}

/// 各ビットの値をベクトルに格納したものから、p進数の数字を取得する。
fn get_state_num(state_vector: &Vec<usize>, base: usize) -> usize {
    // state_num: 変換したいベクトル。各ビットには(0からp未満の値が格納されている)
    // base: p進数
    // state_num: 10進法で表された値

    let mut state_num = 0;
    let mut power = 1;

    for i in 0..state_vector.len() {
        state_num += state_vector[i] * power;
        power *= base;
    }
    return state_num
}