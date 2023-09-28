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
use std::collections::BTreeSet;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    // 2023-09-27 21:08-22:00 (52 min)
    // 2023-09-28 12:26-12:52 (26 min)
    //            19:31-20:43 (72 min) <- セグメント木を使う決断に至る
    //                  total (150 min)
    input! {
        n: usize,
        m: usize,
        mut a: [[usize; m]; n],
    }

    // 考察
    // Sijの組み合わせだけで、N*(N-1)*...*1 = N!通り
    // くそわからねぇ。

    // 具体例を考えてみる
    // (N,M)=(3,2)
    // S1: 1 3
    // S2: 2 8
    // S3: 4 6
    // S1∪S2 = (1,2,3,8)
    // f(S1,S2) = 1+3=4

    // S1∪S3 = (1,3,4,6)
    // f(S1,S3) = 1+2=3
    
    // S2∪S3 = (2,4,6,8)
    // f(S2,S3) = 1+4=5

    // ans = f(S1,S2) + f(S1,S3) + f(S2,S3) = 4+3+5=12

    // 全集合からうまく考えられるか?
    // S1∪S2∪S3 = (1,2,3,4,6,8), |S1∪S2∪S3| = M * N <= 10^2*10^4=10^6
    // rank:         1,2,3,4,5,6
    // S1:           *   *
    // rank:           1,  2,3,4
    // S2:             *       *
    
    // S1={1,3}の1 より小さい奴が何個あるかって話 
    //     -> 0個. 1は常に1位なので、S1の1がansに寄与する値は、1 * (n-1) = 1 * (3-1) = 1*2=2
    // S1={1,3}の3 より小さい奴が何個あるか
    //     ->1と2だけなので、2個。1は常に比べられるから、S1∪Sj (j=2,3,...n)について+1になる。2は、1回だけ現れるので1+1
    //     -> S1の3がansに寄与する量は、(1+1) * (n-1) + 1 = 2*2+1=5
    
    // S2={2,8}の2 より小さい奴は何個あるか? (ただしS1の要素は除いた順位。)
    //     -> 0個。 2が寄与するのは、1 * (n-2) = 1 * (3-2) = 1
    // S2={2,8}の8 より小さい奴は何個あるか? (ただしS1の要素は除いた順位。)
    //     -> (2,4,6)の3個。
    //     -> 2は常に比べられる。 (1+1) * (n-2) = 2*1=2の寄与
    //     -> 4,6は1回ずつ寄与するので、1+1=2
    //     -> 結局、8の寄与はトータルで2+2=4
    // よって、Σ[2<j] f(S2, Sj) = 1 + 4 = 5

    solve(n, m, a);

}

fn solve(n: usize, m: usize, mut a: Vec<Vec<usize>>) {
    // 3つの順位を管理して解く (解法は競プロフレンズのものに似ている)
    // (1)普遍的な全体の順位                              <- HashMapで管理
    // (2)Si (i=1,2,...N-1)を逐次除いた可変な全体の順位    <- BITで管理 (更新が必要なのでHashMapで管理が難しい)
    // (3)Si内での順位                                   <- HashMapで管理

    // all := A_ijの全NM個の要素を格納 (all[x] := Aijの中でx番目に小さいもの)
    let mut all: Vec<usize> = vec![];
    // bit := A_ij の個数の累積和を格納 (indexは、Aijを昇順に並べたときのものに対応)
    let mut bit: BinaryIndexedTree<usize> = BinaryIndexedTree::new(n*m);
    
    // key: A_ij
    // value: [A_ijの集合Aにおける順位:universal_global_rank, A_ijの集合Si内での順位:local_rank]
    let mut hash: HashMap<usize, Vec<usize>> = HashMap::new();

    for i in 0..n {
        a[i].sort();
        for j in 0..m {
            all.push(a[i][j]);
            // 集合Si内でのAijの順位jを格納。集合A
            hash.insert(a[i][j], vec![0, j]);
        }
    }
    all.sort();
    for i in 0..n*m {
        // bitには順位を刻む
        bit.add(i, 1);
        (*(hash.get_mut(&all[i]).unwrap()))[0] = i;
    }

    let mut ans = 0;
    for set_i in 0..n {
        // set_i := 自分が所属する集合

        // temp_ranks := 集合Siの要素のグローバルなランクを格納
        let mut temp_ranks = vec![];
        for j in 0..m {

            let vector = hash.get(&a[set_i][j]).unwrap();
            
            // univ_global_rank := グローバルな順位
            let univ_global_rank = vector[0];
            
            // local_rank := 自分が所属する集合Siの中での順位
            let local_rank = vector[1];

            temp_ranks.push(univ_global_rank);

            // temp_global_rank: 全体における順位 (ただし、自分が所属する集合よりも前の奴は除外)
            let temp_global_rank = bit.sum(univ_global_rank) - 1;

            // diff := a[i][j]が正解に寄与する値
            let diff = (local_rank + 1) * (n - set_i - 1) + (temp_global_rank - local_rank);
            // println!("a[{set_i}][{j}] = {}, univ_global_rank = {univ_global_rank}, temp_global_rank = {temp_global_rank}, set_i = {set_i}, local_rank = {local_rank}, diff = {diff}", a[set_i][j]);
            ans += diff;
        }
        // 集合Siを削除していく
        for j in 0..m {
            bit.subtract(temp_ranks[j], 1);
        }
    }

    println!("{}", ans);
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

impl<T: Default + Copy + std::ops::AddAssign + std::ops::SubAssign + std::fmt::Debug> BinaryIndexedTree<T> {
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
}

