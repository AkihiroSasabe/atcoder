use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use proconio::marker::Chars;

fn main() {
    input! {q: usize}
    let mut query = vec![];

    for _ in 0..q {
        input! {n: usize}
        match n {
            1 => {  input! {x: usize};
                    query.push(vec![n, x])
                },
            2 => {  input! {x: usize, c: usize};
                    query.push(vec![n, x, c])
                },
            3 => (  query.push(vec![n])
                ),
            _ => (),
        }
    }

    let mut hash_map = HashMap::new();
    let mut heap_min = BinaryHeap::new();
    let mut heap_max = BinaryHeap::new();
    
    // let mut s_max = 0;
    // let mut s_min = 10_usize.pow(9_u32);
    for i in query {
        match i[0] {
            1 => {  // 1 x : S に x を 1 個追加する。
                    if hash_map.contains_key(&i[1]) {
                        // 既にhash_mapに存在するなら+1
                        *hash_map.get_mut(&i[1]).unwrap() += 1;
                    }
                    else {
                        // hash_mapに存在しなければ新規に追加
                        hash_map.insert(i[1], 1);
                    }
                    
                    
                    // heapに追加する 
                    // heap_min.push(MinHeapVertex::new(i[1]));
                    heap_min.push(- (i[1] as isize));

                    heap_max.push(i[1] as isize);
                },
            2 => {  // 2 x c : S から x を min(c, (S に含まれる x の個数 )) 個削除する。
                    // hash_mapに存在しないなら無視
                    if !(hash_map.contains_key(&i[1])) {continue}
                    // S(=hash_map)からxを決まった個数を削除
                    *hash_map.get_mut(&i[1]).unwrap() -= min(i[2], hash_map[&i[1]]);
                },
            3 => {  while hash_map[& ( (-*(heap_min.peek().unwrap())) as usize)] == 0 {
                        heap_min.pop();
                    }
                    while hash_map[& ( (*(heap_max.peek().unwrap())) as usize)] == 0 {
                        heap_max.pop();
                    }
                    println!("{}", *(heap_max.peek().unwrap()) + *(heap_min.peek().unwrap()));
                },
            _ => (),
        }
        // println!("s: {:?}", s);
        // println!("{:?}", i);
    }
}