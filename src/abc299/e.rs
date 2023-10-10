#![allow(dead_code, unused_imports)]
use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::BTreeMap;
use std::hash::Hash;
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::collections::HashSet;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    // 2023-10-09 9:51-13:07 (3h16min=196m)
    // 2023-10-09 16:20-19:37 (3h17min=297min)
    // total = 6h33min = 577min
    input! {
        n: usize,
        m: usize
    }
    let mut graph = vec![vec![]; n];
    for i in 0..m {
        input! {
            u_i: usize,
            v_i: usize,
        }
        graph[u_i-1].push(v_i-1);
        graph[v_i-1].push(u_i-1);
    }
    input! {
        k: usize
    }
    let mut p = vec![];
    let mut d = vec![];
    for i in 0..k {
        input! {
            p_i: usize,
            d_i: usize
        }
        p.push(p_i-1);
        d.push(d_i);
    }

    // colors[i]は、各頂点の色を表す。4つの色を用意しておく。
    // 0(白確定)
    // 1(黒確定), 
    // 2(黒候補),
    // 3(未確定, 初期値)
    let mut colors: Vec<usize> = vec![3; n];
    let mut grays: Vec<HashSet<usize>> = vec![HashSet::new(); n]; // grays[v] := 頂点vが持つ、gray(2) な頂点の集合
    let mut blacks: Vec<HashSet<usize>> = vec![HashSet::new(); n]; // blacks[v] := 頂点vが持つ、black(1) な頂点の集合
    let mut gray_owners: Vec<HashSet<usize>> = vec![HashSet::new(); n]; // gray_owners[v] := 頂点vをgray(2) としている、頂点の集合


    // まずはBFSでいいや。
    for i in 0..k {
        let pi = p[i];
        let di = d[i];
        // println!("----pi = {:?} ----", pi);
        
        let init_distance: usize = 1_000_000_000_000_000_000; // 10^18
        let mut queue = VecDeque::new();
        let mut distance = vec![init_distance; graph.len()];
        distance[pi] = 0;
        if di == 0 { // 黒じゃないといけない
            if colors[pi] == 0 { // 元が白
                // 矛盾が生じるのでこの時点でアウト
                println!("No");
                return;
            }
            else if colors[pi] == 1 { // 元が黒
                // 何もしない
            }
            else if colors[pi] == 2 { // 元が灰色
                // 黒にしていく
                gray_to_black(pi, &mut colors, &mut blacks, &mut grays, &mut gray_owners);
            }
            else if colors[pi] == 3 { // 元が未確定
                // 黒にしていく
                colors[pi] = 1;
                blacks[pi].insert(pi);
            }

        }
        else { // 白じゃないといけない
            if colors[pi] == 0 { // 元が白
                // 何もしない
            }
            else if colors[pi] == 1 { // 元が黒
                // 矛盾が生じるのでこの時点でアウト
                println!("No");
                return;
            }
            else if colors[pi] == 2 { // 元が黒候補
                // 白にしていく
                gray_to_white(pi, &mut colors, &mut grays, &mut gray_owners);
            }
            else if colors[pi] == 3 { // 元が未確定
                colors[pi] = 0;
            }

        }
        queue.push_back(pi);

        while queue.len() > 0 {
            let v = queue.pop_front().unwrap();
            // if distance[v] > di {break} // diより離れた頂点は、考慮しなくてよいのでループ終了

            for j in 0..graph[v].len() {
                let nv = graph[v][j];
                // 訪問済みならスキップ
                if distance[nv] != init_distance {continue}
                distance[nv] = distance[v] + 1;

                // 距離がdi未満なら白にしないといけない
                if distance[nv] < di {
                    if colors[nv] == 0 { // 元が白
                        // 何もしない
                        ()
                    }
                    else if colors[nv] == 1 { // 元が黒
                        // 矛盾が生じるのでこの時点でアウト
                        println!("No");
                        return;
                    }
                    else if colors[nv] == 2 { // 元が黒候補
                        // 白にしていく
                        gray_to_white(nv, &mut colors, &mut grays, &mut gray_owners);
                    }
                    else if colors[nv] == 3 { // 元が未確定
                        colors[nv] = 0;
                    }
                }
                else if distance[nv] >= di {
                    // nvを黒候補に入れていく
                    if colors[nv] == 0 {} // 元が白確定。何もしない
                    else if colors[nv] == 1 {} // 元が確定。何もしない
                    else if colors[nv] == 2 { // 元が黒候補。ownerを増やす。
                        grays[pi].insert(nv);
                        gray_owners[nv].insert(pi);
                    }
                    else if colors[nv] == 3 { // 元が未確定。そこを灰色にして、ownerを増やす。
                        colors[nv] = 2;
                        grays[pi].insert(nv);
                        gray_owners[nv].insert(pi);
                    }
                }
                queue.push_back(nv);
            }
        }
        // println!("distance = {:?}", distance);
    }

    // println!("colors = {:?}", colors);
    // println!("grays = {:?}", grays);
    // println!("blacks = {:?}", blacks);
    // println!("gray_owners = {:?}", gray_owners);


    // 全頂点についてチェック。(1.黒が1個以上いるか, 2.灰色と未確定を黒色に染める)
    let mut exist_black_flag = false;
    for i in 0..n {
        // gray だった場合
        if colors[i] == 2 {
            gray_to_black(i, &mut colors, &mut blacks, &mut grays, &mut gray_owners)
        }
        // 未確定だった場合
        if colors[i] == 3 {
            colors[i] = 1; // とりあえず黒にしておく
            // blacks[i].insert(pi);
        }

        if colors[i] == 1 {
            exist_black_flag = true;
        }
    }

    // piから距離diの位置に黒色が1個でもあるか確認するために、もう一回BFSをする
    for i in 0..k {
        let pi = p[i];
        let di = d[i];
        let init_distance: usize = 1_000_000_000_000_000_000; // 10^18
        let mut queue = VecDeque::new();
        let mut distance = vec![init_distance; graph.len()];
        distance[pi] = 0;
        queue.push_back(pi);
        let mut exist_black_on_di = false;
        if di == 0 && colors[pi] == 1 {
            exist_black_on_di = true;
        }

        while queue.len() > 0 {
            let v = queue.pop_front().unwrap();
            for j in 0..graph[v].len() {
                let nv = graph[v][j];
                if distance[nv] != init_distance {continue}
                distance[nv] = distance[v] + 1;
                if distance[nv] == di {
                    if colors[nv] == 1 {
                        exist_black_on_di = true;
                    }
                }
                queue.push_back(nv);
            }
        }

        if !exist_black_on_di {
            println!("No");
            return;
        }
    }
    
    if exist_black_flag {
        println!("Yes");
        // 全頂点についてチェック
        for i in 0..n {
            print!("{}", colors[i]);
        }
    }
    else {
        println!("No");
    }
}


// 頂点vを灰色から黒色にする
fn gray_to_black(v: usize, colors: &mut Vec<usize>, blacks: &mut Vec<HashSet<usize>>, grays: &mut Vec<HashSet<usize>>, gray_owners: &mut Vec<HashSet<usize>> ) {
    // grays[v] := 頂点vが持つ、gray(2) な頂点の集合
    // blacks[v] := 頂点vが持つ、black(1) な頂点の集合
    // gray_owners[v] := 頂点vをgray(2) としている、頂点の集合

    colors[v] = 1;
    for &owner in gray_owners[v].iter() {
        grays[owner].remove(&v);
        blacks[owner].insert(v);
    }
    gray_owners[v] = HashSet::new();
}


// 頂点vを灰色から白色にする
fn gray_to_white(v: usize, colors: &mut Vec<usize>, grays: &mut Vec<HashSet<usize>>, gray_owners: &mut Vec<HashSet<usize>> ) {
    // grays[v] := 頂点vが持つ、gray(2) な頂点の集合
    // blacks[v] := 頂点vが持つ、black(1) な頂点の集合
    // gray_owners[v] := 頂点vをgray(2) としている、頂点の集合

    colors[v] = 0;
    for &owner in gray_owners[v].iter() {
        grays[owner].remove(&v);
    }
    gray_owners[v] = HashSet::new();
}