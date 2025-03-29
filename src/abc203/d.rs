#![allow(dead_code, unused_imports)]
use proconio::{input, marker::{Usize1, Isize1}};
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
use rand::Rng;
use num::{BigUint, ToPrimitive};
use num_bigint::ToBigUint;
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [[usize; n]; n],
    }

    // solve_tle(n,k,a);
    solve(n,k,a);
}



fn solve(n: usize, k: usize, a: Vec<Vec<usize>>) {
    // 2025-03-27 20:34-21:00
    // 2025-03-28 12:47-

    // めぐる式二分探索
    // 関数じゃなくて、クロージャーを使うと、引数を少なく出来る。
    let judge = |mid: usize| -> bool {
        // 全区間の中央値で、mid以下が1区間でもあるかを判定する関数
        // println!("mid = {:?}", mid);

        let mut a2 = vec![vec![0; n]; n];
        for i in 0..n {
            for j in 0..n {
                if a[i][j] > mid {
                    a2[i][j] = 1;
                }
            }
        }
        // println!("a2 = {:?}", a2);

        // 大きい方から2番目のx
        // 1 2 3 5 5 5 x以下が、N-2個以上はある。

        let cum_2d = get_2d_cumulative_sum(&a2);
        // k^2 / 2 + 1 番目に高い
        // 自分以下が、k^2 - 個
        let max_num = k*k / 2 + 1;
        // println!("max_num = {:?}", max_num);
        for i in 0..n {
            if i+k-1 >= n {continue}
            for j in 0..n {
                if j+k-1 >= n {continue}
                let ys = i;
                let xs = j;
                let yt = i+k-1;
                let xt = j+k-1;
                let num = get_sum_from_2d_cum(&cum_2d, ys, xs, yt, xt);
                // mid よりでかいマスの個数が、 k^2 / 2 + 1 個未満なら、中央値はm以下となる。
                // println!("mid, ys, xs, yt, xt, num = {:?}", (mid,ys, xs, yt, xt, num));
                if num < max_num {return true}
            }
        }
        return false
    };


    // 全区間の中央値について、ok以下となる中央値が1個でも存在する。そんな最小のokを求める。
    // k^2 / 2 + 1 
    let mut ng = 0;
    let mut ok = 1_000_000_001; // 
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
    println!("{}", ok);
    
    // ２次元累積和




}



/// 2次元累積和 cum を計算する関数
/// cum[y][x] := 原点(0,0)から右下(y,x)までの画素値の累積和
fn get_2d_cumulative_sum<T>(img: &Vec<Vec<T>>) -> Vec<Vec<T>> 
    where T: std::ops::Add<Output = T> + std::ops::Sub<Output = T> + Copy + Default
{
    let h = img.len();
    let w = img[0].len();
    let zero: T = Default::default();

    // 累積和の初期化
    let mut cum: Vec<Vec<T>> = vec![vec![zero; w]; h];
    cum[0][0] = img[0][0];

    // 1行目(y=0)の計算
    for x in 1..w {
        cum[0][x] = img[0][x] + cum[0][x-1];
    }
    // 2行目以降(y>0)の計算
    for y in 1..h {
        // 1列目(x=0)の計算
        cum[y][0] = img[y][0] + cum[y-1][0];
        // 2列目以降(x>0)の計算
        for x in 1..w {
            cum[y][x] = img[y][x] + cum[y-1][x] + cum[y][x-1] - cum[y-1][x-1];
        }
    }

    return cum
}

/// 2次元累積和から、閉区間(ys, xs) ~ (yt, xt) で囲まれた矩形領域の和を求める関数
fn get_sum_from_2d_cum<T>(cum_2d: &Vec<Vec<T>>, ys: usize, xs: usize, yt: usize, xt: usize) -> T 
    where T: std::ops::Add<Output = T> + std::ops::Sub<Output = T> + Copy,
{
    if xs == 0 && ys == 0 {
        let sum = cum_2d[yt][xt];
        return sum
    }
    else if xs == 0 && ys != 0 {
        let sum = cum_2d[yt][xt] - cum_2d[ys-1][xt];
        return sum
    }
    else if xs != 0 && ys == 0 {
        let sum = cum_2d[yt][xt] - cum_2d[yt][xs-1];
        return sum
    }
    else {
        // xs != 0 && ys != 0
        let sum = cum_2d[yt][xt] + cum_2d[ys-1][xs-1] - cum_2d[yt][xs-1] - cum_2d[ys-1][xt];
        return sum
    }
}



fn solve_tle(n: usize, k: usize, a: Vec<Vec<usize>>) {
    let mut a_array = vec![];
    for i in 0..n {
        for j in 0..n {
            a_array.push(a[i][j]);
        }
    }
    // println!("a_array = {:?}", a_array);

    let (ranked_array, sorted_array, original_to_rank) = rank_array(&a_array);
    let mut a2 = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            a2[i][j] = ranked_array[i*n + j];
        }
    }
    // println!("a2 = {:?}", a2);

    // Output: 
    //    ranked_array:     順位変換済み配列
    //    sorted_array:     ソート済みの配列(順位から元の値をマップさせる)
    //    original_to_rank: 元の値から順位を対応させるマップ
    // Example:
    // let array = vec![333, 111, 444, 111, 555, 999];
    // let (ranked_array, _sorted_array, _original_to_rank) = rank_array(&array);
    // assert_eq!(ranked_array, vec![2, 0, 3, 0, 4, 5]);

    // 800^2
    // 640_000 = 6.4*10^5

    // 800^3
    // 512_000_000 = 5.1 * 10^8


    let mut lake = vec![];
    let mut bit: BinaryIndexedTree<usize> = BinaryIndexedTree::new(n*n);
    for i in 0..k {
        for j in 0..k {
            lake.push(a2[i][j]);
            bit.add(a2[i][j], 1);
        }
    }
    // bit.print_all_cum();
    lake.sort();
    // println!("lake = {:?}", lake);
    let max_rank = k*k/2 + 1;
    let min_rank = k*k + 1 - max_rank;
    // println!("min_rank = {:?}", min_rank);
    let ind = min_rank - 1;
    let mid = lake[ind];
    // println!("mid = {:?}", mid);
    let mut pre_ans = mid;
    let ans = sorted_array[pre_ans];
    // println!("ans = {:?}", ans);

    for yc in 0..n-k+1 {
        let aaa: Vec<usize> = if yc % 2 == 0 {
            (0..n-k+1).collect()
        }
        else {
            (0..n-k+1).rev().collect()
        };
        for (ind, &xc) in aaa.iter().enumerate() {
            if yc == 0 && xc == 0 {continue}
            if ind == 0 {
                // 上から下
                for diff in 0..k {
                    let x = xc + diff;
                    bit.add(a2[yc+k-1][x], 1);
                    bit.subtract(a2[yc-1][x], 1);
                }
            }
            else {
                // 左から右 or 右から左
                for diff in 0..k {
                    let y = yc + diff;
                    let x_head = if yc % 2 == 0 {xc + k -1} else {xc};
                    let x_tail = if yc % 2 == 0 {xc-1} else {xc + k};
                    bit.add(a2[y][x_head], 1);
                    bit.subtract(a2[y][x_tail], 1);
                }
            }

            // めぐる式二分探索
            // 関数じゃなくて、クロージャーを使うと、引数を少なく出来る。
            let judge = |mid: usize| -> bool {
                return min_rank <= bit.sum(mid)
            };
            // fn judge(mid: usize) -> bool {
            //    return true
            // }

            // min_rank
            let mut ng = 0;
            let mut ok = n*n-1;
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
            pre_ans = min(pre_ans, ok);
        }
    }

    // 75_851_851
    let ans = sorted_array[pre_ans];
    println!("{}", ans);




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




fn rank_array<T: Ord + std::hash::Hash + Clone + Copy>(array: &Vec<T>) -> (Vec<usize>, Vec<T>, HashMap<T, usize>) {
    // 配列を順位変換する関数 O(NlogN)
    // 要素の値を圧縮することを、目的として使うことを想定している。(座標圧縮)
    // Input: 
    //     array: 配列
    // Output: 
    //    ranked_array:     順位変換済み配列
    //    sorted_array:     ソート済みの配列(順位から元の値をマップさせる)
    //    original_to_rank: 元の値から順位を対応させるマップ
    // Example:
    // let array = vec![333, 111, 444, 111, 555, 999];
    // let (ranked_array, _sorted_array, _original_to_rank) = rank_array(&array);
    // assert_eq!(ranked_array, vec![2, 0, 3, 0, 4, 5]);

    // 配列のサイズ
    let n = array.len();

    // B木<数列中に登場する値, 頻度>
    let mut btree: BTreeMap<T, usize> = BTreeMap::new();
    for i in 0..n {
        *(btree.entry(array[i]).or_insert(0)) += 1;
    }

    // 昇順ソート済みの、順位変換済み配列
    let mut sorted_rank_array = vec![];
    let mut rank = 0;
    for (k, frequency) in btree {
        for j in 0..frequency {
            sorted_rank_array.push(rank);
        }
        rank += frequency; // sorted_rank_array = [0, 0, 2, 3, 4, 5], 
        // ここを1にすると、隙間なくなる。
        // rank += 1; //sorted_rank_array = [0, 0, 1, 2, 3, 4], 
    }
    // println!("sorted_rank_array = {:?}, ", sorted_rank_array);

    // 順位から元の値をマップさせる
    let mut sorted_array = (*array).clone();
    sorted_array.sort();

    // 元の値から順位を対応させるマップ
    let mut original_to_rank: HashMap<T, usize> = HashMap::new();
    for i in 0..n {
        original_to_rank.insert(sorted_array[i], sorted_rank_array[i]);
    }

    // 元の順序の、順位変換済み配列
    let mut ranked_array: Vec<usize> = vec![];
    for i in 0..n {
        ranked_array.push(*(original_to_rank.get(&array[i]).unwrap()));
    }

    return (ranked_array, sorted_array, original_to_rank)
}
