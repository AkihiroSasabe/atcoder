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
    // 2024-01-27 21:16-21:52 (36min)
    input! {
        n: usize,
    }
    let mut a = vec![];
    let mut b = vec![];

    // ab[i] := [小さい番号の方の弦の頂点, 大きい番号の方の弦の頂点]
    let mut ab = vec![];
    // let mut aaa = vec![0; 2*n];
    // let mut bit = BinaryIndexedTree::new(2*n);

    // 本問は、要素を再代入更新し、区間の最大値を取得するタイプのセグメント木
    // seg := 配列の　i=v_min 番目の要素には、v_minを始点としたとき、最大の相方が格納されている。
    let mut seg = SegmentTree::new(2*n);
    for i in 0..n {
        input!{
            aii: usize,
            bii: usize,
        }
        let ai = aii - 1;
        let bi = bii - 1;
        a.push(ai);
        b.push(bi);

        let v_min = min(ai, bi);
        let v_max = max(ai, bi);

        ab.push([v_min, v_max]);

        // v_minを始点とする、相方
        seg.update(v_min, v_max as isize);

    }
    // abを、先頭頂点が、若い順にソートする
    ab.sort();
    let mut exist = false;
    for i in 0..n {
        let ai = ab[i][0];
        let bi = ab[i][1];
        let max_b = seg.range_max_query(ai, bi);
        // 交わってしまう <=> 区間[ai+1,bi-1]を始点とし、かつbi+1以上を終点とする頂点が存在する
        if max_b as usize > bi {
            exist = true;
        }
    }
    if exist {
        println!("Yes");
    }
    else {
        println!("No");

    }



}



// セグメント木
// Derive注釈は、自作の構造体に有用な振る舞いを追加する。(Debugはprintの為、Cloneはベクトルの要素として使う為に追加した)
// 参考: https://doc.rust-jp.rs/book-ja/ch05-02-example-structs.html?highlight=derive#%E3%83%88%E3%83%AC%E3%82%A4%E3%83%88%E3%81%AE%E5%B0%8E%E5%87%BA%E3%81%A7%E6%9C%89%E7%94%A8%E3%81%AA%E6%A9%9F%E8%83%BD%E3%82%92%E8%BF%BD%E5%8A%A0%E3%81%99%E3%82%8B
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


// /// Binary Indexed Tree (BIT)
// /// 参考: https://algo-logic.info/binary-indexed-tree/
// /// (1)構築: O(N)
// /// (2)加算: O(logN): 数列Anのi番目の項にxを足す (区間加算じゃないので注意)
// /// (3)区間和: O(logN): 数列Anの先頭からi番目までの項の和を求める (閉区間だからiも含めるので注意)
// /// セグメント木より機能が限定的だが、実装が簡単 & 定数倍で高速 & 省メモリ
// #[derive(Debug, Clone, PartialEq, Eq)]
// struct BinaryIndexedTree<T> {
//     n: isize,       // 配列の要素数(数列の要素数+1)
//     bit: Vec<T>    // データの格納先(1-indexed)。初期値は0
//     // 0 始まり(0-indexed) ではなく 1 から番号を始めている(1-indexed)
//     // また半開区間ではなく閉区間で考える。
//     // これは後で計算をする際に楽になるため。
// }

// impl<T: Default + Copy + std::ops::AddAssign + std::ops::SubAssign + std::fmt::Debug + std::ops::Sub<Output = T>> BinaryIndexedTree<T> {
//     fn new(n: usize) -> Self {
//         BinaryIndexedTree {
//             n: (n + 1) as isize,
//             bit: vec![T::default(); n + 1] // 例えばTがusizeならdefault()は0を返す
//         }
//     }

//     // add のインターフェースとしてindexは元の数列のindexを採用している(内部で+1している)
//     fn add(&mut self, index: usize, x: T) {
//         let mut i = (index + 1) as isize;
//         // let mut i = index as isize; // こっちを採用すると、インターフェースも半開区間にできる
//         while i < self.n {
//             self.bit[i as usize] += x;
//             // println!("i={}, i={:05b} -i={:05b}", i, i, -i);

//             // i の最後の1のビット = i & -i (∵負の数は「ビット反転+1」で表現される)
//             // 例: 6 & -6 = (00…0110)_2 & (11…1010)_2 = (00…0010)_2
//             i += (i & - i); // iにi の最後の1のビットを足すと、親のインデックスに移れる

//             // Rustでは、負の数は2の補数表現で保持される。
//             // 補数の定義: N進法において自然数xを表現するのに必要な最小の桁数をnとしたとき
//             // xのNの補数はN^n - x となる
//             // 例： 5(10進数)=101(2進数)の2の補数は、2^3-5(10進法) = 1000 - 101 (2進法) = 011(2進法)となる
//             // 参考1: http://www.cc.kyoto-su.ac.jp/~kbys/kiso/number/negative.html
//             // 参考2: http://www.f.waseda.jp/takezawa/math/joho/hosuu.pdf
//             // もっと端的に言うと、
//             // 0の定義を、そのデータ型のビット数の限界に1桁左から1を追加したものとする
//             // 例: 3bitだけ使える場合、下記のように考える
//             // -3: 0101
//             // -2: 0110
//             // -1: 0111
//             //  0: 1000 <- 3bitの0の定義
//             //  1: 0001
//             //  2: 0010
//             //  3: 0011
//             // また、isizeの場合、-1は
//             // 1111111111
//             // 1111111111
//             // 1111111111
//             // 1111111111
//             // 1111111111
//             // 1111111111
//             // 1111 となる (1が64個)。
//         }
//     }

//     // Tが非負整数型(usizeなど)のときに、除算を行う
//     fn subtract(&mut self, index: usize, x: T) {
//         let mut i = (index + 1) as isize;
//         // let mut i = index as isize; // こっちを採用すると、インターフェースも半開区間にできる
//         while i < self.n {
//             self.bit[i as usize]  -= x;
//             i += (i & - i); // iにi の最後の1のビットを足すと、親のインデックスに移れる
//         }
//     }

//     // a_1 + a_2 + ... + a_i を計算する (sumのインターフェースは半開区間ではなく閉区間を採用。a[index]は足される)
//     fn sum(&self, index: usize) -> T {
//         let mut i = (index + 1) as isize;
//         // let mut i = index as isize; // こっちを採用すると、インターフェースも開区間にできる
//         let mut sum = T::default(); // 例えばTがusizeならdefault()は0を返す
//         while i > 0 {
//             // println!("i={}, sum={:?}", i, sum);
//             sum += self.bit[i as usize];
//             // i の最後の1のビット = i & -i (∵負の数は「ビット反転+1」で表現される)
//             // 例: 6 & -6 = (00…0110)_2 & (11…1010)_2 = (00…0010)_2
//             i -= (i & - i); // iにi の最後の1のビットを引くと、1個左上のノードのインデックスに移れる
//             // println!("i={}, sum={:?}", i, sum);
//             // println!("==== ==== ==== ====");
//         }
//         return sum
//     }
//     // 閉区間[left, right]を取得する
//     fn sum_range(&self, left: usize, right: usize) -> T {
//         let right_sum: T = self.sum(right);
//         let left_sum: T = match left == 0 {
//             true => Default::default(), // 0のこと
//             false => self.sum(left-1)
//         };
//         let range_sum: T = right_sum - left_sum;
//         return range_sum
//     }
//     // index番目の値を取得する (sum()は累積和を取得するメソッド)
//     fn get_element(&self, index: usize) -> T {
//         let element = match index == 0 {
//             true => self.sum(index),
//             false => self.sum(index) - self.sum(index - 1)
//         };
//         return element
//     }
//     fn print_all_cum(&self) {
//         // デバッグ用に、各インデックスにおける、累積和を標準出力に print
//         print!("bit = ");
//         for i in 0..self.n-1 {
//             let sum_i = self.sum(i as usize);
//             print!("{:?} ", sum_i);
//         }
//         println!("");
//     }
// }

