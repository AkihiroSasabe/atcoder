use proconio::input;
use std::collections::HashSet;
use std::cmp::{max, min};


fn main() {
    // 2023-12-06 17:39-19:00 (1h21min)
    // 2023-12-06 22:45-24:00 (1h15min)
    // 2023-12-07 13:01-15:28 (2h27min)
    // total: 5h3min
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
        q: usize,
        xy: [(usize, usize); q],
    }

    // しゃくとり法っぽい感じでいけそう。 (競プロフレンズの回答を参考)
    let mut hash_a = HashSet::new();
    let mut hash_b = HashSet::new();

    // これから追加するやつ
    let mut a_ind = 0;
    let mut b_ind = 0;

    let mut a_ind_pre = -1;
    let mut b_ind_pre = -1;

    let mut borders_start = vec![];
    borders_start.push(vec![a_ind_pre as isize, b_ind_pre as isize]);

    let mut borders_end = vec![];

    let mut is_same_pre = true;
    let mut num_same = 0;

    loop {
        if a_ind >= n || b_ind >= n {break}
        // 新しく追加
        if !hash_a.contains(&a[a_ind]) && hash_b.contains(&a[a_ind]) {
            // a[a_ind] がa にとって新登場で、かつ、bが持っていた場合、被り数は+1増える
            num_same += 1;
        }
        hash_a.insert(a[a_ind]);

        if hash_a.contains(&b[b_ind]) && !hash_b.contains(&b[b_ind]) {
            num_same += 1;
        }
        hash_b.insert(b[b_ind]);

        // println!("a_ind, b_ind = ({a_ind}, {b_ind}), num_same = {num_same}");
        // println!("hash_a = {:?}", hash_a);
        // println!("hash_b = {:?}", hash_b);

        // 今回は一致したか?
        let is_same_now = hash_a.len() == hash_b.len() && hash_a.len() == num_same;
        // println!("is_same_pre = {:?}", is_same_pre);
        // println!("is_same_now = {:?}", is_same_now);

        // 前回の状態がどうなっているか?
        if is_same_pre {
            // 今回が不一致だった場合、そこがボーダーになる
            if !is_same_now {
                // if !(a_ind_pre == n && b_ind_pre == n) {
                //     // 前回一致していて、今回不一致だった場合
                //     borders_end.push(vec![a_ind_pre, b_ind_pre]);
                // }
                borders_end.push(vec![a_ind_pre, b_ind_pre]);

            }
        }
        // 前回不一致で、今回一致していた場合
        else {
            if is_same_now {
                borders_start.push(vec![a_ind as isize, b_ind as isize]);
            }
        }

        a_ind_pre = a_ind as isize;
        b_ind_pre = b_ind as isize;
        // println!("borders_end = {:?}", borders_end);

        if is_same_now {
            // もし今回一致していたら、可能な限り一致させつづける。
            if a_ind + 1 < n && hash_a.contains(&a[a_ind + 1]) {
                // aを増やしても問題なければaを増やす
                a_ind += 1;
            }
            else {
                if b_ind + 1 < n && hash_b.contains(&b[b_ind + 1]) {
                    // bを増やしても問題なければbを増やす
                    b_ind += 1;
                }
                else {
                    // aもbも増やしても駄目なら、少ない方をとりあえず増やす。
                    if a_ind <= b_ind {
                        a_ind += 1;
                    }
                    else {
                        b_ind += 1;
                    }
                }
            }
        }
        else {
            // もし今回違うなら...
            // 取り敢えず少ない方を増やすのがいいだろう
            if hash_a.len() < hash_b.len() {
                a_ind += 1;
            }
            else if hash_a.len() > hash_b.len() {
                b_ind += 1;
            }
            else {
                // 数が等しければ、小さい方のindexを増やす
                if a_ind <= b_ind {
                    a_ind += 1;
                }
                else {
                    b_ind += 1;
                }
            }
        }
        is_same_pre = is_same_now;
    }
    // println!("borders_end = {:?}", borders_end);
    // println!("borders_start = {:?}", borders_start);

    // mergeは必ずstartから始まる
    let mut borders_merge = vec![];
    for i in 0..borders_start.len() {
        borders_merge.push(borders_start[i].clone());
    }

    for i in 0..borders_end.len() {
        borders_merge.push(borders_end[i].clone());
    }
    borders_merge.sort();
    // println!("borders_merge = {:?}", borders_merge);

    let mut ok_a = vec![vec![]; n];

    if borders_merge.len() != 0 {
        // start
        let start_a = borders_merge[0][0];
        let start_b = borders_merge[0][1];

        let end_a;
        let end_b;
        if 1 < borders_merge.len() {
            end_a = borders_merge[1][0];
            end_b = borders_merge[1][1];    
        }
        else {
            end_a = (n-1) as isize;
            end_b = (n-1) as isize;
        }

        if end_a != -1 {
            for a_ind in max(0, start_a)..max(0, end_a+1) {
                ok_a[a_ind as usize] = vec![max(start_b, 0) as usize, end_b as usize];
            }
        }
    }

    for i in 2..borders_merge.len() {
        if i % 2 == 0 {
            // start
            let start_a = borders_merge[i][0];
            let start_b = borders_merge[i][1];

            let end_a;
            let end_b;
            if i+1 < borders_merge.len() {
                end_a = borders_merge[i+1][0];
                end_b = borders_merge[i+1][1];    
            }
            else {
                end_a = (n-1) as isize;
                end_b = (n-1) as isize;
            }
            for a_ind in start_a..end_a+1 {
                ok_a[a_ind as usize] = vec![start_b as usize, end_b as usize];
            }
        }
    }
    // println!("ok_a = {:?}", ok_a);

    for i in 0..q {
        let x = xy[i].0 - 1;
        let y = xy[i].1 - 1;
        if ok_a[x].len() == 0 {
            println!("No");
        }
        else {
            if ok_a[x][0] <= y && y <= ok_a[x][1] {
                println!("Yes");
            }
            else {
                println!("No");
            }
        }
    }

}