#![allow(dead_code, unused_imports)]
use proconio::{input, marker::{Usize1, Isize1, Chars}};
use itertools::Itertools;
use std::cmp::{max, min, Ordering, Reverse};
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::collections::{VecDeque, BinaryHeap, HashMap, BTreeMap, HashSet, BTreeSet};
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
use rand::Rng;
use num::{BigUint, ToPrimitive, Integer};
use num_bigint::ToBigUint;
fn main() {
    // 2025-06-07 21:41-22:33 (52min)
    input! {
        t: usize,
    }
    let mut n = vec![];
    let mut s = vec![];
    let mut s2 = vec![];
    for i in 0..t {
        input!{
            ni: usize,
            si: Chars,
        }
        n.push(ni);
        s2.push(si.clone());

        let mut temp = vec![];
        for ssi in si {
            temp.push(ssi as usize - 'a' as usize);
        }
        // println!("temp = {:?}", temp);
        s.push(temp);
    }

    for q in 0..t {

        let mut seg = SegmentTree::new(s[q].len());
        for i in 0..s[q].len() {
            seg.update(i, s[q][i] as isize);
        }

        // // i 以降で一番でかいやつ
        // let mut cum = vec![0; s[q].len()];
        // cum[s[q].len()-1] = s[q][s[q].len()-1];
        // for i in (1..s[q].len()).rev() {
        //     cum[i-1] = max(cum[i], s[q][i-1]);
        // }

        let mut ll = 0;
        let mut rr = 0;
        let mut is_chage = false;
        for i in 0..s[q].len()-1 {
            if s[q][i] > s[q][i+1] {
                is_chage = true;
                ll = i;
                if seg.range_max_query(i+1, s[q].len()-1) as usize <= s[q][i] {
                    rr = s[q].len();
                    break;
                }

                // めぐる式二分探索
                // 関数じゃなくて、クロージャーを使うと、引数を少なく出来る。
                let judge = |mid: usize| -> bool {
                    let max_val = seg.range_max_query(i+1, mid);
                    let is_ok = (max_val as usize) > s[q][i];
                    // println!("(q = {q}, mid, is_ok) = {:?}", (mid, is_ok));
                    return is_ok
                };
                // fn judge(mid: usize) -> bool {
                //    return true
                // }

                let mut ng = i+1;
                let mut ok = s[q].len()-1;
                if judge(ng) {
                    ok = ng;
                }
                while (ng as i128 - ok as i128).abs() > 1 {
                    let mid = (ng + ok) / 2;
                    let is_ok = judge(mid);
                    if is_ok {
                        ok = mid;
                    }
                    else {
                        ng = mid;
                    }
                }
                rr = ok;
                // println!("{}", ok);
                break;
            }
        }
        // println!("(q, ll, rr) = {:?}", (q, ll, rr));
        if is_chage {
            // ll を排除して、 rrの前に に挿入
            let ch = s2[q][ll].clone();
            s2[q].insert(rr, ch);
            s2[q].remove(ll);
        }
        for i in 0..s2[q].len() {
            print!("{}", s2[q][i]);
        }
        println!("");
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