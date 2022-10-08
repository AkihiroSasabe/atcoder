use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut s: [String; n],
        mut t: [String; m],
    }

    // // Sの各単語の後ろに'_'をつける
    // for i in 0..s.len() {
    //     s[i].push('_');
    //     println!("{}", s[i]);
    // }
    s.iter_mut().for_each(|s_i| s_i.push('_'));

    // s_iと'_'を全て繋げた文字数を数える (末尾には'_'がないので、-1しておく。)
    // let mut min_x_length = 0;
    // for i in 0..s.len() {
    //     min_x_length += s[i].len();
    // }
    // min_x_length -= 1;
    let min_x_length = s.iter().map(|s_i| s_i.len()).sum::<usize>() - 1;
    // println!("min_x_length: {}", min_x_length); // min_x_length: 9
    
    // '_'を最小のxから何文字足せるか?
    let additional_underbar = 16 - min_x_length;

    // sに'_'を足せる分を足す。
    // let x_candidate = (0..additional_underbar).map(|_| String::from("_"));
    // for i in x_candidate {
    //     println!("i: {}", i);
    // }
    // for i in 0..additional_underbar {
    //     s.push(String::from("_"));
    // }
    // extendでイテレータをまとめてpushできる。
    s.extend((0..additional_underbar).map(|_| String::from("_")));
    // println!("s:{:?}", s); // s:["choku_", "dai_", "_", "_", "_", "_", "_", "_", "_"]
    s.sort();
    // println!("s:{:?}", s); // s:["_", "_", "_", "_", "_", "_", "_", "choku_", "dai_"]
    // 後でtを2分探索するので事前に辞書順にソートしておく。
    t.sort();
    while {
        // Vec<String>の要素を連結して一つのStringにする
        let mut merged =  String::from("");
        for i in s.iter() {
            merged.push_str(i);
        }
        // println!("merged: {}", merged); // merged: _______choku_dai_
        // 先頭と末尾の'_'を削除していく。
        let x = merged.trim_matches('_').to_string();
        // println!("x: {}", x); // x: choku_dai
        match t.binary_search(&x) {
            Ok(_) => (),
            _ => {  
                    if x.len() >= 3 {
                        println!("{}", x);
                        return;
                    }
                }
        }
        s.next_permutation()
    } {}
    println!("-1");

}

// fn main() {
//     // https://atcoder.jp/contests/abc268/submissions/34778739 参考
//     input! {
//         n: usize,
//         m: usize,
//         mut s: [String; n],
//         mut t: [String; m],
//     }
//     t.sort();

//     // for i in s.iter_mut() {
//     //     i.push('_');
//     // }
//     // for_each()はイテレータを返さない。自分自身について操作するか、ただprintするだけとか。
//     s.iter_mut().for_each(|s_i| s_i.push('_')); // for_each()は、for文を省略しただけ。

//     // println!("s:{:?}", s); // exameple2: s:["choku_", "dai_"]
//     // map()は各要素に関数を適応してイテレータを返す。
//     // xにあと何個の'_'を付け加えることが出来るか。 3 <= |x| <= 16
//     let add = (16 + 1) - s.iter().map(|s| s.len()).sum::<usize>(); // 最後の+1は末尾のS_N'には'_'がつかない分を差し引いている。
//     // println!("add: {}", add); // example2: add: 9

//     // // extend()メソッドは、String型の文字列の末尾に、char型の複数の文字を付け加えることが出来る。
//     // // You can extend a String with some chars. Example:
//     // let mut message = String::from("abc");
//     // message.extend(['d', 'e', 'f'].iter());
//     // assert_eq!("abcdef", &message);

//     // filter_map()メソッドは、map -> filter -> mapを順にする処理と同じ。
//     // Option のイテレータについて、内側の Some を剥がす。
//     // let aaa = ["1", "two", "NaN", "four", "5"];
//     // let mut iter = aaa.iter().filter_map(|s| s.parse().ok());
//     // println!("{:?}", iter);

//     // // 「失敗するかもしれない(Result を返す)関数でmapし、その結果から Ok なものだけを取り出して、その中身を列挙するイテレータを返す」
//     // let bbb = ["1", "two", "NaN", "four", "5"];
//     // let mut iter = bbb.iter().map(|s| s.parse()).filter(|s| s.is_ok()).map(|s| s.unwrap());
//     // println!("{:?}", iter);
//     // assert_eq!(iter.next(), Some(1));
//     // println!("{:?}", iter);
//     // assert_eq!(iter.next(), Some(5));
//     // // assert_eq!(iter.next(), None);
//     // println!("{:?}", iter);


//     // // flat_map()は、ネストしたイテレータについて、内側のイテレータを剥がす(unwrap()する)
//     // // イテレータに対して、｛要素を受け取ってイテレータ(に変換できるもの)を返す関数｝を適用し、
//     // // 出てきたそれぞれのイテレータを順に繋げて返す関数
//     // let words = ["alpha", "beta", "gamma"];
//     // chars() returns an iterator. chars()で、&strをchar型のイテレータに分解出来る
//     // let merged: String = words
//     //     .iter()
//     //     .flat_map(|s| s.chars())
//     //     .collect();
//     // println!("{:?}", merged);
//     // assert_eq!(merged, "alphabetagamma");

//     // flat_map()は下記のように書き換えられる。
//     // let mut merged = String::from("");
//     // for temp_iter in words {
//     //     for i in temp_iter.chars() {
//     //         merged.push(i);
//     //         // println!("{:?}", i);
//     //     }
//     // } 
//     // println!("{:?}", merged);

//     // extend()はcharを要素とするイテレータを、String型の末尾に追加
//     s.extend((0..add).map(|_| String::from("_")));
//     // println!("s: {:?}", s); // s: ["choku_", "dai_", "_", "_", "_", "_", "_", "_", "_"]
//     s.sort();
//     // println!("s: {:?}", s); // s: ["_", "_", "_", "_", "_", "_", "_", "choku_", "dai_"]
//     // s.next_permutation();
//     // println!("s: {:?}", s); // s: ["_", "_", "_", "_", "_", "_", "_", "dai_", "choku_"]
//     // s.next_permutation();
//     // println!("s: {:?}", s); // s: ["_", "_", "_", "_", "_", "_", "choku_", "_", "dai_"]

//     // let mut xxx = vec![1,0,0];
//     // let mut yyy = vec![1,2,3];
//     // let mut count = 0;
//     // while count != 10 {
//     //     println!("xxx: {:?}", xxx);
//     //     println!("yyy: {:?}", yyy);
//     //     xxx.next_permutation();
//     //     yyy.next_permutation();
//     //     count += 1;
//     // }
//     // return;
    
//     while {
//         // flat_map()は、ネストしたイテレータについて、内側のイテレータを剥がす(unwrap()する)
//         // 出てきたそれぞれのイテレータを順に繋げて返す関数
//         let a = s.iter().flat_map(|s| s.chars()).collect::<String>();
//         // println!("a: {}", a);                                   // a: choku_dai________
//         // trim_matches()は、指定したprefixとsuffixを除去する。
//         let a = a.trim_matches('_').to_string();
//         // println!("b: {}", a);                                   // b: choku_dai
//         if a.len() >= 3 && t.binary_search(&a).is_err() {
//             // println!("{:?}", t.binary_search(&a));              // Err(2)
//             // println!("a: {}", a);                               // a: dai________choku
//             // println!("t: {:?}", t);                             // t: ["choku_dai", "chokudai"]
//             println!("{}", a);
//             return;
//         }
//         // println!("before s: {:?}", s);
//         // next_permutation()はboolを返す。辞書順で最後の順列を返すときは、falseを返し、それ以外はtrueを返す 
//         s.next_permutation()
//         // println!("after s: {:?}", s);
//     } {}
//     println!("-1");
// }



// 当日の自分の解答。デバッグできていない。
// fn main() {
//     input! {
//         n: usize,
//         m: usize,
//         mut s: [Chars; n],
//         t: [Chars; m],
//     }

//     // let mut t_bunkai = vec![vec![]; m];
//     let mut t2 = vec![vec![]; m];
//     let INF = 1 << 60;
    
//     // 単語群tのループ    
//     for i in 0..m {
//         println!("{:?}", t[i]);
//         let mut start = 0;

//         // 単語t[i]で'_'を探索
//         let mut whole_match_flag = true;
//         for j in 0..t[i].len() {
//             if t[i][j] == '_' {
//                 if j - start != 0 {
//                     println!("uooo");
//                     whole_match_flag = false;
//                     // 単語群sの単語とマッチするか?
//                     for s_i in 0..s.len() {
//                         let match_flag = judge_match(&s[s_i], &t[i][start..j].to_vec());
//                         if match_flag {
//                             t2[i].push(s_i);
//                             println!("shaaaa: i: {}, t2[i]:{:?}",i, t2[i]);
//                             whole_match_flag = true;
//                             break
//                         }
//                         else {
//                             println!("ERROR")
//                         }
//                     }
//                     if !whole_match_flag {
//                         break
//                     }
//                 }
//                 else {
//                     t2[i].push(INF);
//                 }
//                 start = j+1;
//             }
//         }
//         println!("nnn: i: {}, t2[i]:{:?}",i, t2[i]);
//         // 最後のワードが一致するか?
//         if whole_match_flag {
//             if start == t[i].len() {
//                 if t[i][start - 1] == '_' && t[i][start - 2] != '_' {
//                     t2[i].push(INF);
//                 }
//             }
//             // 最後が"_"で終わる場合は、考えなくて良い
//             else {
//                 let mut last_word_match_flag = false;
//                 for s_i in 0..s.len() {
//                     println!("eee: {:?}, {:?}, start: {}", s[s_i], t[i][start..t[i].len()].to_vec(), start);
//                     if judge_match(&s[s_i], &t[i][start..t[i].len()].to_vec()) {
//                         t2[i].push(s_i);
//                         last_word_match_flag = true;
//                     }
//                 }
//                 if !last_word_match_flag {
//                     whole_match_flag = false;
//                 }
//             }

//         }
//         if !whole_match_flag {
//             t2[i] = vec![];
//         }
//         println!("t2[i]: {:?}", t2[i]);

//     }

//     let mut total_length = 0;
//     for i in 0..n {
//         total_length += s[i].len();
//     }
//     total_length += (n-1);
//     let extra_underbar_num = 16 - total_length;
//     let mut s_encode: Vec<usize> = (0..s.len()).collect();
//     for i in 0..extra_underbar_num {
//         s_encode.push(INF);
//         for perm in s_encode.iter().permutations(s_encode.len()) {
//             let mut flag = true;
//             for j in 0..t2.len() {
//                 if perm == t2[j] {
//                     flag = false;
//                     break
//                 }
//                 // println!("perm: {:?}, t2[j]: {:?}", perm, t2[j]);
//             }
//         }
//     }

// }


// fn judge_match(word1: &Vec<char>, word2: &Vec<char>) -> bool {
//     let mut flag = true;
//     if word1.len() != word2.len() {
//         flag = false;
//     }
//     else {
//         for i in 0..word1.len() {
//             if word1[i] != word2[i] {
//                 flag = false;
//                 break
//             }
//         }
//     }
//     return flag
// }


// fn judge_prefix(prefix: &Vec<char>, word: &Vec<char>) -> bool {
//     let mut flag = true;
//     if prefix.len() > word.len() {
//         flag = false;
//     }
//     else {
//         for i in 0..prefix.len() {
//             if prefix[i] != word[i] {
//                 flag = false;
//                 break
//             }
//         }
//     }
//     return flag
// }