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
    // 2023-10-29 11:02-12:50 (1h48min)
    // 2023-10-29 17:18-17:46 (28min)
    // total (2h16min = 136min)

    // 最悪計算量 5x5のケースを考える
    // 1個目と2個目の制約で、i行目は
    // X = R[i]とすれば、下記の3パターンで20通りに絞られる。　ここでYとZはX以外の2つの文字で、'Y'と'Z'と'.'は、順番の入れ替えが可能である。
    // X[YZ..] -> []内の入れ替えは、4P2 = 4 * 3 = 12 通り
    // .X[YZ.] -> []内の入れ替えは、3P2 = 3 * 2 = 6 通り
    // ..X[YZ] -> []内の入れ替えは、2P2 = 2 * 1 = 2 通り
    // 
    // 1行が20通りで5行あるので、上2つの制約だけで得られる全マスの模様は、20^5 = 3_200_000 通りしか存在しない。
    // 1通りにつき、全マス分(5*5)の書き換えたりすると、
    // トータルの計算量は、20^5 * 25 = 80_000_000 < 10^8 になるので間に合う。

    // しょぼん氏や、kyopro_friendsのように、3重のpermutationで処理しても楽だったかもしれない。
    //　https://x.com/shobonvip/status/1718270214280970504?s=20
    // https://atcoder.jp/contests/abc326/submissions/47011446

    input! {
        n: usize,
        r: Chars,
        c: Chars,
    }
    let mut image = vec![vec!['.'; n]; n];

    let mut end_flag = false;
    dfs(&mut image, 0, &r, &c, &mut end_flag);

    if !end_flag {
        println!("No");
    }

    // 847_288_609_443

    // 4 ** 25 = 
    // 1_125_899_906_842_624

    // 5 * 4 * 3 = 60通り <- 1行
    // 60^4 = 12_960_000 通り。なんかいけそう

    // 20 ** 5 = 
    // 3_200_000

    // 5P3 = 5*4*3 * 2 = 60 * 2 = 120 // 5個の中から3個を選ぶ <- ある行(or列)
    // 5C3 = 5 * 4 * 3 / (3 * 2) = 10

    // 120^3 = 1_728_000
    // 120^5 = 24_883_200_000
}

fn dfs(img: &mut Vec<Vec<char>>, y: usize, r: &Vec<char>, c: &Vec<char>, end_flag: &mut bool) {
    if *end_flag {return;}
    let n = img.len();
    if y == n {
        if check_all(img, r, c) {
            *end_flag = true;
            println!("Yes");
            for i in 0..n {
                for j in 0..n {
                    print!("{}", img[i][j]);
                }
                println!("");
            }
        }
        return;
    }

    let top = r[y];
    let others = get_others(top);
    
    // 0列目がtopにした場合
    for perm in (1..n).combinations(2) {
        img[y] = vec!['.'; n];
        img[y][0] = top;
        img[y][perm[0]] = others[0];
        img[y][perm[1]] = others[1];
        dfs(img, y+1, r, c, end_flag);
        if *end_flag {return;}

        img[y] = vec!['.'; n];
        img[y][0] = top;
        img[y][perm[0]] = others[1];
        img[y][perm[1]] = others[0];
        dfs(img, y+1, r, c, end_flag);
        if *end_flag {return;}
    }

    // 1列目がtopにした場合
    for perm in (2..n).combinations(2) {
        img[y] = vec!['.'; n];
        img[y][1] = top;
        img[y][perm[0]] = others[0];
        img[y][perm[1]] = others[1];
        dfs(img, y+1, r, c, end_flag);
        if *end_flag {return;}

        img[y] = vec!['.'; n];
        img[y][1] = top;
        img[y][perm[0]] = others[1];
        img[y][perm[1]] = others[0];
        dfs(img, y+1, r, c, end_flag);
        if *end_flag {return;}
    }

    // 2列目がtopにした場合
    for perm in (3..n).combinations(2) {
        img[y] = vec!['.'; n];
        img[y][2] = top;
        img[y][perm[0]] = others[0];
        img[y][perm[1]] = others[1];
        dfs(img, y+1, r, c, end_flag);
        if *end_flag {return;}

        img[y] = vec!['.'; n];
        img[y][2] = top;
        img[y][perm[0]] = others[1];
        img[y][perm[1]] = others[0];
        dfs(img, y+1, r, c, end_flag);
        if *end_flag {return;}
    }

    img[y] = vec!['.'; n];

}


fn get_others(x: char) -> Vec<char> {
    if x == 'A' {
        return vec!['B', 'C'] 
    }
    else if x == 'B' {
        return vec!['C', 'A'] 
    }
    else if x == 'C' {
        return vec!['A', 'B'] 
    }
    else {
        return vec![]
    }
}

fn check_all(img: &Vec<Vec<char>>, r: &Vec<char>, c: &Vec<char>) -> bool {
    if !check_row(img, r) {
        return false;
    }
    if !check_col(img, c) {
        return false;
    }
    if !check_abc(img) {
        return false;
    }
    return true;
}

fn check_row(img: &Vec<Vec<char>>, r: &Vec<char>) -> bool {
    let n = img.len();
    for y in 0..n {
        let mut ok_flag = false;
        for x in 0..3 {
            if img[y][x] != '.' && img[y][x] != r[y] {
                return false
            }
            if img[y][x] == r[y] {
                ok_flag = true;
                break;
            }
        }
        if !ok_flag {
            return false;
        }
    }
    return true
}

fn check_col(img: &Vec<Vec<char>>, c: &Vec<char>) -> bool {
    let n = img.len();
    for x in 0..n {
        let mut ok_flag = false;
        for y in 0..3 {
            if img[y][x] != '.' && img[y][x] != c[x] {
                return false
            }
            if img[y][x] == c[x] {
                ok_flag = true;
                break;
            }
        }
        if !ok_flag {
            return false;
        }
    }
    return true
}

fn check_abc(img: &Vec<Vec<char>>) -> bool {
    let n = img.len();
    for y in 0..n {
        let mut a_flag = false;
        let mut b_flag = false;
        let mut c_flag = false;
        for x in 0..n {
            if img[y][x] == 'A' {
                if a_flag {
                    return false;
                }
                a_flag = true;
            }
            else if img[y][x] == 'B' {
                if b_flag {
                    return false;
                }
                b_flag = true;
            }
            else if img[y][x] == 'C' {
                if c_flag {
                    return false;
                }
                c_flag = true;
            }
        }
        if !(a_flag && b_flag && c_flag) {
            return false;
        }
    }

    for x in 0..n {
        let mut a_flag = false;
        let mut b_flag = false;
        let mut c_flag = false;
        for y in 0..n {
            if img[y][x] == 'A' {
                if a_flag {
                    return false;
                }
                a_flag = true;
            }
            else if img[y][x] == 'B' {
                if b_flag {
                    return false;
                }
                b_flag = true;
            }
            else if img[y][x] == 'C' {
                if c_flag {
                    return false;
                }
                c_flag = true;
            }
        }
        if !(a_flag && b_flag && c_flag) {
            return false;
        }
    }
    return true;

}