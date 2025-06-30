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

    // let a = 5_usize.overflowing_mul(2);
    // println!("a = {:?}", a);

    // let x: usize = 1_000_000_000_000_000_000;
    // let a = x.overflowing_mul(1_000_000_000_000_000_000);
    // println!("a = {:?}", a);

    // let x: usize = 2;
    // let a = x.overflowing_mul(1_000_000_000_000_000_000);
    // println!("a = {:?}", a);
    // return;

    // 2025-06-29 15:16-16:36 (80min)
    // 競プロフレンズ氏のツイートをチラ見した。
    // 
    input! {
        l: usize,
        r: usize,
    }
    // ◆問題文
    // a[n] :=  1,2,...,n の最小公倍数
    // a[L], a[L+1], ..., a[R] の中に何種類の整数があるか?
    // ◆制約
    // R-L<=10^7
    // 1<=L<=R<=10^14

    // ◆解法のまとめ
    // a[x] (x=L+1,L+2...,R)が新規の数になる条件は、
    // x=p^b となる数値になること。
    // 例えば、{1,2,3,4}でa[L=3]=(2^1)*(3^1)のとき、
    // L+1=4 = 2^2となるので、、a[L+1=4]=(2^2)*(3^1) は
    // 新規の数になりえる。
    // 厳密に証明すると、もっとも素数の次数が高いときに、新規の数になるが、
    // 素数の次数が最高になる数xというのは、素因数が1種類になる。

    // ◆考察の過程 (試行錯誤がダラダラ書いてある。)
    // 素数が何個あるかって話.
    //
    // a[5] = (2,3,4,5) = 2*3*2*5=60

    // a[n] = X
    // a[n+1] = ?
    // つまり、a[L]さえ求まれば、a[L+1]を求めるのは、ユークリッドの互除法でlog(a[L])で計算可能。
    // a[L]をどうやって求めるかって話
    // 最小公倍数
    // a[L] 以下の素数を全部列挙???
    // a[L]を直接求めるのは無理かも?

    // a[L] = LCM(1,2,...,L)
    // a[L+1] = LCM(1,2,...,L,L+1) <- L+1という数字が、L以下の整数の積で割り切れたら既出。
    // a[L+2] = LCM(1,2,...,L,L+1,L+2)

    // L+1を素因数分解するのはきついって。。。O(√L)掛かってしまう。。。
    
    // L+1,...Rまでについて、L以下の寄与を計算しまくるとか?
    // √R以下の素数を全列挙し、そいつらにエラトステネスの篩をしまくる?
    // 10と11なら? L+1が素数だと1個増える?
    // 3と4なら? 
    // L+1,...,Rにあたらしい素数が存在するかって話。
    // あたらしい素数が登場したら+1
    
    // 素因数分解できそうな気がする。
    // p0*1,p0*2,p0*3,....,
    // 次数が最大のものは...?
    // p0^(1) * 1
    // p0^(1) * 2
    // p0^(1) * 3
    // p0^(2)
    // p0^(3)

    // 素数の次数が一番高くなる瞬間
    // A[i] = p^b だと1up

    // 新規の素数もでてくんねんな。p^1みたいなのはどうしようもなくないか。。。
    // LからR以下の素数はそもそもあるかって問題。
    // 素数判定は試し割り。
    // p0

    // √R以下の素数を全列挙
    let ps = sieve_of_eratosthenes((r as f64).sqrt() as usize + 10);
    // println!("ps = {:?}", ps);
    let mut set = BTreeSet::new();

    // L,..,Rの範囲で、 p^b (pは√R以下)で表現可能な数を　set　に格納.
    for &p in ps.iter() {
        let mut temp = p;
        while temp <= r {
            if l < temp {
                set.insert(temp);
            }
            // temp *= p;
            let (val, is_overflow)= temp.overflowing_mul(p);
            // println!("(p val, is_overflow) = {:?}", (p, val, is_overflow));
            temp = val;
            if is_overflow {break}
        }
    }
    // println!("---ok0");

    // あとは pが√Rより大きい素数がL,..,Rの中に何個あるかって話
    let mut is_ps= vec![true; r-l+1];
    for &p in ps.iter() {
        // d*p > L となる d を求める。
        let mut d = l / p + 1;
        while d * p <= r {
            let num = d * p;
            is_ps[num-l] = false;
            d += 1;
        }
    }
    // println!("---ok1");

    for i in 0..r-l+1 {
        if is_ps[i] {
            set.insert(i+l);
        }
    }
    println!("{}",set.len());




}




// エラトステネスの篩(ふるい)
// n以下の素数を全て列挙する為のアルゴリズムO(n*log(log(n)))
fn sieve_of_eratosthenes(n: usize) -> Vec<usize> {
    // is_prime_list[i] := iが素数なら true , そうでなければ false
    let mut is_prime_list = vec![true; n + 1]; // この初期化でO(N)かかる!
    is_prime_list[0] = false; // 0は素数ではない
    is_prime_list[1] = false; // 1は素数ではない

    // prime_list := n以下の素数を格納したリスト
    let mut prime_list = vec![];
    
    // ここの計算で、O(N/2 + N/3 + N/5 + N/7 + N/11 + ...)  = O(N (1/2 + 1/3 + 1/5 + 1/7 + 1/11 + ... )) = O(Nlog(logN))かかる。
    // ※素数の逆数和は、log(logN)と漸近していくため。自然数の逆数和は、logNに漸近する。
    for i in 2..(n+1) {
        if !is_prime_list[i] {continue}
        for j in 2..((n/i)+1) {
            // i の倍数が素数ではないことを記録
            is_prime_list[j * i] = false;
        }
        prime_list.push(i);
    }
    return prime_list;
}