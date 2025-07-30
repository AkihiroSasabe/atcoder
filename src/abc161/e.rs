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
    // 2025-07-26 14:24-16:49 (2h25min)
    input! {
        n: usize,
        k: usize, // work
        c: usize, // yasumu
        s: Chars,
    }

    // dp[i][0] := i 日目を休んだとき、0~i-1日目までで最大何日働けるか?
    // dp[i][1] := i 日目を働いたとき、0~i日目までで最大何日働けるか?
    let mut dp = vec![vec![0; 2]; n + c + 1];

    // dp_rev[i][0] := i 日目を休んだとき、i+1~n-1日目までで最大何日働けるか?
    // dp_rev[i][1] := i 日目を働いたとき、i+1~n-1日目までで最大何日働けるか?
    let mut dp_rev = vec![vec![0; 2]; n + c + 1];

    for i in 0..n {
        if s[i] == 'o' {
            dp[i][1] = 1;
            dp_rev[i][1] = 1;
        }
    }
    for i in 1..n {
        // 休む時
        dp[i][0] = max(dp[i][0], dp[i-1][0]);
        dp[i][0] = max(dp[i-1][0], dp[i-1][1]);

        // 働くとき
        if s[i] == 'x' {continue}
        if i >= c + 1 {
            dp[i][1] = max(dp[i][1], dp[i-c-1][1] + 1);
        }
        if i >= c {
            dp[i][1] = max(dp[i][1], dp[i-c][0] + 1);
        }
    }
    for i in (0..n-1).rev() {
        dp_rev[i][0] = max(dp_rev[i][0], dp_rev[i+1][0]);
        dp_rev[i][0] = max(dp_rev[i][0], dp_rev[i+1][1]);

        if s[i] == 'x' {continue}
        dp_rev[i][1] = max(dp_rev[i][1], dp_rev[i+c+1][1] + 1);
        dp_rev[i][1] = max(dp_rev[i][1], dp_rev[i+c][0] + 1);
    }

    // for i in 0..n {
    //     println!("dp[{i}] = {:?}", dp[i]);
    // }
    // for i in 0..n {
    //     println!("dp_rev[{i}] = {:?}", dp_rev[i]);
    // }
    // println!("dp = {:?}", dp);
    // println!("dp_rev = {:?}", dp_rev);

    let mut seg0 = SegmentTree::new(n);
    let mut seg1 = SegmentTree::new(n);

    for i in 0..n {
        // i日目に休み、i+c+1日目に働く。
        let mut val0 = 0;
        val0 += dp[i][0];
        if i + c < n {
            val0 += max(dp_rev[i+c][0], dp_rev[i+c][1]);
        }
        seg0.update(i, val0 as isize);

        // i日目に働き、i+c+1日目に休む。
        let mut val1 = 0;
        val1 += max(dp[i][0], dp[i][1]);
        if i + c + 1 < n {
            val1 += dp_rev[i+c+1][0];
        }
        seg1.update(i, val1 as isize);
    }

    // seg0.print_tree();
    // seg1.print_tree();

    for i in 0..n {
        // println!("i = {:?} ---- ---- ----", i);
        if s[i] == 'x' {continue}

        let mut val = 0;

        let left0 = (i+1).saturating_sub(c);
        let right0 = i;
        // println!("(left0, right0) = {:?}", (left0, right0));
        let cand0: isize = seg0.range_max_query(left0, right0);
        val = cand0;
        // println!("cand0 = {:?}", cand0);

        if i != 0 {
            let left1 = i.saturating_sub(c+1);
            let right1 = i-1;
            // println!("(left1, right1) = {:?}", (left1, right1));
            let cand1 = seg1.range_max_query(left1, right1);
            // println!("cand1 = {:?}", cand1);
            val = max(val, cand1);
        }
        if i == 0 {
            val = max(val, dp_rev[0][0]);
        }
        if val < k as isize {
            println!("{}", i + 1);
        }
    }
}


// セグメント木
// Derive注釈は、自作の構造体に有用な振る舞いを追加する。(Debugはprintの為、Cloneはベクトルの要素として使う為に追加した)
// 参考: https://doc.rust-jp.rs/book-ja/ch05-02-example-structs.html?highlight=derive#%E3%83%88%E3%83%AC%E3%82%A4%E3%83%88%E3%81%AE%E5%B0%8E%E5%87%BA%E3%81%A7%E6%9C%89%E7%94%A8%E3%81%AA%E6%A9%9F%E8%83%BD%E3%82%92%E8%BF%BD%E5%8A%A0%E3%81%99%E3%82%8B
// 本問は、要素を再代入更新し、区間の最大値を取得するタイプのセグメント木。
#[derive(Debug, Clone)]
struct SegmentTree {
    // 探索対象の配列の大きさ
    list_size: usize,
    // セグメント木の頂点の総数
    tree_size: usize,
    // セグメント木の葉の総数
    leaf_size: usize,
    // セグメント木
    tree: Vec<isize>,
}

// セグメント木実装で参考にしたのは、蟻本とE8氏の解答と下記のブログ
// https://easthop.hatenablog.com/entry/2020/12/15/211044
impl SegmentTree {
    fn new(list_size: usize) -> Self {
        // セグメント木の頂点の総数tree_sizeを求める。
        // まずはセグメント木の葉の数leaf_sizeを、
        // (leaf_size / 2 < list_size <= leaf_size)
        // を満たす2のべき乗数となるように計算
        let mut leaf_size = 1;
        while (leaf_size < list_size) {
            leaf_size *= 2;
        }

        // セグメント木の頂点数 = セグメント木の葉の数 * 2 - 1
        let tree_size = leaf_size * 2 - 1;

        // 1 << 60 = 1,152,921,504,606,846,976 = 1.152 * 10^18
        // let NEGATIVE_INF = (1 << 60) * (-1);
        let NEGATIVE_INF = std::isize::MIN;
        let tree = vec![NEGATIVE_INF; tree_size];
        return SegmentTree {list_size, tree_size, leaf_size, tree}
    }

    //      0
    //  1       2
    // 3, 4    5, 6
    // child1 = 2 * parent + 1
    // child2 = 2 * parent + 2

    // 探索対象の配列のk番目の要素を、値xに変更する
    fn update(&mut self, k: usize, x: isize) {
        // セグメント木におけるインデックスに変換
        let mut tree_index = k + self.tree_size / 2;
        self.tree[tree_index] = x;

        // 木を登りながら更新
        while tree_index > 0 {
            // 親の頂点
            tree_index = (tree_index - 1) / 2;
            let child_index_0 = tree_index * 2 + 1;
            let child_index_1 = tree_index * 2 + 2;
            self.tree[tree_index] = max(self.tree[child_index_0], self.tree[child_index_1]);
        }
    }

    // クラスの外からクエリを行うときのメソッド
    fn range_max_query(&self, q_l: usize, q_r: usize) -> isize {
        return self._range_max_query(q_l, q_r, 0, 0, self.leaf_size - 1);
    }

    // 閉区間[q_l, q_r]の最大値を求める。右端が開区間')'ではなく、閉区間']'にしているので注意
    fn _range_max_query(&self, q_l: usize, q_r: usize, v: usize, v_l: usize, v_r: usize) -> isize {
        // q_l:    探索区間の左端
        // q_r:    探索区間の右端(閉区間)
        // v:      現在の頂点のインデックス
        // v_l:    現在の頂点の守備範囲の左端
        // v_r:    現在の頂点の守備範囲の右端(閉区間)
        // 外からは、self._range_max_query(q_l, q_r, 0, 0, self.leaf_size - 1)として呼ぶ。特にv_rは、self.list_sizeではないので注意

        // (1)探索範囲が、その頂点が持つ守備範囲と、交差しない
        if v_r < q_l || q_r < v_l {
            let NEGATIVE_INF = (1 << 60) * (-1);
            return NEGATIVE_INF
        }
        // (2)探索範囲が、その頂点が持つ守備範囲を、完全に含む
        else if q_l <= v_l && v_r <= q_r {
            return self.tree[v]
        }
        // (3)探索範囲が、その頂点が持つ守備範囲と、部分一致
        else {
            // 2つの子頂点の内、大きい方を返す
            let child_0 = self._range_max_query(q_l, q_r, 2 * v + 1, v_l, (v_l + v_r) / 2);
            let child_1 = self._range_max_query(q_l, q_r, 2 * v + 2, (v_l + v_r) / 2 + 1, v_r);
            return max(child_0, child_1);
        }
    }

    // 配列を確認(デバッグ用)
    fn print_list(&self) {
        println!("Print Array: ");
        for i in 0..self.list_size {
            let tree_index = i + self.tree_size / 2;
            print!("{}, ", self.tree[tree_index]);
        }
        println!("");
    }

    // セグメント木を確認(デバッグ用)
    fn print_tree(&self) {
        let mut next_depth_index = 1;
        println!("Print Segment Tree: ");
        for i in 0..self.tree_size {
            if i == next_depth_index {
                println!("");
                next_depth_index = next_depth_index * 2 + 1;
            }
            print!("{} ", self.tree[i]);
            
        }
        println!("");
    }

}