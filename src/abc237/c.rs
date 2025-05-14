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
        s: Chars
    }
    let n = s.len();
    let num_unique_char = 26;

    let mut s2 = vec![];
    for i in 0..s.len() {
        s2.push('a');
    }
    for i in 0..s.len() {
        s2.push(s[i]);
    }

    let mut rev_s2 = s2.clone();
    rev_s2.reverse();

    // ローリングハッシュの取得
    let hasher = RollingHash::new(&s2, num_unique_char);
    let rev_hasher = RollingHash::new(&rev_s2, num_unique_char);

    for i in (0..n).rev() {
        // 偶数長の回分のとき
        let length = n - i;
        let st = s.len() + i;
        let hash = hasher.get_substring_hash(st, st + length - 1);

        let rev_st = s.len() - i;
        let rev_hash = rev_hasher.get_substring_hash(rev_st, rev_st + length - 1);
        
        if rev_st + length - 1 >= n - 1 {
            if hash == rev_hash {
                println!("Yes");
                return;
            }
        }

        // 奇数長の回分のとき
        let rev_st = s.len() - 1 - i;
        let rev_hash: (usize, usize) = rev_hasher.get_substring_hash(rev_st, rev_st + length - 1);
        if rev_st + length - 1 >= n - 1 {
            if hash == rev_hash {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");






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
        cum_hash[0] = (string[0] as usize - 'a' as usize + 1) % modulus;
        for i in 1..string.len() {
            // 文字を数字に変換('A'=1, 'B'=2, ...になるような感じで計算)
            let ci = (string[i] as usize - 'a' as usize + 1) % modulus; // 最後の"+1"は、 "a" と "aa" を区別できるようにするため、"a"に1を割り当てている。
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