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
use std::collections::HashSet;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
    }
    let mut a = vec![];
    let mut b = vec![];
    let mut c = vec![];
    let mut d = vec![];
    for i in 0..n {
        input! {
            a_i: f64,
            b_i: f64,
        }
        a.push(a_i);
        b.push(b_i);
    }

    for i in 0..m {
        input! {
            c_i: f64,
            d_i: f64,
        }
        c.push(c_i);
        d.push(d_i);
    }

    // 方針
    //     「集合の中で大きい方から K 番目の値 x を求める」
    // <=> 「y + dy以上の値がk個未満、y - dy以上の値がk個以上 が dy -> 0 (~10^-9) で成り立つようなyを求める」
    // <=> 「あるyについて、y以上の要素の個数がk未満かk以上か?という判定問題」　+　「2分探索」で求める
    
    // 例 x=60%のとき、x以上の個数はk=4個
    // x+dx=61のとき、x+dx以上の個数はK個未満
    // x-dx=59のとき、x-dx以上の個数はK個以上
    // 順位    濃度(%)
    // 1       90
    // 2       80
    // 3       70 <-x+dx
    // 4 <-K   60 <-X
    // 5       50 <-x-dx
    // 6       40

    // ABC-294-F「MN通りある砂糖水のうち、濃度が concentration 以上の砂糖水は何本あるか?」
    // 2つの砂糖水を混ぜたときに、濃度が concentration 以上か判定する方法

    // (1)1つの砂糖水の濃度を、水の量を変えずに追加で砂糖を加えてconcentration以上にする方法
    // 砂糖水の内訳が(砂糖, 水) = (sugar, water) gram であるとき、
    // この砂糖水の水の量を変えずに、濃度 = concentration にしようとするとき、砂糖の量は sugar_new は下記のようになる。
    // concentration := sugar_new / (sugar_new +  water) より
    // sugar_new = (sugar_new + water) * concentration
    // sugar_new = water * concentration / (1 - concentration)

    // よって下記のことがいえる。
    // sugar_new <= sugar: 元の砂糖水の濃度がconcentration以上
    // また、濃度をconcentration以上にする為にはdiff gramの砂糖が必要と分かる。
    // diff = sugar_new - sugar

    // (2)2つの砂糖水を混ぜたときに、濃度がconcentration以上か判定する方法
    // ・片方の砂糖水の濃度をconcentrationにする為に、必要な追加の砂糖の量をdiff_1
    // ・もう片方の砂糖水の濃度をconcentrationにする為に、必要な追加の砂糖の量をdiff_2
    // としたとき、
    // diff1 + diff2 <= 0 
    // なら2つの砂糖水を混ぜたときに、濃度が concentration 以上になる

    // 青木君の各砂糖水のdiffを格納した列 diff_aoki = (d1, d2, ... dm) を用意
    // diff_aoki をソート
    // 高橋君の各砂糖水のdiffを格納した列 diff_takahashi = (d1, d2, ... dn) も用意
    // 高橋君の各砂糖水iについて、
    // diff_takahashi[i] + diff_aoki[j] <= 0 
    // になるような最小のj (=j_min) を2分探索によりO(logM)で求めて足し合わせ、それがK以上か判定する


    // takahashi: [sato: ai, mizu: bi] x n本
    // aoki     : [sato: ci, mizu: di] x m本
    let mut ng = 0.0; // nm種類の組み合わせの砂糖水のうち、k個以上の砂糖水が濃度ng以上になるような濃度。
    let mut ok = 1.0; // nm種類の組み合わせの砂糖水のうち、k個未満の砂糖水が濃度ok以上になるような濃度
    // 10^-9の精度で解く必要がある。 log10(2^30)=9.03...なので、30回2分探索すれば理論上は良いが、余裕をもって100回
    // O(100 * (N+M) logM)
    for _ in 0..100 {
        let concentration = (ng + ok) / 2.0;
        let sugar_over_water = concentration / (1.0 - concentration);

        // O(MlogM)
        // diff_aoki := 青木君の各砂糖水の濃度を concentration にする為に必要な砂糖の量
        let mut diff_aoki = vec![];
        for i in 0..m {
            diff_aoki.push(c[i] - d[i] * sugar_over_water);
        }
        diff_aoki.sort_by(|x1, x2| x1.partial_cmp(x2).unwrap());

        // O(NlogM)
        let mut num = 0;
        for i in 0..n {
            // diff_takahashi_i := 高橋君の砂糖水iの濃度を concentration にする為に必要な砂糖の量
            let diff_takahashi_i = a[i] - (b[i] * sugar_over_water);
            // O(logM)
            // 高橋君の砂糖水iと青木君の砂糖水を混ぜたときに、濃度が concentration 以上になる最小のindex
            // <=>青木君の砂糖水のdiffが -diff_takahashi_i 以上になる最小のindex
            let idx = diff_aoki.lower_bound_by(|&a| a.partial_cmp(&(-diff_takahashi_i)).unwrap());
            num += m - idx;
        }
        if num < k {
            ok = concentration;
        }
        else {
            ng = concentration;
        }
    }
    // y=xのときy以上の濃度はk個あるから、返すべき濃度はk個未満のokではなく、k個以上である一応ngを返すべき。
    println!("{}", ng * 100.0);
    // println!("{}", ok * 100.0);

}