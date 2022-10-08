// 2022-04-29 9:34 
// usize: 18_446_744_073_709_551_615
//                       998_244_353

use proconio::input;
fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize
    }

    // n, m, k
    // n個の数列を使って、総和がKであるような数列の総数
    // dp[n, k]
    // 初期化
    let mut dp: Vec<Vec<usize>> = vec![vec![0;k+1]; n+1];
    for j in 1..k+1 {
        if j <= m {
            dp[1][j] = 1;
        }
    }
    // 初期化時のdpの確認
    // for i in dp.iter() {
    //     println!("{:?}", i);
    // }

    for i in 0..n {
        for j in 0..k+1 {
            for x in 1..m+1 {
                if (k>=j+x) {
                    // 配る型
                    // println!("i, j, x: {} {} {}",i, j, x);
                    // println!("dp[i+1][j+x], dp[i][j]: {} {}", dp[i+1][j+x], dp[i][j]);
                    dp[i+1][j+x] += dp[i][j] % 998244353;
                }
            }
        }
    }
    
    // 緩和済みのdpの確認
    // println!("relaxed dp");
    // for i in dp.iter() {
    //     println!("{:?}", i);
    // }

    let mut answer = 0;
    for j in 0..k+1 {
        answer += dp[n][j];
    }
    println!("{}", answer % 998244353);
}