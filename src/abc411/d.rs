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
    // 2025-06-21 22:28-22:40 (12min)
    // 2025-06-22 17:39-18:17 (38min)
    // 2025-06-22 18:17-18:49 (32min, デバッグ。別作業していたので、もう少し早く解けてたかも)
    // Total: 82 min
    input! {
        n: usize,
        q: usize,
    }

    let mut t = vec![];
    let mut p = vec![];
    let mut s = vec![];
    for i in 0..q {
        input!{
            ti: usize,
            pi: Usize1,
        }
        t.push(ti);
        p.push(pi);
        if ti == 2 {
            input! {
                si: Chars,
            }
            s.push(si);
        }
        else {
            s.push(vec![]);
        }
    }

    // ◆自力解法：
    // 2番目の操作だけが、既存の文字列のどこかを変える。
    // それ以外の操作は、ポインタを入れ替えているだけに過ぎない。
    // 
    // 2番目の操作で、追加される文字列をノードとすれば、グラフを構築できる。
    // エッジの根本（元からあった文字列）とエッジの先端（新しく追加する文字列）の関係は、1対多 の関係になるので、
    // グラフを前から後ろに辿ると、文字列は一意にならないが、
    // グラフを後ろから前に辿ると最終的に知りたい文字列が一意で求まる。
    // 1番目と3番目の操作で保持するポインタは、文字列のノードの先端と末端を指すようにすればいい。
    // ◆計算量：
    // 1番目と3番目の操作は、1回にO(1)で全体でO(Q)、2番目の操作は文字列の移動が伴うので、O(Σ|si|)
    // よって、O(Q+Σ|si|)で解ける。

    let num = n+1+q; // n個のPC + 1個のサーバー + Q個のクエリ
    
    // strs[i] := i個目の文字列。
    // i=0~n-1はPCの初期の空文字列。
    // i=nはサーバーの初期の空文字列。
    // i=n+1~n+qは、クエリの文字列。
    let mut strs = vec![vec![]; num];

    // pointers[i] = (s0,e0) はi番目のPCが、strs[s0]からstrs[e0]までを保持している。ただし、インデックスnはサーバーを表す。 
    let mut pointers = vec![(n+1, n+1); n+1];
    for i in 0..n+1 {
        pointers[i] = (i, i);
    }
    let mut rev_graph = vec![vec![]; num];

    for i in 0..q {
        let ti = t[i];
        let pi = p[i];
        if ti == 1 {
            // PC pi の文字列をサーバーの文字列で置き換え
            pointers[pi] = pointers[n];
        }
        else if ti == 2 {
            // PC pi の文字列の末尾にsiを追加
            strs[n+1+i] = s[i].clone();
            let (start, end) = pointers[pi];
            pointers[pi].1 = n+1+i;
            rev_graph[n+1+i].push(end);
        }
        else if ti == 3 {
            // serverをPC piで置き換え 
            pointers[n] = pointers[pi];
        }
    }

    let (start, end) = pointers[n];
    let mut ans = vec![];
    let mut now = end;
    loop {
        ans.push(strs[now].clone());
        if now == start {
            break;
        }
        let next = rev_graph[now][0];
        now = next;
    }
    ans.reverse();
    for vec_char in ans {
        print!("{}", vec_char.iter().collect::<String>());
    }
    println!("");
}