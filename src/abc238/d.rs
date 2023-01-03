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
    // 2023-01-04 水曜日 18:09-19:30 (1h21m)
    input! {
        t: usize
    }
    let mut a = vec![];
    let mut s = vec![];
    for i in 0..t {
        input! {
            a_i: usize,
            s_i: usize,
        }
        a.push(a_i);
        s.push(s_i);
    }
    for i in 0..t {
        let exist_flag = check(a[i], s[i]);
        if exist_flag {
            println!("Yes");
        }
        else {
            println!("No");
        }
    }
}

fn check(and: usize, sum: usize) -> bool {
    // x < 2^1: 1桁だけ見ればいい。
    // x < 2^60: 60桁だけ見ればいい。
    let mut exist_flag = true;
    let mut kuriage = 0; // 0: なし, 1: あり

    for digit in 0..60 {
        // andのdigit桁目が1のとき ((x,y) = (1,1))
        if (1 << digit) & and != 0 {
            // sumのdigit桁目が1のとき
            if (1 << digit) & sum != 0 {
                // 前回の繰り上げが0だと失格
                if kuriage == 0 {
                    exist_flag = false;
                    break
                }
            }
            // sumのdigit桁目が0のとき
            else {
                // 前回の繰り上げが1だと失格
                if kuriage == 1 {
                    exist_flag = false;
                    break
                }
            }
            kuriage = 1;
        }
        // andのdigit桁目が0のとき ((x,y) = (0,0),(1,0),(0,1))
        else {
            // sumのdigit桁目が1のとき
            if (1 << digit) & sum != 0 {
                // 繰り上げの条件はなんでも良い。
                // kuriage == 0 ならば、(x,y) == (0,1) or (1,0) である必要がある
                // kuriage == 1 ならば、(x,y) == (0,0) である必要がある
                kuriage = 0;
            }
            // sumのdigit桁目が0のとき
            else {
                // 繰り上げの条件はなんでも良い。
                // kuriage == 0 ならば、(x,y) == (0,0)
                // kuriage == 1 ならば、(x,y) == (1,0) or (0,1)
                if kuriage == 0 {
                    kuriage = 0;
                }
                else if kuriage == 1 {
                    kuriage = 1;
                }
            }
        }
        if digit == 59 {
            if kuriage == 1 {
                exist_flag = false;
            }
        }
    }
    return exist_flag
}

// 条件
// x & y == a
// x + y = s
// となるx,yはそんざいするか?
// t <= 10^5
// a,s < 2^60

// 例1
// 1 8
// a = 1 = 0001
// s = 8 = 1000
// x&y = 0001
// x+y = 1000
// x = ***1
// y = ***1
// x + y = 1100

// 例2
// a=4 = 100
// s=2 = 010
// x = 1*0
// y = 1*0

// ■1桁目のチェックok
// aの要請: (x,y) == (0,0) or (0,1) or (1,0)
// sの要請: (x,y) == (0,0)
// 次のkuriage = 0

// ■2桁目のチェック
// aの要請: (x,y) == (0,0) or (0,1) or (1,0)
// sの要請: (x,y) == (0,1) or (1,0) (kuriage == 0なので)
// 次のkuriage = 0

// ■3桁目のチェック
// aの要請: (x,y) == (1,1)
// sの要請: (x,y) == (1,1)
// 次のkuriage = 1

// ■4桁目のチェック
// aの要請: (x,y) == (0,0) or (0,1) or (1,0)
// sの要請: (x,y) == (0,1) or (1,0) (kuriage == 1なので)
// 次のkuriage = 1
// x=110
// y=100

// ... (これが60桁目まで続く...)

// ■60桁目のチェック
// 本文の制約による要請: s < 2^60なのに、kuriage=1なので破綻