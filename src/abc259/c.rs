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
fn main() {
    input! {
        s: Chars,
        t: Chars
    }

    let mut flag = true;
    let mut s_i = 0;


    for i in 0..t.len() {
        // Sの末尾にこれ以上アクセスできないケース。下記のような例
        // S: bb
        // T: bbb
        if s.len() <= s_i {
            if s[s_i - 1] == t[i] && s[s_i - 1] == s[s_i - 2] {
                // pass
            }
            else {
                flag = false;
                break
            }
        }
        // 一致
        else if t[i] == s[s_i] {
            s_i += 1;
        }
        // 不一致
        else {
            // 前方2文字がないと、挿入ができないので、s_i>=2が必要
            if s_i >= 2 {
                // 挿入できるケース
                if s[s_i - 1] == t[i] && s[s_i - 1] == s[s_i - 2] {
                    // pass
                    // println!("Sonyu!!! s_i: {},  t[i]: {}", s_i, t[i]);
                }
                // 挿入できないケース
                else {
                    flag = false;
                    break;
                }
            }
            else {
                flag = false;
                break;
            }
        }
    }

    // Sの途中まではTと一致しているが、Sの最後が一致していないケースはNG (この条件は無くてもACする)
    if s.len() > s_i + 1 {
        flag = false;
    }

    if flag {
        println!("Yes");
    }
    else { 
        println!("No");
    }
}