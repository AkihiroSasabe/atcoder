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
        mut a: [usize; n]
    }
    let MAX_VOLUME = 1_000_000_000;

    a.sort();
    let mut selected_a = vec![];
    let mut duplication: usize = 0;
    selected_a.push(a[0]);
    for i in 1..n {
        if a[i] != a[i-1] {
            selected_a.push(a[i]);
        }
        else {
            duplication += 1;
        }
    }

    // 最大で読める巻数
    let mut commic = 0;
    // 次読もうとしている本のindex
    let mut index = 0;
    // 今持っている中で最新巻のindex
    let mut last_index = selected_a.len() - 1;
    let mut left_book_num = a.len();

    loop {
        // 読み進められるとき
        if (index <= last_index) && (selected_a[index] == commic + 1) {
            commic += 1;
            index +=1;
            // println!("Have {}", commic);
        }
        // 本を売れるとき
        else if left_book_num > 1 {
            // ダブっている本を売れるとき
            if duplication >= 2 {
                duplication -= 2;
                left_book_num - 1;
                commic += 1;
                // println!("Sell two duplicated books and buy: {}", commic);
            }
            // ダブっている本を1冊と、ダブっていない本を1冊売れるとき
            else if duplication == 1 && last_index >= index {
                duplication -= 1;
                commic += 1;
                if last_index >= 1 {
                    last_index -= 1;
                }
                else {
                    break
                }
                // println!("Sell a duplicated book and a owned book, and buy: {}", commic);
            }
            // ダブっていない本を2冊売れるとき
            else if last_index > index {
                // 今持っている本を売れる
                left_book_num -= 1;
                commic += 1;
                if last_index >= 2 {
                    last_index -= 2;
                }
                else {
                    break
                }
                // println!("Sell two owned book and buy: {}", commic);
            }
            // 本をもう売れないとき
            else {
                break
            }
        }
        // 本を売れないとき(現在持っている単行本が 1 冊以下の場合、これ以上何も出来ない)
        else {
            break
        }
    }

    println!("{}", commic);

}