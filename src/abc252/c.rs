use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};

fn main() {
    input! {
        n: usize,
    }
    // 各リールの各値(0-9)が、それぞれ最短何秒待つと出現するかを格納
    let mut s: Vec<Vec<usize>> = vec![vec![0;10]; n];
    for i in 0..n {
        input! {
            s_i: Chars
        }

        let s_i: Vec<usize> = s_i.iter().map(|x| (*x as usize - 48)).collect();
        
        let mut j = 0;
        for val in s_i {
            s[i][val] = j;
            j += 1;
        }
    }
    // println!("{:?}", s);

    let INF: usize = 1 << 60;

    // 0-9の値で揃える場合の最短時刻をそれぞれ格納
    let mut answers = vec![0; 10];

    // 0-9の値で揃える場合の最短時刻を全探索
    for val in 0..10 {
        // n個の各リールを値valで揃えるときに掛かる最短時間を、リール毎に格納
        let mut time_list = vec![];
        for i in 0..n {
            time_list.push(s[i][val]);
        }
        // 各リールを値valで揃えるときに掛かる時間の頻度を探索(最小0秒-最大9秒)
        let mut time_hist: Vec<usize> = vec![0;10];
        for j in 0..n {
            time_hist[time_list[j]] += 1;
        }
        // 同一時刻にvalが揃ってしまうリールに関しては1周分(=10秒)待ってもらう
        for t in 0..10 {
            if time_hist[t] >= 1 {
                answers[val] = max(answers[val], (time_hist[t]-1) * 10 + t);
            }
        }
    }
    let answer = answers.iter().min().unwrap();
    println!("{}", answer);

}