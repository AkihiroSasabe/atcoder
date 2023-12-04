use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::{VecDeque, HashSet};
use std::collections::BinaryHeap;
use std::collections::{HashMap, BTreeMap};
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
fn main() {
    // 2023-12-03 11:27-12:00 (32min)
    // 2023-12-03 19:44-20:00 (26min) 解説を見る。
    // 2023-12-04 13:35-13:42 (7min)
    // total 65min
    input! {
        n: usize,
        c: [usize; n],
        x: [usize; n],
    }
    
    // 重要な性質: バブルソートの交換回数 = 転倒数

    // 純粋な転倒数を求める
    let mut inv_num = get_inv_num(&x);

    // 同じ色毎に数列をまとめる。
    let mut hash = HashMap::new();
    for i in 0..n {
        hash.entry(c[i]).or_insert(vec![]).push(x[i]);
    }

    for (_, color_array) in hash {
        // 同じ色同士で発生している転倒数の寄与を求める
        let same_color_inv_num = get_inv_num(&color_array);
        // 同じ色同士の交換コストは0なので、転倒数の寄与から消していく
        inv_num -= same_color_inv_num;
    }
    println!("{}", inv_num);

    // 転倒数とバブルソートについてのメモ
    // ■バブルソートのアルゴリズム概要:
    // https://algo-method.com/tasks/439/editorial
    // 1. 内側のループ：
    //     i=0,1,⋯,N−2 について
    //     もし A[i]>A[i+1] ならば、A[i] と A[i+1] を交換
    // 2. 外側のループ：
    //     内側のループを繰り返す。内側のループで要素同士の交換が1度も発生しなかったら終了

    // ■バブルソートの計算量はO(N^2): 
    // https://judge.u-aizu.ac.jp/onlinejudge/commentary.jsp?id=ALDS1_2_A&pattern=post&type=general&filter=Algorithm
    // 外側のループを一周こなす毎に、最小値から位置が確定していく。
    // なので最大でも外側のループは、N周こなせばソートが完了していく。
    // 内側のループは回す毎に1個はソート位置が確定していくので、ループの回数も1個ずつ減って、N-1回, N-2回, ..., 1回で済む。よって計算量はO(N(N-1)/2)
    
    // ■典型89
    // 転倒数 := Ai > Aj かつi < jを満たす(i,j)の組の個数。つまり、「左にいるのに大きいやんけ」となる組の個数 (列の乱れの具合)
    // 性質「転倒数 = バブルソートのスワップの回数」
    // 証明は89を俺のコードに書いたので見てくれ。

    // 転倒数の最大値は、N-1 + N-2 + ... + 1 = ((N-1) + 1) * (N-1) / 2 = N(N-1)/2

    // バブルソートの例
    // --------初期状態--------
    // 5,3,1,0 ：転倒数3+2+1=6
    // --------ループ1--------
    // 3,5,1,0 i=0,1をswap: 転倒数5
    // 3,1,5,0 i=1,2をswap: 転倒数4
    // 3,1,0,5 i=2,3をswap: 転倒数3 [1回目のループ終了後、1番大きい5が、後ろから1番目に来ている]
    // --------ループ2--------
    // 1,3,0,5 i=0,1をswap: 転倒数2
    // 1,0,3,5 i=1,2をswap: 転倒数1 [2回目のループ終了後、2番目に大きい3が、後ろから2番目に来ている]
    // --------ループ3--------
    // 0,1,3,5 i=0,1をswap: 転倒数0 [3回目のループ終了後、3番目に大きい1が、後ろから3番目に来ている]
}


/// 配列の転倒数を求める関数: O(NlogN)
fn get_inv_num<T>(array: &Vec<T>) -> usize 
    where T: Ord + std::hash::Hash + Clone + Copy
{
    let n = array.len();

    // 調査済みの数列をBITに格納していく。　より具体的にいうと、
    // BIT の index は数列内における順位、 data にはその順位が何個登場しているか、を格納する。
    let mut bit = BinaryIndexedTree::new(n);
    // 入力配列を順位変換
    let (ranked_array, _, _) = rank_array(array);

    let mut inv_num = 0;
    for i in 0..n {
        // a[i]より大きい項がiより後ろに何個控えているか、を計算し、転倒数として加算していく

        // 数列全体におけるa[i]の順位
        let rank = ranked_array[i];
        
        if rank != 0 {
            // a[i]が転倒数に与える寄与 = 自分の順位 - 既に調査済みの自分より大きい数列の個数
            let inv_num_i = rank - bit.sum(rank-1);
            inv_num += inv_num_i;
        }
        // 調査済みの項を、BITに1個追加していく。
        bit.add(rank, 1);
    }
    return inv_num
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

