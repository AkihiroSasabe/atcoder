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
    // 2024-08-15 11:29-11:50 (31min)
    // 2024-08-15 16:28-18:05 (1h37min)
    // total: 2h 8min

    // 【エラトステネスの篩】
    // 55_555個以下の素数からなる長さNの数列を選択。任意の5つの項の和が、合成数(素数じゃない)になるような長さNの数列を答える問題。
    // 5個の和が素数じゃない => 5で割った余りが全て同じであれば、確実に5の倍数になる。
    // そこに気付けなかった。

    input! {
        n: usize
    }
    // 5個の素数を選ぶと、確実に奇数になってしまう。
    // 2 

    // 555_523 未満になれば良い。
    let prime_list = sieve_of_eratosthenes(55_555); // 最大 55_547
    let prime_list2 = sieve_of_eratosthenes(55_555 * 10); // 最大 555_523
    let mut set = BTreeSet::new(); // 素数の集合 (後で合成数か判定するために使う)
    for i in 0..prime_list2.len() {
        set.insert(prime_list2[i]);
    }

    let mut ans = vec![];
    for p in prime_list {
        if p % 5 == 1 {
            ans.push(p);
        }
        if ans.len() == n {break}
    }

    // println!("prime_list = {:?}", prime_list);
    // println!("prime_list.len() = {:?}", prime_list.len()); // 5637 個
    // println!("prime_list2 = {:?}", prime_list2);
    // println!("prime_list2.len() = {:?}", prime_list2.len()); // 45740 個


    // // combination による、全探索 (余裕でTLE)
    // for comb in prime_list.iter().combinations(n) {
    //     if judge(&comb, n, &set) {
    //         return
    //     }
    // }

    // dpかも?
    // 4個
    // dp[x][y] := x番目までみたとき、0<=y<=5 個足したときの和

    // math.comb(5637, 5)
    // 47346544408524417

    // 5637個の素数の中から、 N <= 55 個を選ぶ組み合わせは、  
    // 5637_C_55 = 12279509736240630295872230609749835176075446268539023044427833079338455863836350516172941356136528886172125091535625620544893414278400
    // 通りあるが、絶対無理...

    // 55_C_5 = 3_478_761 = 3.4 * 10^6 // 全部チェックすることはできる。
    // math.comb(55, 5)
    // 3478761
    
    // 途中の計算を省くこともでき...る?

    // // dp[num] := {ここまでで存在する num個の要素の和 を全列挙したもの}
    // let mut dp = vec![BTreeSet::new(); 6];
    // dp[0].insert(0);
    // let mut ans = vec![];

    // // for &p in prime_list.iter().rev() {
    // for p in prime_list {
    //     // p は追加可能か?
    //     let mut is_p_ok = true;
    //     for &sum in dp[4].iter() {
    //         if set.contains(&(sum + p)) {
    //             // 5つの和が合成数じゃない
    //             is_p_ok = false;
    //             break
    //         }
    //     }

    //     if !is_p_ok {continue}

    //     // p は追加可能
    //     ans.push(p);
    //     if ans.len() == n {break}
    //     let pre_dp = dp.clone();
    //     for num in 0..5 {
    //         for &v in pre_dp[num].iter() {
    //             dp[num+1].insert(v + p);
    //         }
    //     }
    // }
    // println!("dp = {:?}", dp);
    // println!("dp[5] = {:?}", dp[5]);
    for a in ans {
        print!("{} ", a);
    }
}

fn judge(p_list: &Vec<&usize>, n: usize, set: &BTreeSet<usize>) -> bool {
    let mut dp = vec![BTreeSet::new(); 6];
    dp[0].insert(0);

    let mut is_p_ok = true;
    for &p in p_list {
        for &sum in dp[4].iter() {
            if set.contains(&(sum + p)) {
                // 5つの和が合成数じゃない
                is_p_ok = false;
                break
            }
        }

        if !is_p_ok {return false}
        let pre_dp = dp.clone();
        for num in 0..5 {
            for &v in pre_dp[num].iter() {
                dp[num+1].insert(v + p);
            }
        }
    }

    for a in p_list.iter() {
        print!("{} ", a);
    }

    return true
}

// エラトステネスの篩(ふるい)
// n以下の素数を全て列挙する為のアルゴリズムO(n*log(log(n)))
fn sieve_of_eratosthenes(n: usize) -> Vec<usize> {
    // prime_judge[i] := iが素数なら1, そうでなければ0
    let mut prime_judge = vec![1; n + 1];
    prime_judge[0] = 0;
    prime_judge[1] = 0;

    // prime_list := n以下の素数を格納したリスト
    let mut prime_list = vec![];
    for i in 2..(n+1) {
        if prime_judge[i] == 0 {continue}
        for j in 2..((n/i)+1) {
            prime_judge[j * i] = 0;
        }
        prime_list.push(i);
    }
    return prime_list;
}