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
    // 2024-04-24 20:55-21:35 (40min)降参
    // 2024-04-25 19:48-22:00 (2h12m)
    // 2024-04-27 10:00-10:47 (47min) はまやん解説をみた
    // 3h39min
    input! {
        s: Chars
    }
    // tle_solve(s);
    // return;

    // 個数を数える
    // (i,j)
    // r[i] := s[i..n]を2019で割った余り とすれば、
    // i~jを2019で割った余りって何?
    // S[i..n] - S[j..n]
    // r[i] - r[j] = s[i..j]を2019で割った余り
    // 2018_2018
    // 20194038

    // let caster = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

    // 12114141335 をデバッグして。答えは3

    let n = s.len();
    let mut r = vec![0; n];
    r[n-1] = s[n-1] as isize - '0' as isize;
    let mut num_r: Vec<usize> = vec![0; 2019];
    num_r[0] = 1;
    let mut power = 1;
    let mut ans = 0;
    for i in (0..n).rev() {
        let digit_i = s[i] as isize - '0' as isize;
        if i == n-1 {
            r[i] = (digit_i * power) % 2019;
        }
        else {
            r[i] = (r[i+1] + digit_i * power) % 2019;
        }
        ans += num_r[r[i] as usize];
        num_r[r[i] as usize] += 1;
        power *= 10;
        power %= 2019;
    }
    // println!("num_r = {:?}", num_r);
    // println!("r = {:?}", r);
    println!("{}", ans);

}

fn tle_solve(s: Vec<char>) {

    let n = s.len();
    let mut cum = vec![0; n];
    let mut power = 1;
    let mut ans = 0;
    for i in (0..n).rev() {
        let di = s[i] as isize - '0' as isize;
        if i == n-1 {
            cum[i] = di * power;
        }
        else {
            cum[i] = cum[i+1] + di * power;
        }
        power *= 10;
        for j in i+1..n {
            let r = cum[i] - cum[j];
            if r % 2019 == 0 {
                println!("i, j = {}, {:?}", i, j);
                ans += 1;
            }
        }
        if cum[i] % 2019 == 0 {
            println!("i,j = {}, {} ", i, -1);
            ans += 1;
        }
    }
    println!("cum = {:?}", cum);
    println!("{}", ans);

}


fn wa_solve(s: Vec<char>) {
    // 2019の倍数で、0を含まない数字をXとする。Xは、5桁の数字から。
    // 10桁以上の数字は、2つの5桁のX(x0, x1)で、x0 + x1 * 10^5と表現することが可能である。

    // sets[d] := d桁のXの集合
    let mut sets = vec![BTreeSet::new(); 10];
    for i in 1..10_000_000_000 {
        let cand = i * 2019;
        // if cand > 10_000_000_000 {break} // 10桁
        if cand > 1_000_000_000 {break} // 9桁
        // if cand > 100_000 {break} // 5桁
        if !contain_0(cand) {
            let num_digits = get_num_digits(cand);
            sets[num_digits].insert(cand);
        }
    }
    // println!("sets[5] = {:?}", sets[5]);
    // println!("sets[6] = {:?}", sets[6].iter().next().unwrap());
    // println!("sets[7] = {:?}", sets[7].iter().next().unwrap());
    // println!("sets[8] = {:?}", sets[8].iter().next().unwrap());
    // println!("sets[9] = {:?}", sets[9].iter().next().unwrap());
    // 例
    // sets[5] = {12114, 14133, 16152, 18171, 24228, 26247, 28266, 34323, 36342, 38361, 42399, 44418, 46437, 48456, 52494, 54513, 56532, 58551, 62589, 66627, 68646, 72684, 76722, 78741, 82779, 84798, 86817, 88836, 92874, 94893, 96912, 98931}
    // 1_942_803 個 = 2*10^6 個

    // Debug: 複数桁を有する、10
    // let lists = get_lists(&s, &sets);
    // let sets_copy = sets.clone();
    // for i in 6..10 {
    //     for &num in sets_copy[i].iter() {
    //         let ss = num_to_chars(num);
    //         // println!("num = {:?}", num);
    //         // println!("ss = {:?}", ss);
    //         let lists = get_lists(&ss, &sets);
    //         for j in 1..lists.len() {
    //             if lists[j].len() != 0 {
    //                 // println!("")
    //                 println!("num = {:?}", num);
    //                 println!("lists = {:?}", lists);
    //             }
    //         }
    //     }
    // }
    // return;
    // let aa = num_to_chars(211);
    // println!("aa = {:?}", aa);
    // return;

    let lists = get_lists(&s, &sets);
    // println!("lists = {:?}", lists);
    
    let mut dp = vec![0; s.len()];
    let mut dp2: Vec<HashSet<usize>> = vec![HashSet::new(); s.len()];
    for left in (0..s.len()).rev() {
        // 区間の左端を、文字列の右端から掃引していく。
        dp[left] = lists[left].len();
        for &match_len in lists[left].iter() {
            if left + match_len >= s.len() {break}

            dp2[left].insert(left + match_len);

            // let mut is_skip = false;
            // for &aaa in dp2[left].iter() {
            //     if dp2[aaa].contains(&(left + match_len)) {
            //         is_skip = true;
            //         break;
            //     }
            // }
            // if !is_skip {
            //     dp[left] += dp[left + match_len];
            //     dp2[left].insert(left + match_len);
            // }
        }
    }
    

    let mut ans = 0;
    for i in 0..s.len() {
        ans += dp[i];
    }
    println!("{}", ans);

    // // println!("sets = {:?}", sets);
    // println!("sets[5] = {:?}", sets[5]);
    // println!("sets[6] = {:?}", sets[6]);

    // 長さ5で、2019の倍数で、0を含まないもの
    // 12114
    // 14133
    // 16152
    // 18171
    // 28266
    // ~~~~~~~~~~~
    // 30285
    // 32304
    // 34323
    // 36342
}

fn num_to_chars(mut x: usize) -> Vec<char> {
    let mut s = vec![];
    let converter = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

    if x == 0 {return vec!['0']}

    while x!= 0 {
        s.push(converter[x % 10]);
        x /= 10;
    }
    s.reverse();
    return s
}

fn get_lists(s: &Vec<char>, sets: &Vec<BTreeSet<usize>>) -> Vec<BTreeSet<usize>> {
    // lists[d] := 左からd桁目の数字を起点として、
    // 2019の倍数となる数字の桁数

    // 例えば、1817181712114 なら、
    // 0文字目が、18171 なので、5桁の2019の倍数
    // 4文字目が、18171 なので、5桁の2019の倍数
    // 8文字目が、12114 なので、5桁の2019の倍数
    // よって以下のようになる。
    // lists = [{5}, {}, {}, {}, {5}, {}, {}, {}, {5}, {}, {}, {}, {}]
    
    // 1つの桁につき、9桁分チェックする
    let mut lists = vec![];
    for i in 0..s.len() {
        let mut list = BTreeSet::new();
        let mut temp = 0;
        for j in 0..9 {
            if i+j >= s.len() {break}
            temp += s[i+j] as usize - '0' as usize;
            if sets[j+1].contains(&temp) {
                list.insert(j+1);
            }
            temp *= 10;
        }
        lists.push(list);
    }
    return lists
}

fn contain_0(mut x: usize) -> bool {
    while x != 0 {
        let r = x % 10;
        if r == 0 {return true}
        x /= 10;
    }
    return false
}

fn get_num_digits(mut x: usize) -> usize {
    let mut num_digits = 0;
    while x > 0 {
        num_digits += 1;
        x /= 10;
    }
    return num_digits
}