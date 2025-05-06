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
use proconio::marker::{Chars, Usize1};
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    // 2024-04-09 21:45-
    // 2024-04-10 12:40-12:45 (5min)
    // 2024-04-10 19:28-20:14 (46min)
    // 2024-04-13 14:22-16:06 (1h44min)
    // Total: 155min
    input! {
        n: usize,
        q: usize,
        c: [Usize1; n],
    }
    let mut lr = vec![];
    for i in 0..q {
        input!{
            li: Usize1,
            ri: Usize1,
        }
        lr.push((li, ri));
    }

    // セグメント木だと、TLEする。
    // [Li,Ri]のクエリに対して、(Ri-Li) * log(Ri-Li) * log(N)かかってしまう。
    // そこで、Mo's Algorithmを使って高速化する。

    mo_algorithm(n, c, q, lr);
}

/// Mo's Algorithm
/// n := 数列の要素数
/// a := 数列
/// q := クエリの個数
/// lr[i] := i番目のクエリ (左端, 右端)
fn mo_algorithm(n: usize, a: Vec<usize>, q: usize, lr: Vec<(usize, usize)>) {
    // Mo's algorithm
    // ◆機能：長さNの配列に対して，Q個の区間クエリ[L,R]を合計O(Q √N)で計算可能。
    // ◆条件1：全クエリを先読みできる(オフラインクエリ)こと。
    // ◆条件2：クエリiからクエリjへの移動時のクエリ値更新の計算量は、マンハッタン距離 O(|Li - Lj| + |Ri - Rj|) で決まること
    // ◆処理内容：
    // (1) 各クエリ(Li,Ri) をL,Rの2次元空間上にマッピング。
    // (2) L軸(横方向)の定義域[0,...,n-1]を√q個の区間に分割。
    // (3) LR平面の原点(([L,R]=[0,0])から出発し、1マスずつ移動。
    // (4) 左の区間から順に各クエリを処理。
    // (5) 各区間のクエリは下から上へ移動しながら処理。
    // ◆計算量：O(N√Q)
    // L軸(横方向)の移動量総和はO(N√Q)
    //      L(横軸)の移動は、1回の移動で最大 N / √Q (1区間の横幅)で、それが全クエリでQ回あるから合計O(N√Q)となる。
    //   ∵(横方向の移動量) = (区間を跨がない移動の総和 + 区間を跨ぐ移動の総和) 
    //                    = (区間幅 * クエリ数 + Lの定義域幅) 
    //                    = (N / √Q) * Q + N
    //                    = O(N√Q)
    // R軸(縦方向)の移動量総和もO(N√Q)
    //      R(縦軸)の移動は、1区間で最大 N で済み、√Q 個の区間があるから、全クエリでO(N√Q)となる。
    //   ∵(縦方向の移動量) = (1区間内での最大移動量 * 区間の個数) 
    //                    = N * √Q 
    //                    = O(N√Q)
    // なので、L,R (横、縦)の移動を合わせて、
    // Mo's Algorithm の全体の計算量は O(N√Q) となる。
    // 参考(gifで分かりやすい): https://kanpurin.hatenablog.com/entry/2021/04/09/001439
    // ◆個別の問題回答時の注意
    // LR平面上を1マス移動したときの、クエリ値の更新方法は、問題により異なるので、
    // mo_add() と、 mo_delete() の中身をカスタマイズすること。
    // また、 ans と counter の初期値にも、注意を払うこと。
    // 本実装では、ABC174-F 用にカスタマイズされている。 (クエリiが[Li,Ri]内の、ユニークな値の種類数)
    // その他のカスタマイズ例は、ABC242-G (クエリiが[Li,Ri]内の、同一値のペアの総数)を参考。

    // ◆実装コード：
    // √q個(仮)の区間に分割
    let root_q = (q as f64).sqrt().ceil() as usize;
    // 区間幅
    let width = max(1, n / root_q);

    // num_section := 区間の個数 (root_q とやりたいところだが、バグ予防のために区間幅から計算)
    let num_section = n / width + 1;
    
    // sections[i][j] := 左からi番目の区間の、下からj番目のクエリ (右端, 左端, クエリ番号))
    let mut sections = vec![vec![]; num_section];
    for i in 0..q {
        let index = lr[i].0 / width;
        sections[index].push((lr[i].1, lr[i].0, i));
    }
    // r(右端)についてソート
    for i in 0..num_section {
        sections[i].sort();
    }

    // [l_pre, r_pre] := 現在注目している区間
    let mut l_pre = 0;
    let mut r_pre = 0;

    // counter[x] := 要素xの個数
    let mut counter = vec![0; n];

    // anss[i] := i番目のクエリの答え
    let mut anss = vec![0; q];
    // ans := クエリが[l,r]の時の答え
    let mut ans = 0;

    // クエリが[l,r] = [0,0] の状態で、 counter と ans を初期化
    mo_add(0, &a, &mut counter, &mut ans);

    // クエリ範囲が1項分、拡大した時の更新 (counter と ans が更新される)
    fn mo_add(added_index: usize, a: &Vec<usize>, counter: &mut Vec<usize>, ans: &mut usize) {
        // クエリ範囲拡大
        let key = a[added_index];
        counter[key] += 1;

        // counter[key] の増減と、 ans の増減の関係は、問題によりけり。 
        // ABC174-F では、counterが増加時に1になったら、 ユニークな値の種類数 ans は +1 増える。
        // ABC242-G では、counterが増加時に偶数個になったら、 ペアの総数ans は +1 増える。
        if counter[key] == 1 {
            *ans += 1;
        }
    };

    // クエリ範囲が1項分、縮小した時の更新 (counter と ans が更新される)
    fn mo_delete(deleted_index: usize, a: &Vec<usize>, counter: &mut Vec<usize>, ans: &mut usize) {
        // クエリ範囲縮小
        let key = a[deleted_index];
        counter[key] -= 1;

        // counter[key] の増減と、 ans の増減の関係は、問題によりけり。 
        // ABC174-F では、counterが減少時に0になったら、 ユニークな値の種類数 ans は -1 減る。
        // ABC242-G では、counterが減少時に奇数個になったら、 ans は -1 減る。
        if counter[key] == 0 {
            *ans -= 1;
        }
    };

    // 左のセクションから順にクエリを実行するイメージ
    for i in 0..num_section {
        // 各セクションについて、前回のクエリから、その次のクエリまで、順にシミュレーション
        for &(ri, li, qi) in sections[i].iter() {
            // LR平面上を、r_pre -> ri, l_pre -> li と、1マスずつ移動しながら，クエリ値を更新
            
            // 下から上に登るとき (上から下に降りるor停滞する場合は、 r_pre >= ri となるため、このループ処理が動作しないので、条件分岐不要)
            for r0 in r_pre+1..ri+1 {
                // クエリ範囲拡大
                mo_add(r0, &a, &mut counter, &mut ans);
            }
            // 上から下に降りるとき (下から上に登るor停滞する場合は、 r_pre <= ri  は,r_pre > riのときしか動作しないのでif文の条件分岐不要)
            for r0 in (ri+1..r_pre+1).rev() {
                // クエリ範囲縮小
                mo_delete(r0, &a, &mut counter, &mut ans);
            }

            // 左から右に移動する時 (右から左に移動or停滞する場合、li<=l_preとなり、このループ処理はスキップされるので、条件分岐不要)
            for l0 in (l_pre..li) {
                // クエリ範囲縮小
                mo_delete(l0, &a, &mut counter, &mut ans);
            }

            // 右から左に移動する時 (左から右に移動or停滞する場合、l_pre<=liとなり、このループ処理はスキップされるので、条件分岐不要)
            for l0 in (li..l_pre).rev() {
                // クエリ範囲拡大
                mo_add(l0, &a, &mut counter, &mut ans);
            }
            // println!("(li, ri, ans) = {:?}", (li, ri, ans));
            anss[qi] = ans;
            l_pre = li;
            r_pre = ri;
        }
    }

    for i in 0..q {
        println!("{}", anss[i]);
    }
}