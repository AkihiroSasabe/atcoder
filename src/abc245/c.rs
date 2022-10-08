use proconio::input;
fn main() {
    input! {
        n: usize,
        k: isize,
        a: [isize; n],
        b: [isize; n]
    }

    let mut ab_list = vec![vec![false, false]; n];
    ab_list[0] = vec![true, true];
    for i in 0..(n-1) {
        // println!("i: {}", i);
        let aa = (a[i+1] - a[i]).abs();
        let ab = (b[i+1] - a[i]).abs();
        let ba = (a[i+1] - b[i]).abs();
        let bb = (b[i+1] - b[i]).abs();
        // println!("aa, ab, ba, bb: {} {} {} {}", aa, ab, ba, bb);

        // xのi番目がaが可能なとき
        if ab_list[i][0] {
            if aa <= k {
                ab_list[i+1][0] = true
            }
            if ab <= k {
                ab_list[i+1][1] = true
            } 
        }

        // xのi番目がbが可能なとき
        if ab_list[i][1] {
            if ba <= k {
                ab_list[i+1][0] = true
            }
            if bb <= k {
                ab_list[i+1][1] = true
            } 
        }

        // xのi+1番目がa,bのどちらもどれないとき
        if !ab_list[i+1][0] && !ab_list[i+1][1] {
            println!("No");
            // println!("ab_list: {:?}", ab_list);
            return
        }
    }
    println!("Yes");
}