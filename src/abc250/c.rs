use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        q: usize,
        x: [usize; q]
    }

    // i番目の要素に、i番目の位置にあるボールに "書かれている整数" が格納されている
    let mut writen: Vec<usize> = (0..n).collect();

    // i番目の要素に、整数iが書かれたボールの "位置" と、"整数i"が格納されている
    let mut positions = vec![vec![0;2];n];
    for i in 0..n {
        positions[i] = vec![i,i];
    }

    for i in x {
        if positions[i-1][0] != n-1 {
            // 今いる場所
            let left_pos = positions[i-1][0].clone();
            let right_val = writen[left_pos+1].clone();
            let right_pos = positions[right_val][0].clone();
            let left_val = i-1;

            positions[right_val][0] = left_pos;
            positions[left_val][0] = right_pos;
            writen[left_pos] = right_val;
            writen[right_pos] = i-1;
        }
        else {
            // 今いる場所
            // println!("owa {}", i);
            let right_pos = positions[i-1][0].clone();
            // println!("right_pos {}", right_pos);
            let right_val = writen[i-1].clone();
            let left_pos = right_pos - 1;
            let left_val = writen[left_pos].clone();

            positions[right_val][0] = left_pos;
            positions[left_val][0] = right_pos;
            writen[left_pos] = right_val;
            writen[right_pos] = i-1;
        }
        encode(&positions);
    }

    // println!("positions: {:?}", positions);
    positions.sort();
    // println!("positions: {:?}", positions);

    for i in positions {
        print!("{} ", i[1] + 1);
    }
}

// 
fn encode(list: &Vec<Vec<usize>>) {
    let mut list2 = list.clone();
    list2.sort();
    // println!("{:?}", list2);
}