// 公式の解説通りにやる
// 1 <= p, q, r <= N
// p = q * r
// c(p) * c(q) * c(r)
// q: 1~Mの整数
// r: 1/q, 2/q, 3/q, ..., M/q

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }
    // println!("a {:?}", a);

    let mut a2 = a.clone();

    // 大きい順にソート
    a2.sort_by(|x, y| y.cmp(x));
    // println!("a2 {:?}", a2);
    let a_max = a2[0];

    // 各要素が何個あるか?
    let mut counter = vec![0; a_max+1];
    for i in a.iter() {
        counter[*i] += 1;
    }
    // for (i, x) in counter.iter().enumerate() {
    //     println!("i: {}, x: {}", i, x);
    // }

    let mut answer = 0;
    for q in 1..a_max+1 {
        // println!("q {}", q);
        for r in 1..(a_max / q + 1) {
            // println!("r {}", r);
            let p = q * r;
            // println!("p, q, r: {} {} {}", p, q, r);
            // println!("counter[p], counter[q], counter[r]: {} {} {}", counter[p], counter[q], counter[r]);
            answer += counter[p] * counter[q] * counter[r];
        }
    }
    println!("{}", answer);
}