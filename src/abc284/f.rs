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
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    // 2023-10-21 18:20-19:00 (40min)
    // 2023-10-25 19:01-20:01 (60min)
    input! {
        n: usize,
        t: Chars
    }


    let mut t_rev = t.clone();
    t_rev.reverse();


    // 以下ローリングハッシュによる解法 (ハッシュの衝突に注意。1 / modulusの確率で起きる。テストケースが甘いことを祈るのみ)
    // 適当な素数
    let modulus = 2_147_483_647; // 2 * 10^9

    // base(=100)の累乗を事前計算
    let base = 26; // baseの値は文字の種類数以上なら何でも良い
    let exponent = t.len(); // 指数は文字数-1だけ必要
    let power_of_base = get_power_of_base(base, exponent, modulus);

    // 先頭部分文字列のhash値を事前計算しておく (先頭からi文字目まで。i=0, 1, 2, ..., n-1)
    let first_hash_list = get_first_substring_hash(&t, base, modulus);
    let rev_first_hash_list = get_first_substring_hash(&t_rev, base, modulus);
    // let rev_hash = get_substring_hash(&rev_first_hash_list, 0, t.len() - 1, &power_of_base, modulus);


    // i = 0 と n-1 は別途考えた方が良さそう。

    let mut i_is_0 = true;
    for i in 0..n {
        if t[n-1-i] != t[i+n] {
            i_is_0 = false;
        }
    }
    if i_is_0 {
        for i in n..2*n {
            print!("{}", t[i]);
        }
        println!("");
        println!("0");
        return;
    }

    for i in 0..n-1 {
        // println!("---- i = {i} ----");
        let first_i = get_substring_hash(&first_hash_list, 0, i, &power_of_base, modulus);
        let last_nmi = get_substring_hash(&first_hash_list, i+n+1, 2*n-1, &power_of_base, modulus);
        let mid: usize = get_substring_hash(&rev_first_hash_list, 2*n-1-(i+n), 2*n-1-(i+1), &power_of_base, modulus);
        let sum = (first_i * power_of_base[1 + (2*n-1) - (i+n+1)] % modulus + last_nmi) % modulus;
        // println!("{} {}, {}, {}", first_i, last_nmi, mid, sum);
        if mid == sum {
            // 本当はここで、ハッシュ値が衝突している可能性があるので、チェックを掛ける必要がある (文字列が全部一致しているか。その頻度は、1/modulusで済む)
            for j in 0..i+1 {
                print!("{}", t[j]);
            }
            for j in i+n+1..2*n {
                print!("{}", t[j]);
            }
            println!("");
            println!("{}", i+1);
            return;
        }
    }
    println!("-1");
    return;

}



// ローリングハッシュ
fn get_substring_hash(first_hash_list: &Vec<usize>, left: usize, right: usize, power_of_base: &Vec<usize>, modulus: usize) -> usize {
    // 任意の部分文字列string[l,r]のhash値を取得する. rは閉区間(rを含む)ので注意。開区間ではない。
    // first_hash_list: r=0,1,2,...nのstring[0,r]について、
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

    // println!("left = {left}, right = {right}");

    if left == 0 {
        return first_hash_list[right];
    }
    let substring_hash = (modulus + first_hash_list[right] - first_hash_list[left-1] * power_of_base[right-left+1] % modulus) % modulus;
    
    return substring_hash
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

// hash: 他動詞: [1]細かく刻む, [2]〔文字列などを〕値に変換する
fn get_first_substring_hash(string: &Vec<char>, base: usize, modulus: usize) -> Vec<usize> {
    // 先頭からの部分文字列のhash値を取得する関数(ローリングハッシュの前処理)
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
    let mut first_hash_list = vec![0_usize; string.len()];
    first_hash_list[0] = (string[0] as usize - 'A' as usize + 1) % modulus;
    for i in 1..string.len() {
        // 文字を数字に変換('A'=1, 'B'=2, ...になるような感じで計算)
        let ci = (string[i] as usize - 'A' as usize + 1) % modulus;
        first_hash_list[i] = (first_hash_list[i-1] * base % modulus + ci) % modulus;
    }

    return first_hash_list
}

