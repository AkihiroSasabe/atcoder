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
        n: usize,
        m: usize,
        a: [usize; n],
    }

    let mut graph = vec![vec![]; n];
    for i in 0..m {
        input! {
            u_i: usize,
            v_i: usize
        }
        graph[u_i - 1].push(v_i - 1);
        graph[v_i - 1].push(u_i - 1);
    }

    let mut heap = BinaryHeap::new();
    
    // [頂点iを切ったときのコスト、 頂点i]
    let mut cost = vec![vec![0]; n];
    for v in 0..n {
        cost[v].push(v);
        for next_v in graph[v].iter() {
            cost[v][0] += a[*next_v];
        }
    }
    for v in 0..n {
        heap.push(vec![-(cost[v][0] as isize), v as isize]);
    }

    // 頂点vを削除したらtrue
    let mut seen = vec![false; n];
    let mut ans = 0;
    let mut counter = 0;
    loop {
        let min_state = heap.pop().unwrap();
        let min_cost = (-min_state[0]) as usize;
        let min_v = min_state[1] as usize;
        if seen[min_v] {
            continue
        }
        else {
            seen[min_v] = true;
            ans = max(ans, min_cost);
            for next_v in graph[min_v].iter() {
                cost[*next_v][0] -= a[min_v];
                heap.push(vec![-(cost[*next_v][0] as isize), *next_v as isize]);
            }
            counter += 1;
            if counter == n - 1 {
                break
            }
        }
    }
    println!("{}", ans);



    // 下記の解法はTLEする。
    // cost.sort();
    // // [頂点i: 頂点iを切ったときのコスト]
    // let mut hash_map = HashMap::new();
    // for i in 0..n {
    //     hash_map.insert(cost[i][1], cost[i][0]);
    // }

    // // let INF = 1 << 60;
    // let mut ans = 0;
    
    // for i in 0..n {
    //     cost.sort();
    //     ans = max(ans, cost[i][0]);
    //     // 頂点vを削除したときに、costがどうなるか。
    //     let v = cost[i][1];
    //     let mut updated_index_and_cost = vec![];
    //     // 頂点vに隣接した頂点next_vと、next_vの更新後のコスト取得
    //     for next_v in graph[v].iter() {
    //         let next_cost = hash_map[next_v];
    //         let mut ok = n - 1;
    //         let mut ng = i;
    //         // 隣接した頂点は、何番目にコストが低いかを取得するための二分探索
    //         while ok - ng > 1 {
    //             let mid = (ng + ok) / 2;
    //             if cost[mid][0] < next_cost {
    //                 ng = mid;
    //             }
    //             else {
    //                 ok = mid;
    //             }
    //         }
    //         hash_map.insert(*next_v, next_cost - a[v]);
    //         updated_index_and_cost.push(vec![ok, next_cost - a[v]]);
    //     }
    //     for i in 0..updated_index_and_cost.len() {
    //         let updated_v = updated_index_and_cost[i][0];
    //         let updated_cost = updated_index_and_cost[i][1];
    //         cost[updated_v][0] = updated_cost;
    //     }
    //     cost[i][0] = 0;
    // }
    // // println!("{:?}", cost);
    // println!("{}", ans);

}
