// 方針
// [S_1, S_2, ..., S_N]から好きな文字列を取り出す組み合わせは2^N通り => ビット全探索の問題
// 選んだ文字列の組み合わせで、26個のアルファベットの小文字(abcdefghijklmnopqrstuvwxyz)がそれぞれ何回呼ばれるかカウント
// 選んだ文字列の組み合わせ内、K個の文字列に登場する英小文字が最も多いものを取り出す
use proconio::input;
use proconio::marker::Chars;
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        k: usize
    }
    let mut s = vec![];
    for i in 0..n {
        input! {
            s_i: Chars
        }
        s.push(s_i);
    }
    // 小文字のアルファベット26文字分
    let small: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    // 選んだ文字列達の各組み合わせについて、各アルファベットが何回登場したかを格納するカウンター
    let mut counter = vec![vec![0; 26]; 1_usize << n];

    // どの文字列を選ぶかの組み合わせ(2^N通り)について、登場するアルファベットの数を数える
    // 計算量は2^N * N * 26 * N
    for bit in 0..(1_usize << n) {
        for i in 0..n {
            if (bit & (1_usize << i)) != 0 {
                for m in 0..26 {
                    for j in 0..s[i].len() {
                        if s[i][j] == small[m] {
                            counter[bit][m] += 1;
                            break
                        }        
                    }
                }
            }
        }
    }

    // k個に登場する文字の種類数(kinds)を数える。そのうち、最も多いものが答えとなる
    let mut kinds = 0;
    let mut max_kinds = 0;
    for bit in 0..(1_usize << n) {
        // println!("bit: {:?}: {:04b}", bit, bit);
        for m in 0..26 {
            if counter[bit][m] == k {
                kinds += 1;
                // println!("{}", small[m]);
            }
        }
        if max_kinds < kinds {
            max_kinds = kinds;
        }
        kinds = 0;
    }

    println!("{}", max_kinds);
}