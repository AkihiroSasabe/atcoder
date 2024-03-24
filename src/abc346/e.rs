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
    input! {
        h: usize,
        w: usize,
        m: usize,
    }
    let mut t = vec![];
    let mut a = vec![];
    let mut x = vec![];

    let mut hs = vec![BinaryHeap::new(); h];
    let mut ws = vec![BinaryHeap::new(); w];

    for i in 0..m {
        input! {
            ti: usize,
            aii: usize,
            xi: usize,
        }
        let ai = aii - 1;
        t.push(ti);
        a.push(ai);
        x.push(xi);
        if ti == 1 {
            hs[ai].push(i);
        }
        else if ti == 2 {
            ws[ai].push(i);
        }
    }

    let mut bit_yoko = BinaryIndexedTree::new(h);
    let mut bit_tate = BinaryIndexedTree::new(w);

    let mut btree = BTreeMap::new();
    for i in (0..m).rev() {
        let ai = a[i];
        let xi = x[i];
        let ti = t[i];
        if ti == 1 {
            // 横塗り
            let mut diff = w;
            if bit_yoko.sum_range(ai, ai) == 0 {
                // まだ塗られていない
                
                // 縦に塗られてる分だけ、差し引く
                diff -= bit_tate.sum(w-1);
                *btree.entry(xi).or_insert(0) += diff;
                bit_yoko.add(ai, 1);
            }
        }
        else if ti == 2 {
            // 縦塗り
            let mut diff = h;
            if bit_tate.sum_range(ai, ai) == 0 {
                // まだ塗られていない
                
                // 横に塗られてる分だけ、差し引く
                diff -= bit_yoko.sum(h-1);
                *btree.entry(xi).or_insert(0) += diff;
                bit_tate.add(ai, 1);
            }
        }
    }

    // 0の扱い
    for i in 0..h {
        // 横塗り
        let ai = i;
        let xi = 0;
        let ti = 1;
        if ti == 1 {
            // 横塗り
            let mut diff = w;
            if bit_yoko.sum_range(ai, ai) == 0 {
                // まだ塗られていない
                
                // 縦に塗られてる分だけ、差し引く
                diff -= bit_tate.sum(w-1);
                *btree.entry(xi).or_insert(0) += diff;
                bit_yoko.add(ai, 1);
            }
        }
        else if ti == 2 {
            // 縦塗り
            let mut diff = h;
            if bit_tate.sum_range(ai, ai) == 0 {
                // まだ塗られていない
                
                // 横に塗られてる分だけ、差し引く
                diff -= bit_yoko.sum(h-1);
                *btree.entry(xi).or_insert(0) += diff;
                bit_tate.add(ai, 1);
            }
        }
    }

    for i in 0..w {
        // 縦塗り
        let ai = i;
        let xi = 0;
        let ti = 2;
        if ti == 1 {
            // 横塗り
            let mut diff = w;
            if bit_yoko.sum_range(ai, ai) == 0 {
                // まだ塗られていない
                
                // 縦に塗られてる分だけ、差し引く
                diff -= bit_tate.sum(w-1);
                *btree.entry(xi).or_insert(0) += diff;
                bit_yoko.add(ai, 1);
            }
        }
        else if ti == 2 {
            // 縦塗り
            let mut diff = h;
            if bit_tate.sum_range(ai, ai) == 0 {
                // まだ塗られていない
                
                // 横に塗られてる分だけ、差し引く
                diff -= bit_yoko.sum(h-1);
                *btree.entry(xi).or_insert(0) += diff;
                bit_tate.add(ai, 1);
            }
        }
    }

    // println!("{}", btree.len());
    let mut kk = vec![];
    let mut vv = vec![];
    for (k, v) in btree {
        // println!("{} {}", k, v);
        if v != 0 {
            kk.push(k);
            vv.push(v);
        }
    }
    println!("{}", kk.len());
    for i in 0..kk.len() {
        println!("{} {}", kk[i], vv[i]);
    }
}


// BITは、要素加算、区間和を求めるセグメント木と同じ。
/// Binary Indexed Tree (BIT)
/// 参考: https://algo-logic.info/binary-indexed-tree/
/// (1)構築: O(N)
/// (2)加算: O(logN): 数列Anのi番目の項にxを足す (区間加算じゃないので注意)
/// (3)区間和: O(logN): 数列Anの先頭からi番目までの項の和を求める (閉区間だからiも含めるので注意)
/// セグメント木より機能が限定的だが、実装が簡単 & 定数倍で高速 & 省メモリ
#[derive(Debug, Clone, PartialEq, Eq)]
struct BinaryIndexedTree<T> {
    n: isize,       // 配列の要素数(数列の要素数+1)
    bit: Vec<T>    // データの格納先(1-indexed)。初期値は0
    // 0 始まり(0-indexed) ではなく 1 から番号を始めている(1-indexed)
    // また半開区間ではなく閉区間で考える。
    // これは後で計算をする際に楽になるため。
}

impl<T: Default + Copy + std::ops::AddAssign + std::ops::SubAssign + std::fmt::Debug + std::ops::Sub<Output = T>> BinaryIndexedTree<T> {
    fn new(n: usize) -> Self {
        BinaryIndexedTree {
            n: (n + 1) as isize,
            bit: vec![T::default(); n + 1] // 例えばTがusizeならdefault()は0を返す
        }
    }

    // add のインターフェースとしてindexは元の数列のindexを採用している(内部で+1している)
    fn add(&mut self, index: usize, x: T) {
        let mut i = (index + 1) as isize;
        // let mut i = index as isize; // こっちを採用すると、インターフェースも半開区間にできる
        while i < self.n {
            self.bit[i as usize] += x;
            // println!("i={}, i={:05b} -i={:05b}", i, i, -i);

            // i の最後の1のビット = i & -i (∵負の数は「ビット反転+1」で表現される)
            // 例: 6 & -6 = (00…0110)_2 & (11…1010)_2 = (00…0010)_2
            i += (i & - i); // iにi の最後の1のビットを足すと、親のインデックスに移れる

            // Rustでは、負の数は2の補数表現で保持される。
            // 補数の定義: N進法において自然数xを表現するのに必要な最小の桁数をnとしたとき
            // xのNの補数はN^n - x となる
            // 例： 5(10進数)=101(2進数)の2の補数は、2^3-5(10進法) = 1000 - 101 (2進法) = 011(2進法)となる
            // 参考1: http://www.cc.kyoto-su.ac.jp/~kbys/kiso/number/negative.html
            // 参考2: http://www.f.waseda.jp/takezawa/math/joho/hosuu.pdf
            // もっと端的に言うと、
            // 0の定義を、そのデータ型のビット数の限界に1桁左から1を追加したものとする
            // 例: 3bitだけ使える場合、下記のように考える
            // -3: 0101
            // -2: 0110
            // -1: 0111
            //  0: 1000 <- 3bitの0の定義
            //  1: 0001
            //  2: 0010
            //  3: 0011
            // また、isizeの場合、-1は
            // 1111111111
            // 1111111111
            // 1111111111
            // 1111111111
            // 1111111111
            // 1111111111
            // 1111 となる (1が64個)。
        }
    }

    // Tが非負整数型(usizeなど)のときに、除算を行う
    fn subtract(&mut self, index: usize, x: T) {
        let mut i = (index + 1) as isize;
        // let mut i = index as isize; // こっちを採用すると、インターフェースも半開区間にできる
        while i < self.n {
            self.bit[i as usize]  -= x;
            i += (i & - i); // iにi の最後の1のビットを足すと、親のインデックスに移れる
        }
    }

    // a_1 + a_2 + ... + a_i を計算する (sumのインターフェースは半開区間ではなく閉区間を採用。a[index]は足される)
    fn sum(&self, index: usize) -> T {
        let mut i = (index + 1) as isize;
        // let mut i = index as isize; // こっちを採用すると、インターフェースも開区間にできる
        let mut sum = T::default(); // 例えばTがusizeならdefault()は0を返す
        while i > 0 {
            // println!("i={}, sum={:?}", i, sum);
            sum += self.bit[i as usize];
            // i の最後の1のビット = i & -i (∵負の数は「ビット反転+1」で表現される)
            // 例: 6 & -6 = (00…0110)_2 & (11…1010)_2 = (00…0010)_2
            i -= (i & - i); // iにi の最後の1のビットを引くと、1個左上のノードのインデックスに移れる
            // println!("i={}, sum={:?}", i, sum);
            // println!("==== ==== ==== ====");
        }
        return sum
    }
    // 閉区間[left, right]を取得する
    fn sum_range(&self, left: usize, right: usize) -> T {
        let right_sum: T = self.sum(right);
        let left_sum: T = match left == 0 {
            true => Default::default(), // 0のこと
            false => self.sum(left-1)
        };
        let range_sum: T = right_sum - left_sum;
        return range_sum
    }
    // index番目の値を取得する (sum()は累積和を取得するメソッド)
    fn get_element(&self, index: usize) -> T {
        let element = match index == 0 {
            true => self.sum(index),
            false => self.sum(index) - self.sum(index - 1)
        };
        return element
    }
    fn print_all_cum(&self) {
        // デバッグ用に、各インデックスにおける、累積和を標準出力に print
        print!("bit = ");
        for i in 0..self.n-1 {
            let sum_i = self.sum(i as usize);
            print!("{:?} ", sum_i);
        }
        println!("");
    }
}

