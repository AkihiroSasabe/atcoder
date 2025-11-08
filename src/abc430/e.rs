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
    input! {
        t: usize,

    }
    let mut aa = vec![];
    let mut bb = vec![];
    for i in 0..t {
        input!{
            ai: Chars,
            bi: Chars,
        }
        aa.push(ai);
        bb.push(bi);
    }

    let mut a = vec![];
    let mut b = vec![];
    for i in 0..t {
        let mut ai = vec![];
        let mut bi = vec![];
        for j in 0..aa[i].len() {
            if aa[i][j] == '0' {
                ai.push(0);
            }
            else {
                ai.push(1);
            }

            if bb[i][j] == '0' {
                bi.push(0);
            }
            else {
                bi.push(1);
            }
        }
        a.push(ai);
        b.push(bi);
    }

    for i in 0..t {

        // println!("--- i= {} ---", i);
        let mut num_a: usize = 0;
        let mut num_b = 0;
        for j in 0..a[i].len() {
            if a[i][j] == 1 {
                num_a += 1;
            }
            if b[i][j] == 1 {
                num_b += 1;
            }
        }
        if num_a != num_b {
            println!("-1");
            continue;
        }

        // let num_unique_char = 26;
        let num_unique_char = 2;
        let hasher_a = RollingHash::new(&aa[i], num_unique_char);
        let hasher_b = RollingHash::new(&bb[i], num_unique_char);
        // // let b_rle = run_length_encoding(&b[i]);

        // let cum_a: Vec<usize> = CumulativeSum::new(&a[i]);
        // let cum_b: Vec<usize> = CumulativeSum::new(&b[i]);

        let mut is_ok = false;
        let n = a[i].len();
        let mut ans = n;
        // println!("ans = {:?}", ans);

        for left in 0..a[i].len() {
            // if a[i][left] != b[i][0] {
            //     continue;
            // }
            let hash_a = hasher_a.get_substring_hash(left, n - 1);
            let hash_b = hasher_b.get_substring_hash(0, n - 1 - left);
            // println!("left = {:?}", left);
            // println!("hash_a = {:?}, hash_b = {:?}", hash_a, hash_b);
            if hash_a != hash_b {
                continue;
            }

            if left != 0 {
                let hash_a = hasher_a.get_substring_hash(0, left-1);
                let hash_b = hasher_b.get_substring_hash(n-left, n-1);
                // println!("hash_a = {:?}, hash_b = {:?}", hash_a, hash_b);

                if hash_a != hash_b {
                    continue;
                }
            }

            ans = min(ans, left);
            is_ok = true;
        }
        if is_ok {
            println!("{}", ans);
        }
        else {
            println!("-1");
        }
    }



}



/// 累積和の処理に関するトレイト
pub trait CumulativeSum<T> {
    fn new(data: &[T]) -> Self;
    fn range_sum(&self, l: usize, r: usize) -> T;
}

impl<T> CumulativeSum<T> for Vec<T>
where
    T: Copy + Default + std::ops::Sub<Output = T> + std::ops::Add<Output = T>,
{
    // 配列の累積和の配列を求める関数 (型の注釈は必須なので注意)
    // let cum: Vec<usize> = CumulativeSum::new(&array); // のように使う 
    fn new(array: &[T]) -> Self {
        let mut cum = Vec::with_capacity(array.len());
        let mut sum = T::default();
        for &value in array {
            sum = sum + value;
            cum.push(sum);
        }
        cum
    }
    // [L, R] の和を求める関数
    fn range_sum(&self, l: usize, r: usize) -> T {
        if r < l {
            return T::default(); // Tのデフォルト値 (通常は0) を返す
        }
        if l == 0 {
            return self[r]
        } else {
            return self[r] - self[l - 1]
        }
    }
}

fn run_length_encoding<T: PartialEq + Clone>(s: &Vec<T>) -> Vec<(T, usize)> {
    // 配列sを、ランレングス圧縮する関数.
    // rle := (要素, 連続長さ) の配列を返す

    let mut rle: Vec<(T, usize)> = vec![];
    let mut count: usize = 0;
    for i in 0..s.len() {
        count += 1;
        if i + 1 == s.len() || s[i] != s[i + 1] {
            rle.push((s[i].clone(), count));
            count = 0;
        }
    }
    return rle
}


// ローリングハッシュ
// 部分文字列を、ハッシュ値に置き換える。2つの文字列の一致判定などに便利。
// 初期化: O(N) (文字列の長さをNとする。)
// クエリ: O(1): 部分文字列S[l,r]のハッシュ値の取得
// 実装参考：鉄則本と、けんちょんのブログ https://drken1215.hatenablog.com/entry/2019/09/16/014600
struct RollingHash {
    modulus_0: usize,
    modulus_1: usize,
    cum_hash_0: Vec<usize>,         // 先頭からの部分文字列のhash値
    cum_hash_1: Vec<usize>,         // 先頭からの部分文字列のhash値
    power_of_base_0: Vec<usize>,    // baseの累乗を格納したリスト (mod modulus_0)
    power_of_base_1: Vec<usize>     // baseの累乗を格納したリスト (mod modulus_1)
}

impl RollingHash {
    /// 初期化 (stringは、英小文字の文字列しか受け付けないので注意。大文字は小文字に変換して入力すること。)
    pub fn new(string: &Vec<char>, num_unique_char: usize) -> Self {
        let modulus_0: usize = 2_147_483_647; // 約 2 * 10^9
        let modulus_1: usize = 1_000_000_007; // 約 10^9

        let base = num_unique_char + 1; // baseの値は文字の種類数以上なら何でも良い。アルファベット(英小文字)なら26+1進法なので27。
        let exponent = string.len(); // 指数は文字数-1あれば十分

        // base の累乗を事前計算 (modulus_0, modulus_1 を法とする。)
        // RollingHashインスタンスを大量に求めるとき、累乗はまとめて1回で済ませた方が効率はいいが、計算量のオーダーは不変で、定数倍の差にしかならないので気にしない。)
        let power_of_base_0 = Self::get_power_of_base(base, exponent, modulus_0);
        let power_of_base_1 = Self::get_power_of_base(base, exponent, modulus_1);

        // cum_hash[i] := 先頭からi文字目までの、部分文字列のハッシュ値
        let cum_hash_0 = Self::get_cum_hash(string, base, modulus_0);
        let cum_hash_1 = Self::get_cum_hash(string, base, modulus_1);

        return RollingHash {
            modulus_0, modulus_1, 
            cum_hash_0, cum_hash_1, 
            power_of_base_0, power_of_base_1
        }
    }

    /// 部分文字列string[l,r]のhash値を取得する. rは閉区間(rを含む)ので注意。開区間ではない。
    pub fn get_substring_hash(&self, left: usize, right: usize) -> (usize, usize) {
        // cum_hash: r=0,1,2,...nのstring[0,r]について、
        //     先頭文字列のハッシュ値を事前に計算して格納したリスト
        // left: 先頭index
        // right: 末尾のindex
        // power_of_base: 底の累乗を計算したリスト。最大指数は文字数-1
        // modulus: 素数。ハッシュ化時に数が膨大過ぎてオーバーフローするのを防ぐため、この素数の剰余を取る

        // 例
        // r = 5
        // l = 2
        //    0123456 <-index
        //    1234567 <-value
        // S="abcdefg"
        // h[5] = 1*base^5 + 2*base^4 + 3*base^3 + 4*base^2 + 5*base^1 + 6*base^0 <-先頭から6文字目までのハッシュ値
        // h[1] = 1*base^1 + 2*base^0 <-先頭から2文字目までのハッシュ値
        // h[2,5] = h[5] - h[1] * base^(4)       <- 具体例
        // h[l,r] = h[r] - h[l-1] * base^(r-l+1) <- 一般化

        if left == 0 {
            let substring_hash_0 = self.cum_hash_0[right];
            let substring_hash_1 = self.cum_hash_1[right];
            return (substring_hash_0, substring_hash_1)
        }

        let substring_hash_0 = (self.modulus_0 + self.cum_hash_0[right] - self.cum_hash_0[left-1] * self.power_of_base_0[right-left+1] % self.modulus_0) % self.modulus_0;
        let substring_hash_1 = (self.modulus_1 + self.cum_hash_1[right] - self.cum_hash_1[left-1] * self.power_of_base_1[right-left+1] % self.modulus_1) % self.modulus_1;
        
        return (substring_hash_0, substring_hash_1)
    }

    /// 先頭からの部分文字列のhash値のリストを取得する関数(ローリングハッシュの前処理)
    fn get_cum_hash(string: &Vec<char>, base: usize, modulus: usize) -> Vec<usize> {
        // hash: 他動詞: [1]細かく刻む, [2]〔文字列などを〕値に変換する
        // <=> string[0,r] (r=0,1,2,...n)についての文字列のハッシュ値を格納したリストを返す
        
        // 入力引数
        // string: 文字列
        // base: 底. base進法. baseの値は文字の種類数以上なら何でも良い
        // modulus: 法


        // ハッシュ化の方法: 
        // 長さNで使用文字種類数Bの文字列は、
        // N桁のB進法(例えば100進法. Bは登場する文字の種類数以上なら何でも良い。アルファベットならB=26で良い。)の数字で一意に表現可能。
        // ハッシュ値 = 100^(N-1) * string[0] + 100^(N-2) * string[1] + ... + 100^(0) * string[N-1]
        // ただしB=100でN=1,000文字の文字列は、ハッシュ値が(100^1000)-1=(10^2000)-1で、64bit(≒10^19)を遥かに超えてしまう。

        // そこで適当な素数Mで割った余りをハッシュ値とする。
        // ハッシュ衝突する確率(異なる文字列が同じハッシュ値になる確率)は1/Mであり非常に小さいため、
        // ハッシュ衝突したときだけ。生の文字列のまま比較する。
        
        // left=0, right=i (i=0, 1, ..., n-1)を計算
        let mut cum_hash = vec![0_usize; string.len()];
        cum_hash[0] = (string[0] as usize - '0' as usize + 1) % modulus;
        for i in 1..string.len() {
            // 文字を数字に変換('A'=1, 'B'=2, ...になるような感じで計算)
            let ci = (string[i] as usize - '0' as usize + 1) % modulus; // 最後の"+1"は、 "a" と "aa" を区別できるようにするため、"a"に1を割り当てている。
            cum_hash[i] = (cum_hash[i-1] * base % modulus + ci) % modulus;
        }

        return cum_hash
    }

    // ローリングハッシュで、文字列をべき乗級数として計算する際に必要な事前計算
    fn get_power_of_base(base: usize, exponent: usize, modulus: usize) -> Vec<usize> {
        // baseのべき乗を事前計算する
        // base: 底
        // exponent: 最大指数. 正確にはexponent-1乗まで求める
        // modulus: 法

        let mut power_of_base =  vec![1_usize; exponent];
        for i in 1..exponent {
            power_of_base[i] = power_of_base[i-1] * base % modulus;
        }
        return power_of_base
    }

}
