// 公式の解説通りにやる
// 1 <= p, q, r <= N
// p = q * r
// c(p) * c(q) * c(r)
// q: 1~Mの整数
// r: 1/q, 2/q, 3/q, ..., M/q

use proconio::input;

// 2022/12/26 22:12~22:17
fn main() {
    input! {
        n: usize,
        mut a: [usize; n]
    }
    // a[i] == a[j] * a[k];


    // println!("{:?}", a);
    let a_max = 100_000;
    let mut hash = vec![vec![]; a_max+1];
    // let mut hash = vec![0; a_max+1];

    for i in 0..n {
        hash[a[i]].push(i);
        // hash[a[i]] = 1;
    }

    let mut ans = 0;
    for x in 1..a_max+1 {
        for y in 1..(a_max/x+1) {
            let z = x * y;
            if hash[x].len() > 0 && hash[y].len() > 0 && hash[z].len() > 0 {
                ans += (hash[x].len() * hash[y].len() * hash[z].len());
            }
        }
    }




    // let mut ans = 0;
    // // 2分探索で探す。無理だ。N^2かかる (j,k)のペアを決めるだけで無理
    // // 1がいると話がややこしくなる
    // for i in 0..n {
    //     for j in 0..n {
    //         if i == j {continue;}
    //         let product = a[i]*a[j];
    //         if product < 100_001 {
    //             ans += hash[product].len();
    //         }
    //     }
    // }
    println!("{}", ans);


    // // println!("a {:?}", a);

    // let mut a2 = a.clone();

    // // 大きい順にソート
    // a2.sort_by(|x, y| y.cmp(x));
    // // println!("a2 {:?}", a2);
    // let a_max = a2[0];

    // // 各要素が何個あるか?
    // let mut counter = vec![0; a_max+1];
    // for i in a.iter() {
    //     counter[*i] += 1;
    // }
    // // for (i, x) in counter.iter().enumerate() {
    // //     println!("i: {}, x: {}", i, x);
    // // }

    // let mut answer = 0;
    // for q in 1..a_max+1 {
    //     // println!("q {}", q);
    //     for r in 1..(a_max / q + 1) {
    //         // println!("r {}", r);
    //         let p = q * r;
    //         // println!("p, q, r: {} {} {}", p, q, r);
    //         // println!("counter[p], counter[q], counter[r]: {} {} {}", counter[p], counter[q], counter[r]);
    //         answer += counter[p] * counter[q] * counter[r];
    //     }
    // }
    // println!("{}", answer);
}