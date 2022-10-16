use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut x: [usize; q]
    }
    // 記載番号とindexを対応させる(1始まりではなく、0始まりにする)
    for i in 0..x.len() {
        x[i] -= 1;
    }

    // 数列b: [key: 番号, value: 位置]
    let mut b = vec![];
    for i in 0..n {
        b.push(i);
    }

    // 数列a: [key: 位置, value: 番号]
    let mut a = b.clone();

    for i in 0..x.len() {
        let position = b[x[i]];
        // クエリの位置は既に右端か?
        if position != n - 1 {
            let neighbor = a[position + 1];
            // 右隣とswap
            a.swap(position, position + 1); 
            b.swap(neighbor, x[i]);
        }
        else {
            // 左隣とswap
            let neighbor = a[position - 1];
            a.swap(position, position - 1);
            b.swap(neighbor, x[i]);
        }
    }

    for i in 0..n {
        print!("{} ", a[i] + 1);
    }

}