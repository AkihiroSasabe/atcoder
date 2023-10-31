#![allow(dead_code, unused_imports)]
use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::{VecDeque, BTreeSet};
use std::collections::BinaryHeap;
use std::collections::{HashMap, BTreeMap};
use std::collections::HashSet;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    // 2023-10-29 19:56-20:20 降参
    // 2023-10-29 20:20-22:00 解説見ながら実装
    // 2023-10-30 20:30-20:55 デバッグ
    input! {
        n: usize,
        a: [usize; n]
    }
    let modulus = 998_244_353;

    // 順位変換で圧縮
    let (ranked_array, sorted_array, original_to_rank) = rank_array(&a);

    // Aを昇順に並べたときの、記録したA[i]の個数の累積和
    let mut bit = BinaryIndexedTree::new(n);

    // Aを昇順に並べたときの、記録したA[i]の累積和
    let mut bit_sum = BinaryIndexedTree::new(n);

    // 期待値の分子
    let mut numerator = 0;
    for i in 0..n {
        // 期待値の分母 (全事象の総数)
        let mut denominator = (i+1) * (i+1);
        denominator %= modulus;

        // 入力例1: 
        // 5 7 5 のとき、各事象が起きる確率
        // (5,5) -> 2/3 * 2/3 = 4/9
        // (5,7) -> 2/3 * 1/3 = 2/9
        // (7,5) -> 2/3 * 1/3 = 2/9
        // (7,7) -> 1/3 * 1/3 = 1/9
        // 対戦表 (表の数値は確率の分子)
        //   | 5,7
        // ----------
        // 5 | 4,2
        // 7 | 2,1

        // 入力例: 
        // 5 7 6 のとき、各事象が起きる確率
        // (5,5) -> 1/3 * 1/3 = 1/9
        // (5,6) -> 1/3 * 1/3 = 1/9
        // (5,7) -> 1/3 * 1/3 = 1/9
        // (6,5) -> 1/3 * 1/3 = 1/9
        // (6,6) -> 1/3 * 1/3 = 1/9
        // (6,7) -> 1/3 * 1/3 = 1/9
        // (7,5) -> 1/3 * 1/3 = 1/9
        // (7,6) -> 1/3 * 1/3 = 1/9
        // (7,7) -> 1/3 * 1/3 = 1/9
        // 対戦表 (表の数値は確率の分子)
        //   | 5,6,7
        // ----------
        // 5 | 1,1,1
        // 6 | 1,1,1
        // 7 | 1,1,1

        if bit.get_element(ranked_array[i]) == 0 {
            // その値を記録するのが、1回目のとき
            bit.add(ranked_array[i], 1);
        }
        else {
            // その値を記録するのが、2回目以降のとき
            bit.add(ranked_array[i], 2);
        }

        // a[i]未満の個数
        let mut num_less_ai = 0;
        if ranked_array[i] != 0 {
            num_less_ai = bit.sum(ranked_array[i]-1);
        }

        // a[i]の個数
        let num_ai = bit.sum(ranked_array[i]) - num_less_ai;
        let mut num_larger_ai = bit.sum(n-1) - bit.sum(ranked_array[i]);

        // aiより小さい奴
        numerator += 2 * num_less_ai * sorted_array[ranked_array[i]];
        numerator %= modulus;

        // 記録の中で、aiと同じ値の和
        numerator += num_ai * sorted_array[ranked_array[i]];
        numerator %= modulus;

        // 記録の中で、aiより大きい数の和
        let sum_over_ai = bit_sum.sum_range(ranked_array[i]+1, n-1);
        numerator += 2 * sum_over_ai;
        numerator %= modulus;

        // println!("num_less_ai = {num_less_ai}, num_ai = {num_ai}, num_larger_ai = {num_larger_ai}");
        // println!("numerator = {:?}, denominator = {:?}", numerator, denominator);

        let ans = mod_dev(numerator, denominator, modulus);
        println!("{}", ans);

        bit_sum.add(ranked_array[i], sorted_array[ranked_array[i]]);
    }

}


fn rank_array<T: Ord + std::hash::Hash + Clone + Copy>(array: &Vec<T>) -> (Vec<usize>, Vec<T>, HashMap<T, usize>) {
    // 配列を順位変換する関数 O(NlogN)
    // 要素の値を圧縮することを、目的として使うことを想定している。
    // Input: 
    //     array: 配列
    // Output: 
    //    ranked_array:     順位変換済み配列
    //    sorted_array:     ソート済みの配列(順位から元の値をマップさせる)
    //    original_to_rank: 元の値から順位を対応させるマップ
    // Example:
    // let array = vec![333, 111, 444, 111, 555, 999];
    // let (ranked_array, _, _) = rank_array(&array);
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
    // index番目の値を取得する (sumは累積和)
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



// mod p を法とした時の割り算 a / b の値
fn mod_dev(a: usize, b: usize, modulo: usize) -> usize {
    return a * mod_inverse(b, modulo) % modulo
}

// mod p を法とした時の逆数(逆元という) 1 / b の値
fn mod_inverse(a: usize, modulo: usize) -> usize {
    // フェルマーの小定理
    //     a^(p-1) = 1     (mod p)
    // <=> a * a^(p-2) = 1 (mod p)
    // <=> 1 / a = a^(p-2) (mod p)
    // (ただし、法pは素数)

    return mod_pow(a, modulo - 2, modulo)
}

// mod p を法とした時の累乗
// base^(x) % mod を繰り返し二乗法により、O(log2(x))の計算量で求める　(O(x)だとTLE)
// No.69参照
fn mod_pow(mut base: usize, mut exponent: usize, modulo: usize) -> usize {
    // 例: 3^4= (3^2)^2 = 9^2 = 81^1
    // 初期
    // 3^4
    // remainder=1
    // base=3
    // exp=4

    // i=0:
    // remainder = 1
    // base = 3 * 3 = 9
    // exp = 4 / 2 = 2

    // i=1:
    // remainder = 1
    // base = 9 * 9 = 81
    // exp = 2 / 2 = 1

    // i=2:
    // remainder = remainder * base = 81
    // base = 81 * 81
    // exp = 1 / 2 = 0

    let mut remainder = 1;
    while exponent != 0 {
        if exponent % 2 == 1 {
            remainder = (remainder * base) % modulo;
        }
        base = (base * base) % modulo;
        exponent /= 2;
    }
    return remainder
}


