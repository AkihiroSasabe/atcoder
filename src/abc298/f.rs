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
    // 2023-10-10 19:53-21:18 (25min)
    // 2023-10-10 21:18-22:12 (54min)
    // total: 79min = 1h19min
    input! {
        n: usize,
    }
    let mut r = vec![];
    let mut c = vec![];
    let mut p = vec![];
    for i in 0..n {
        input! {
            r_i: usize,
            c_i: usize,
            p_i: usize,
        }
        r.push(r_i);
        c.push(c_i);
        p.push(p_i);
    }
    
    // y行目に居る全x
    let mut h_ws = HashMap::new();
    // x列目に居る全y
    let mut w_hs = HashMap::new();

    // y行目の和
    let mut h_sum = BTreeMap::new();
    // x列目の和
    let mut w_sum = BTreeMap::new();
    // 点(y,x)における点数
    let mut hw_point = HashMap::new();

    for i in 0..n {
        h_ws.entry(r[i]).or_insert(vec![]).push(c[i]);
        w_hs.entry(c[i]).or_insert(vec![]).push(r[i]);

        // h_hash.entry(r[i]).or_insert(HashMap::new()).insert(c[i], p[i]);
        // w_hash.entry(c[i]).or_insert(HashMap::new()).insert(r[i], p[i]);
        *h_sum.entry(r[i]).or_insert(0) += p[i];
        *w_sum.entry(c[i]).or_insert(0) += p[i];
        hw_point.insert((r[i], c[i]), p[i]);
    }

    let mut y_asshuku = BTreeMap::new();
    let mut count = 0;
    for (&y, &y_sum) in h_sum.iter() {
        y_asshuku.insert(y, count);
        count += 1;
    }
    let mut x_asshuku = BTreeMap::new();
    let mut count = 0;
    for (&x, &x_sum) in w_sum.iter() {
        x_asshuku.insert(x, count);
        count += 1;
    }

    let mut hw_point_asshuku = HashMap::new();
    for ((h, w), point) in hw_point.iter() {
        hw_point_asshuku.insert((y_asshuku[&h], x_asshuku[&w]), point);
    }
    
    // セグメント木
    let mut w_seg = SegmentTree::new(x_asshuku.len());

    for (&x, &x_sum) in w_sum.iter() {
        w_seg.update(x_asshuku[&x], x_sum as isize);
    }

    let mut ans = 0;
    for (&y, &y_sum) in h_sum.iter() {
        // セグメント木の更新
        // y列目に居る全xを炙り出す
        for &x in h_ws[&y].iter() {
            let x_comp = x_asshuku[&x];
            // let aaa = hw_point[&(y, x)];
            w_seg.update(x_comp, w_sum[&x] as isize- hw_point[&(y, x)] as isize);
        }
        let max_w = w_seg.range_max_query(0, x_asshuku.len()-1);
        ans = max(ans, max_w as usize + y_sum);
        // 元に戻す
        for &x in h_ws[&y].iter() {
            let x_comp = x_asshuku[&x];
            w_seg.update(x_comp, w_sum[&x] as isize);
        }
    }
    println!("{}", ans);




}


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