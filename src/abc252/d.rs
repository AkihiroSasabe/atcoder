use std::collections::HashMap;

use proconio::input;
fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    // 解法
    // iをスライドさせる。i以降にあるa[i]以外の数字を2個選ぶ
    // 2個のうち、重複してしまうものを消す

    // 値ごとに頻度を連想配列で記録
    let mut hash_map = HashMap::new();
    for i in 0..n {
        if !hash_map.contains_key(&a[i]) {
            hash_map.insert(a[i], 1);
        }
        else {
            *hash_map.get_mut(&a[i]).unwrap() += 1;
        }
    }

    // 値ごとにnC2を計算する(同じ値を重複して選ぶ組み合わせ数を数える)
    let a_max = 2 * 100_000;
    let mut tyohuku = vec![0; a_max+1];
    let mut tyohuku_sum = 0;
    for (key, val) in &hash_map {
        let tyohuku_per_key = *val * (*val-1) / 2; // =nC2
        tyohuku[*key] = tyohuku_per_key;
        tyohuku_sum += tyohuku_per_key;
    }

    let mut ans = 0;
    for i in 0..n-2 {
        // i番目より後ろにいて、選択可能な項の数
        let nokori = n - i - hash_map[&a[i]];
        let nokori_pattern = nokori * (nokori - 1) / 2; // =nC2
        ans += nokori_pattern - (tyohuku_sum - tyohuku[a[i]]);

        // 各データの更新
        *hash_map.get_mut(&a[i]).unwrap() -= 1;
        tyohuku_sum -= tyohuku[a[i]];
        if hash_map[&a[i]] > 0 {
            tyohuku[a[i]] = hash_map[&a[i]] * (hash_map[&a[i]] - 1) / 2;
            tyohuku_sum += tyohuku[a[i]];
        }
    }
    println!("{}", ans);
}