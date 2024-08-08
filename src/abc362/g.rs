#![allow(dead_code, unused_imports)]
// use proconio::input;
// use itertools::Itertools;
// use core::num;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::BTreeMap;
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::collections::{HashSet, BTreeSet};
use std::time;
// use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
// use superslice::*;
fn main() {
    solve_abc362_g();
    // solve_aoj_alds1_14_d();

    // suffix array の生成例
    // let mut s: Vec<char> = "mmiissiissiippii".chars().collect();
    // let suffix_array = sa_is::get_suffix_array(&s);
    // println!("suffix_array = {:?}", suffix_array);

    // let mut s: Vec<char> = "aaacccbbb".chars().collect();
    // let suffix_array = sa_is::get_suffix_array(&s);
    // println!("suffix_array = {:?}", suffix_array);

    // println!("{}", '~' as usize); // 126
    // println!("{}", '*' as usize); // 42
    // '*': 42
    // 'A': 65, 'Z':90
    // 'a': 97, 'z':122
    // '~': 126
    // for i in 0..256 {
    //     println!("i = {i}, {}", i as u8 as char);
    // }
    // return;
}

fn solve_aoj_alds1_14_d() {
    // https://onlinejudge.u-aizu.ac.jp/problems/ALDS1_14_D
    // pi が T に存在するかを答える問題。
    // 存在するなら1を、しないなら0をprint
    let mut t = read_vec_char();
    let q = read_usize();
    let mut p = vec![];
    for i in 0..q {
        let pi = read_vec_char();
        p.push(pi);
    }

    let suffix_array = sa_is::get_suffix_array(&t);
    t.push(0 as char);

    // 2分探索
    for i in 0..q {
        let mut pi = p[i].clone();

        // pi <= T[suffix_array[ok]..] となる最小の ok を探す
        let ok = binary_search(&t, &pi, &suffix_array);
        
        // pi + '~' <= T[suffix_array[ok2]..] となる最小の ok2 を探す
        pi.push('~');
        let ok2 = binary_search(&t, &pi, &suffix_array);

        if ok2 - ok > 0 {
            println!("1");
        }
        else {
            println!("0");
        }
    }

}

fn solve_abc362_g() {
    // https://atcoder.jp/contests/abc362/tasks/abc362_g
    // 2024-07-14 11:02-
    // 2024-08-05 20:40-21:00 (20 min)
    // 2024-08-07 18:58-20:20 (1h22min)
    // Total 1h42min

    // AOJ では prconio が使えないので。
    // input! {
    //     mut s: Chars,
    //     q: usize,
    //     t: [Chars; q]
    // }
    let mut s = read_vec_char();
    let q = read_usize();
    let mut t = vec![];
    for i in 0..q {
        let ti = read_vec_char();
        t.push(ti);
    }

    // Sの部分文字列 で、Tiと一致するものが、何個?
    // 26文字
    let n = s.len();

    let suffix_array = sa_is::get_suffix_array(&s);
    s.push(0 as char);

    // Debug
    // println!("suffix_array = {:?}", suffix_array);
    // println!("s = {:?}", s);
    // for i in 0..n+1 {
    //     let ind = suffix_array[i];
    //     println!("i = {i}, &s[{ind}..] = {:?}", &s[ind..]);
    // }

    // Tiと一致する個数
    // 辞書順で、Ti以上なものを探せばok
    for i in 0..q {

        // t[i] <= s[sa[ok]..] となる最小の ok を見つける
        let ok = binary_search(&s, &t[i], &suffix_array);

        // t[i]+'~' <= s[sa[ok2]..] となる最小の ok2 を見つける
        let mut ti = t[i].clone();
        ti.push('~'); // '~' は、どんなアルファベットよりも、辞書順が遅い (126番目)
        let ok2 = binary_search(&s, &ti, &suffix_array);

        // t[i]より辞書的に大きく、t[i]'~'より辞書的に小さいものの総数
        // 例えば、t[i] == "aa" とすると、 aabはカウント。 ac はカウントから除外。
        // aa <= aab かつ、 aab < aa~
        // aa <= ac だが、 aa~ < ac
        let ans: usize = ok2 - ok; 
        println!("{}", ans);
    }
}

fn binary_search(s: &Vec<char>, t: &Vec<char>, suffix_array: &Vec<usize>) -> usize {
    // めぐる式二分探索 で t[..] <= s[suffix_array[ok]..] となる最小の ok を見つける
    let n = s.len() - 1;
    if t[..] > s[suffix_array[n]..] {return n + 1} // 辞書的に一番最後のsuffix が、 t より辞書順で早いときは、早期リターン
    let mut ng = 0; // suffix array の index。s[suffix_array[ng]..] < t[i]
    let mut ok = n; // suffix array の index。s[suffix_array[ok]..] >= t[i]
    while (ng as i128 - ok as i128).abs() > 1 {
        let mid = (ng + ok) / 2;
        let ind = suffix_array[mid];
        let is_ok = t[..] <= s[ind..];
        if is_ok {
            ok = mid;
        }
        else {
            ng = mid;
        }
    }
    return ok
}

// インタラクティブな読み込みをする関数 (文字列をVec<char>で受け取り)
// 入力: abcd -> 出力: vec!['a', 'b', 'c', 'd'] みたいな。
// 入力: a b c d -> 出力: vec!['a', 'b', 'c', 'd'] とはならないので注意
fn read_vec_char() -> Vec<char> {
    let s: String = read_string();
    // String -> Vec<char> に変換
    let cs: Vec<char> = s.chars().collect();
    return cs
}

// 文字列のインタラクティブな読み込みをする関数 (1行に1変数)
fn read_string() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    // trim()は末尾の"\n"を消して、&str型で返す。(trimしないとString型のまま)
    return s.trim().to_string()
    // s.trim().parse().unwrap()
}

// インタラクティブな読み込みをする関数 (1行に1個のusize)
fn read_usize() -> usize {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim().parse().unwrap()
}

/// suffix array を O(N) で求めるアルゴリズム
/// suffix array とは、文字列S の (|S| + 1) 個　の suffix を 開始位置の index で表現し、辞書順にソートしたものである。
/// |S| + 1 の 1 は、文字列の最後尾につける空文字('$' や '*')で、辞書順で1番先頭にくる(0 as char でもよい。 SA構築後に、s.push(0 as char) しておくと安心。)。
/// 例えば、S = "acb" ならsuffixは、"acb$", "cb$", "b$", "$"となり、suffix_array = [3, 0, 2, 1] となる。
/// ちなみに '~' は、どのアルファベットよりも辞書的に遅いので、二分探索する際に便利。
/// 
/// 理解と実装の参考: まめめも氏が、Ruby実装だが一番分かりやすい。 https://mametter.hatenablog.com/entry/20180130/p1 
/// flare氏の3部作は、Rust実装だが、パワポの途中で数式の意味がよく分からなくなった。 (1) https://qiita.com/flare/items/20439a1db54b367eea70 (2) https://qiita.com/flare/items/ac11972dbc590a91980d (3) https://speakerdeck.com/flare/sa-is
/// 東工大生の解説は、まめめも氏のを参考にして作っているが、 tokodai というSA-ISの理解には不適当な例を使っているため分かりにくい。 https://trap.jp/post/953/
pub mod sa_is {
    /// 使い方
    // let mut s: Vec<char> = "mmiissiissiippii".chars().collect();
    // let suffix_array = sa_is::get_suffix_array(&s);
    // println!("suffix_array = {:?}", suffix_array);
    // suffix_array = [16, 15, 14, 10, 6, 2, 11, 7, 3, 1, 0, 13, 12, 9, 5, 8, 4]

    // let mut s: Vec<char> = "aaacccbbb".chars().collect();
    // let suffix_array = sa_is::get_suffix_array(&s);
    // println!("suffix_array = {:?}", suffix_array);
    // suffix_array = [9, 0, 1, 2, 8, 7, 6, 5, 4, 3]

    pub fn get_suffix_array(s: &Vec<char>) -> Vec<usize> {
        let k = 256; // 文字種の数 (char型は8bitなので、256種類しかありえない)
        let mut ss = vec![];
        for &si in s {
            ss.push(si as usize);
        }
        ss.push(0); // $ の代わり。$は、他のアルファベットと比べると辞書順に早い。
        let suffix_array = sa_is(&ss, k);
        return suffix_array
    
        // 著者の t は、
        // # L 型か S 型か　を格納したリスト
        // t = [nil] * s.size
    
        // 筆者の lmss は、 LMSの index だけ取り出したリスト (「種」)
        // lmss = (0...s.size).select {|i| lms?(t, i) }
        // lms_list の方が分かりやすいかと。
    
        // 題材の文字列を "mmiissiissiippii"
        // 文字列の suffix (i, suffix)で、iはsuffixの開始位置
        // 0 mmiissiissiippii$
        // 1 miissiissiippii$
        // 2 iissiissiippii$
        // 3 issiissiippii$
        // 4 ssiissiippii$
        // 5 siissiippii$
        // 6 iissiippii$
        // 7 issiippii$
        // 8 ssiippii$
        // 9 siippii$
        // 10 iippii$
        // 11 ippii$
        // 12 ppii$
        // 13 pii$
        // 14 ii$
        // 15 i$
        // 16 $
    
        // suffix をソート
        // 16 $
        // 15 i$
        // 14 ii$
        // 10 iippii$
        // 6 iissiippii$
        // 2 iissiissiippii$
        // 11 ippii$
        // 7 issiippii$
        // 3 issiissiippii$
        // 1 miissiissiippii$
        // 0 mmiissiissiippii$
        // 13 pii$
        // 12 ppii$
        // 9 siippii$
        // 5 siissiippii$
        // 8 ssiippii$
        // 4 ssiissiippii$
    
        // suffix array 
        // [16, 15, 14, 10, 6, 2, 11, 7, 3, 1, 0, 13, 12, 9, 5, 8, 4]
    
        //
        // 0 $ S : --
        // 1 i L : --
        // 2 i L : --
        // 3 i S : --
        // 4 i S : --
        // 5 i S : --
        // 6 i S : --
        // 7 i S : --
        // 8 i S : --
        // 9 m L : --
        // 10 m L : --
        // 11 p L : --
        // 12 p L : --
        // 13 s L : --
        // 14 s L : --
        // 15 s L : --
        // 16 s L : --
        
        // Bin
        // 文字'i'のBinは1-8, 
        // 文字'm'のBinは9-10, 
        // 文字'm'のBinは11-12, 
        // 文字's'のBinは13-16, 
    
        // LMS型
        // インデックス i - 1 が L 型、i が S 型になっているとき
        // i を LMS（Left-most S, 最も左の S）と言う。
        // LMS から次の LMS までの部分文字列を LMS 部分文字列 と言う。
        // mmiissiissiippii$
        // LLSSLLSSLLSSLLLLS
        //   ^   ^   ^     ^ <= LMS のインデックス
        //   iissi           <= LMS 部分文字列
        //       iissi       <= LMS 部分文字列
        //           iippii$ <= LMS 部分文字列
        //                 $ <= LMS 部分文字列
    
        // 文字 ch で始まる suffix は、sa の bin[ch] 番目から bin[ch+1] - 1 番目のどこかに入れればよい
        // L 型の suffix と S 型の suffix が同じビンに入る場合（つまり先頭の文字が同じ場合）、必ず L 型の方を前に入れる
    
        // Induced Sort
        // 1. LMS のインデックスをビンの終わりから詰めていく
        // 2. L 型のインデックスをビンの頭から詰めていく
        // 3. S 型のインデックスを（LMS を含めて再度）ビンの終わりから詰めていく
        
        // Step1. LMS のインデックスをビンの終わりから詰めていく
        // LMS をどういう順序で挿入するか? => この時点で正しい順序はわからないので、適当でok
        // 2 、6 、10 と並んでいるが、正しくは上から 10 、6 、2
        //  0 $ S : 16 $
        //  1 i L : --
        //  2 i L : --
        //  3 i S : --
        //  4 i S : --
        //  5 i S : --
        //  6 i S :  2 iissiissiippii$
        //  7 i S :  6 iissiippii$
        //  8 i S : 10 iippii$
        //  9 m L : --
        // 10 m L : --
        // 11 p L : --
        // 12 p L : --
        // 13 p L : --
        // 14 p L : --
        // 15 s L : --
        // 16 s L : --
    
        // Step2. L 型のインデックスをビンの頭から詰めていく
        // 「挿入位置」は必ず「今見ているところ」より後に来ます *2
        // *2:インデックス i に対して i-1 が L 型のときだけ挿入する。L 型の定義から s[i-1..] > s[i..] 。つまり s[i-1..] の挿入位置は s[i..] より後になる。
        // 0 $ S : 16 $    <===== 今見ているところ
        // 1 i L : 15 i$   <===== 挿入位置
        // 2 i L : --
        // 3 i S : --
        // 4 i S : --
        // 5 i S : --
        // 6 i S :  2 iissiissiippii$
        // 7 i S :  6 iissiippii$
        // 8 i S : 10 iippii$
        // 9 m L : --
        // 10 m L : --
        // 11 p L : --
        // 12 p L : --
        // 13 p L : --
        // 14 p L : --
        // 15 s L : --
        // 16 s L : -_  
    
        // 次は、入れたばかりの 15 です。14 も L 型なので入れます。これを繰り返していくと、最終的に次のようになります。
        //  0 $ S : 16 $
        //  1 i L : 15 i$
        //  2 i L : 14 ii$
        //  3 i S : --
        //  4 i S : --
        //  5 i S : --
        //  6 i S :  2 iissiissiippii$
        //  7 i S :  6 iissiippii$
        //  8 i S : 10 iippii$
        //  9 m L :  1 miissiissiippii$
        // 10 m L :  0 mmiissiissiippii$
        // 11 p L : 13 pii$
        // 12 p L : 12 ppii$
        // 13 s L :  5 siissiippii$
        // 14 s L :  9 siippii$
        // 15 s L :  4 ssiissiippii$
        // 16 s L :  8 ssiippii$
    }
    
    
    fn sa_is(s: &Vec<usize>, k: usize) -> Vec<usize> {
        // 2024-07-27 ~16:24
        // 参考: https://mametter.hatenablog.com/entry/20180130/p1#fn-1aedbc3e

        // println!("s = {:?}", s);
        // println!("s.len() = {}===========", s.len());
    
        // 筆者の t に対応。 ls_array[i] := s[i] が L型なら'L'、S型なら'S'が格納される
        let mut ls_array = get_ls_array(&s);
    
        // l_s_lms_array[i] := s[i] が L型なら'L'、S型なら'S'、LMS型なら'M'が格納される
        let l_s_lms_array = get_l_s_lms_array(ls_array);
    
        // suffix_array の各要素のうち、頭文字が同じ範囲を格納した bin を求める
        let bin: Vec<(usize, usize)> = get_bin(&s, k);
    
        // LMS のインデックスだけを集めた配列
        let mut lms_list = vec![];
        for i in 0..s.len() {
            if l_s_lms_array[i] == 'M' {
                lms_list.push(i);
            }
        }
    
        // // 適当な「種」: seed は lms_list を適当にシャッフルして並べ替えてあっても良い。
        // let seed = lms_list;
    
        // 1 回目の induced sort
        let suffix_array = induced_sort(&s, k, &l_s_lms_array, &bin, &lms_list);
        
        // 再帰的に、 induced sort を実行する。
        // 元の文字列の長さに比べて半分未満になってるので、O(n + n/2 + n/4 + ...) = O(2n) = O(n) ということで、線形時間が保たれます
    
        // ====================================
    
        // induced sort の結果から LMS 型の suffix だけ取り出す
        let mut sa = vec![];
        for i in 0..suffix_array.len() {
            if l_s_lms_array[suffix_array[i]] == 'M' {
                sa.push(suffix_array[i]);
            }
        }
    
        // LMS のインデックス i に対して番号 nums[i] を振る
        let mut nums = vec![s.len(); s.len()];
    
        // sa[0] の LMS は $ と決まっているので、番号 0 を振っておく
        let mut num = 0;
        nums[sa[0]]= num;
    
        // 隣り合う LMS を比べる
        for i in 0..sa.len()-1 {
            let mut is_diff = false;
    
            let v = sa[i];
            let w = sa[i+1];
    
            // 隣り合う LMS 部分文字列の比較
            for ti in 0..s.len() {
                if (s[v + ti] != s[w + ti]) || ((l_s_lms_array[v + ti] == 'M') != (l_s_lms_array[w + ti] == 'M')) {
                    // LMS 部分文字列の範囲で異なる文字があった
                    is_diff = true;
                    break
                }
                else if ti > 0 && ((l_s_lms_array[v + ti] == 'M') || (l_s_lms_array[w + ti] == 'M')) {
                    // LMS 部分文字列の終端に至った
                    break
                }
            }
    
            // 隣り合う LMS 部分文字列が異なる場合は、番号を増やす
            if is_diff {
                num += 1;
            }
            // LMS のインデックス j に番号 num を振る
            nums[w] = num;
        }
    
        // nums の中に出現する番号のみを並べると、LMS 部分文字列を番号に置き換えた列が得られる
        let nums_temp = nums;
        let mut nums = vec![];
        for x in nums_temp {
            if x == s.len() {continue}
            nums.push(x);
        }
    
        // ================================
    
        if num + 1 < nums.len() {
            // 番号の重複があるので再帰
            sa = sa_is(&nums, num + 1);
        }
        else {
            // 番号の重複がない場合、suffix array を容易に求められる
            sa = vec![nums.len(); nums.len()];
            for (ind, &num) in nums.iter().enumerate() {
                sa[num] = ind;
            }       
        }
        
        // 正しい 「種」
        // let mut seed = sa.into_iter().map(|i| lms_list[i]).collect();
    
        let mut lms_list_temp = lms_list;
        let mut lms_list = vec![];
        for x in sa {
            lms_list.push(lms_list_temp[x]);
        }
    
        // 2回目のinduced sort
        let sa = induced_sort(&s, k, &l_s_lms_array, &bin, &lms_list);
    
        return sa
    }
    
    fn induced_sort(s: &Vec<usize>, k: usize, l_s_lms_array: &Vec<char>, bin: &Vec<(usize, usize)>, lms_list: &Vec<usize>) -> Vec<usize> {
        // suffix_array を初期化
        let mut suffix_array = vec![s.len(); s.len()];
        // println!("suffix_array = {:?}", suffix_array);
    
        // Step1: LMS型のsuffixを、各binの末尾に順番を無視して挿入
        // println!("---- ---- ---- Step 1 LMS type ---- ---- ----");
        insert_lms_type(&mut suffix_array, &s, k, &l_s_lms_array, &bin, &lms_list);
        // println!("suffix_array = {:?}", suffix_array);
    
        // Step2: L型のsuffixを挿入
        // println!("---- ---- ---- Step 2 L type ---- ---- ----");
        insert_l_type(&mut suffix_array, &s, k, &l_s_lms_array, &bin);
        // println!("suffix_array = {:?}", suffix_array);
    
        // Step3: S型のsuffixを挿入
        // println!("---- ---- ---- Step 3 S type ---- ---- ----");
        insert_s_type(&mut suffix_array, &s, k, &l_s_lms_array, &bin);
        // println!("suffix_array = {:?}", suffix_array);
    
        return suffix_array
    }
    
    fn insert_s_type(suffix_array: &mut Vec<usize>, s: &Vec<usize>, k: usize, l_s_lms_array: &Vec<char>, bin: &Vec<(usize, usize)>) {
    // fn insert_s_type(suffix_array: &mut Vec<usize>, s: &Vec<char>, k: usize, l_s_lms_array: &Vec<char>, bin: &Vec<(usize, usize)>) {
        // Step3. suffix array を逆順に捜査して S 型の suffix を埋めていきます。
        // LMS も S 型の一種なので埋めなおします。
        // ステップ 2 と同じように、「挿入位置」は必ず「今見ているところ」より前に来ます。理由も同様です。
        // 最初から入っていた LMS は（"$" を除いて）いったん全部書きつぶされ、その後で再挿入されます。*
    
        // ビン毎に、すでに挿入したS型の数をカウントする
        let mut num_inserted_s_type = vec![0; k];
    
        // sa.reverse_each do |i|
        // next if i == nil
        // next if i == 0
        // next if t[i - 1] == :L
    
        // # sa に入っているインデックス i について、i - 1 が S 型であるとき、
        // # 文字 s[i - 1] に対応するビンに i - 1 を終わりから詰めていれる
        // ch = s[i - 1]
        // sa[bin[ch + 1] - 1 - count[ch]] = i - 1 # 上書きすることもある
        // count[ch] += 1
        // end
    
        // suffix_array を逆順に走査して S 型 とLM-S型の suffix を埋めていく
        for i in (0..suffix_array.len()).rev() {
            if suffix_array[i] == suffix_array.len() {continue}     // 初期値だった場合は、スキップ (著者解説コードのnilに対応。)
            if suffix_array[i] == 0 {continue}                      // suffix_array[i] - 1 が負になってしまうので、スキップ (1個前がL型のものだけ挿入するので、挿入漏れの心配不要)
            if l_s_lms_array[suffix_array[i]-1] == 'L' {continue}   // 1個前が、Lだったときは、スキップ  (今回はS型だけを挿入したいので。)
            
            // 挿入するsuffixの、頭文字のインデックス
            let ch = s[suffix_array[i] - 1];
            
            // suffix_array の挿入位置 
            let insert_index = bin[ch].1 - 1 - num_inserted_s_type[ch];
                //  bin[x] := (文字xのbinの先頭のindex, 末尾のindex+1) となるbinを返す
    
            // 挿入
            suffix_array[insert_index] = suffix_array[i] - 1;
            num_inserted_s_type[ch] += 1;
            // println!("---- i = {:?} ----", i);
            // println!("insert_index = {:?}", insert_index);
            // println!("suffix_array = {:?}", suffix_array);
        }
    
    //     0 $ S : 16 $
    //     1 i L : 15 i$
    //     2 i L : 14 ii$
    //     3 i S : --
    //     4 i S : --
    //     5 i S : --
    //     6 i S :  2 iissiissiippii$
    //     7 i S :  6 iissiippii$
    //     8 i S :  7 issiippii$   <===== 挿入位置
    //     9 m L :  1 miissiissiippii$
    //    10 m L :  0 mmiissiissiippii$
    //    11 p L : 13 pii$
    //    12 p L : 12 ppii$
    //    13 s L :  5 siissiippii$
    //    14 s L :  9 siippii$
    //    15 s L :  4 ssiissiippii$
    //    16 s L :  8 ssiippii$    <===== 今見ているところ
    
    
        // 0 $ S : 16 $
        // 1 i L : 15 i$
        // 2 i L : 14 ii$
        // 3 i S : 10 iippii$
        // 4 i S :  2 iissiissiippii$
        // 5 i S :  6 iissiippii$
        // 6 i S : 11 ippii$
        // 7 i S :  3 issiissiippii$
        // 8 i S :  7 issiippii$
        // 9 m L :  1 miissiissiippii$
        // 10 m L :  0 mmiissiissiippii$
        // 11 p L : 13 pii$
        // 12 p L : 12 ppii$
        // 13 s L :  5 siissiippii$
        // 14 s L :  9 siippii$
        // 15 s L :  4 ssiissiippii$
        // 16 s L :  8 ssiippii$
    
    
    
    }
    
    fn insert_l_type(suffix_array: &mut Vec<usize>, s: &Vec<usize>, k: usize, l_s_lms_array: &Vec<char>, bin: &Vec<(usize, usize)>) {
    // fn insert_l_type(suffix_array: &mut Vec<usize>, s: &Vec<char>, k: usize, l_s_lms_array: &Vec<char>, bin: &Vec<(usize, usize)>) {
        // Step2. L 型のインデックスをビンの頭から詰めていく
        // L型: i文字目が(i+1)文字目よりも辞書順で後 (Larger)
    
        // ビン毎に、すでに挿入したL型の数をカウントする
        let mut num_inserted_l_type = vec![0; k];
    
        // suffix_array を正順に走査して L 型の suffix を埋めていく
        //  suffix_array に入っているインデックス suffix_array[i] について、suffix_array[i] - 1 が L 型であるとき、
        //  文字 s[suffix_array[i] - 1] に対応するビンに suffix_array[i] - 1 を頭から詰めていれる
        //      「挿入位置」は必ず「今見ているところi」より後に来ます *2
        //          *2:インデックス suffix_array[i] に対して suffix_array[i]-1 が L 型のときだけ挿入する。
        //          L 型の定義から s[suffix_array[i]-1..] > s[suffix_array[i]..] 。つまり s[suffix_array[i]-1..] の挿入位置は s[suffix_array[i]..] より後になる。
        for i in 0..suffix_array.len() {
            if suffix_array[i] == suffix_array.len() {continue}     // 初期値だった場合は、スキップ (著者解説コードのnilに対応。)
            if suffix_array[i] == 0 {continue}                      // suffix_array[i] - 1 が負になってしまうので、スキップ (1個前がL型のものだけ挿入するので、挿入漏れの心配不要)
            if l_s_lms_array[suffix_array[i]-1] != 'L' {continue}   // 1個前が、Sだったときは、スキップ (今回はL型だけを挿入したいので。)
            
            // 挿入する suffix の、頭文字のインデックス
            let ch = s[suffix_array[i] - 1];
            
            // suffix_array の挿入位置 
            let insert_index = bin[ch].0 + num_inserted_l_type[ch];
                //  bin[x] := (文字xのbinの先頭のindex, 末尾のindex+1) となるbinを返す
    
            // 挿入
            suffix_array[insert_index] = suffix_array[i] - 1;
            num_inserted_l_type[ch] += 1;
            // println!("---- i = {:?} ----", i);
            // println!("insert_index = {:?}", insert_index);
            // println!("suffix_array = {:?}", suffix_array);
        }
    
        // 著者の t は、
        // # L 型か S 型か　を格納したリスト
        // t = [nil] * s.size
    
    
        // 0 $ S : 16 $    <===== 今見ているところ
        // 1 i L : 15 i$   <===== 挿入位置
        // 2 i L : --
        // 3 i S : --
        // 4 i S : --
        // 5 i S : --
        // 6 i S :  2 iissiissiippii$
        // 7 i S :  6 iissiippii$
        // 8 i S : 10 iippii$
        // 9 m L : --
        // 10 m L : --
        // 11 p L : --
        // 12 p L : --
        // 13 p L : --
        // 14 p L : --
        // 15 s L : --
        // 16 s L : -_  
    
        // 次は、入れたばかりの 15 です。14 も L 型なので入れます。これを繰り返していくと、最終的に次のようになります。
        //  0 $ S : 16 $
        //  1 i L : 15 i$
        //  2 i L : 14 ii$
        //  3 i S : --
        //  4 i S : --
        //  5 i S : --
        //  6 i S :  2 iissiissiippii$
        //  7 i S :  6 iissiippii$
        //  8 i S : 10 iippii$
        //  9 m L :  1 miissiissiippii$
        // 10 m L :  0 mmiissiissiippii$
        // 11 p L : 13 pii$
        // 12 p L : 12 ppii$
        // 13 s L :  5 siissiippii$
        // 14 s L :  9 siippii$
        // 15 s L :  4 ssiissiippii$
        // 16 s L :  8 ssiippii$
    
    }
    
    fn insert_lms_type(suffix_array: &mut Vec<usize>, s: &Vec<usize>, k: usize, l_s_lms_array: &Vec<char>, bin: &Vec<(usize, usize)>, lms_list: &Vec<usize>) {
        // Step1. LMS のインデックスをビンの終わりから詰めていく
        // LMS (Left-most S) をどういう順序で挿入するか? => この時点で正しい順序はわからないので、適当でok
        // 2 、6 、10 と並んでいるが、正しくは上から 10 、6 、2
        //  0 $ S : 16 $
        //  1 i L : --
        //  2 i L : --
        //  3 i S : --
        //  4 i S : --
        //  5 i S : --
        //  6 i S :  2 iissiissiippii$
        //  7 i S :  6 iissiippii$
        //  8 i S : 10 iippii$
        //  9 m L : --
        // 10 m L : --
        // 11 p L : --
        // 12 p L : --
        // 13 p L : --
        // 14 p L : --
        // 15 s L : --
        // 16 s L : --
    
        // ビンごとにすでに挿入した数をカウントする
        let mut num_inserted_lms = vec![0; k];
        for &i in lms_list.iter().rev() {
            // 文字のindex
            let ch = s[i];
            // let bin_s = bin[ch].0;
            let bin_t = bin[ch].1;
            suffix_array[bin_t-1-num_inserted_lms[ch]] = i;
            num_inserted_lms[ch] += 1;
    
        }
    
        // for i in (0..s.len()).rev() {
        //     if l_s_lms_array[i] == 'M' {
        //         // 文字のindex
        //         let ch = s[i] as usize;
        //         // let bin_s = bin[ch].0;
        //         let bin_t = bin[ch].1;
        //         suffix_array[bin_t-1-num_inserted_lms[ch]] = i;
        //         num_inserted_lms[ch] += 1;
        //     }
        // }
    }
    
    
    fn get_l_s_lms_array(mut ls_array: Vec<char>) -> Vec<char> {
        // L型とS型からなる配列を、L型とS型とLMS型となる配列に変換する
        
        // LMS型
        // インデックス i - 1 が L 型、i が S 型になっているとき
        // i を LMS（Left-most S, 最も左の S）と言う。
        // LMS から次の LMS までの部分文字列を LMS 部分文字列 と言う。
        // mmiissiissiippii$
        // LLSSLLSSLLSSLLLLS
        //   ^   ^   ^     ^ <= LMS のインデックス
        //   iissi           <= LMS 部分文字列
        //       iissi       <= LMS 部分文字列
        //           iippii$ <= LMS 部分文字列
        //                 $ <= LMS 部分文字列
    
        for i in (1..ls_array.len()).rev() {
            if ls_array[i-1] == 'L' && ls_array[i] == 'S' {
                ls_array[i] = 'M'; // (LMSをMとする)
            }
        }
        return ls_array
    }
    
    fn get_bin(s: &Vec<usize>, k: usize) -> Vec<(usize, usize)> {
        //  bin[x] := (文字xのbinの先頭のindex, 末尾のindex+1) となるbinを返す
    
        // let mut s: Vec<char> = "aaacccbbb".chars().collect();
    
        // 初期化
        let mut counter = vec![0; k];
    
        // s に出現する文字種ごとのカウント
        for i in 0..s.len() {
            // println!("{} as usize = {:?}", s[i], s[i] as usize);
            // 78891 as usize = 78891
            counter[s[i]] += 1;
        }
    
        // 文字種を累積することで bin のインデックスを決める
        for i in 1..k {
            counter[i] = counter[i] + counter[i-1];
        }
    
        // bin[i-1]..bin[i] までが、文字iのbi
        let mut bin: Vec<(usize, usize)> = vec![];
        bin.push((0, counter[0]));
    
        for i in 1..k {
            bin.push((counter[i-1], counter[i]));
        }
        return bin
    }
    
    fn get_ls_array(s: &Vec<usize>) -> Vec<char> {
        /// suffix s[i..] が、L型かS型か決める
        /// 参考: https://mametter.hatenablog.com/entry/20180130/p1#f-7d7c0936
    
        // &str → Vec<char>
        // 題材の文字列
        // // let mut s: Vec<char> = "mmiissiissiippii".chars().collect();
        // let mut s: Vec<char> = "aaacccbbb".chars().collect();
        // s.push('$');
    
        // let a: Vec<char> = (0..s.len()).map(|n| (n as u8 + '0' as u8) as char).collect(); // 数字 -> char に変換 (例: 5 -> '5' )
        // let a: Vec<char> = (0..10).map(|n| (std::char::from_digit(n as u32, 10).unwrap()) as char).collect(); // 数字 -> char に変換 (例: 5 -> '5' )
        // let k = 256; // 文字種の数
    
        // # L 型か S 型か
        let mut ls_array: Vec<char> = vec!['a'; s.len()];
    
        // # 最後は S型
        ls_array[s.len()-1] = 'S';
    
        for i in (0..s.len()-1).rev() {
            // # s[i] < s[i+1] なら明らかに s[i..] < s[i+1..] => i は S 型
            // # s[i] > s[i+1] なら明らかに s[i..] > s[i+1..] => i は L 型
            // # s[i] == s[i+1] の場合、s[i..] <=> s[i+1..] の比較結果は
            // # s[i+1..] <=> s[i+2..] に準ずる (つまり ls_array[i + 1] と同じ)
            ls_array[i] = match s[i].cmp(&s[i + 1]) {
                std::cmp::Ordering::Less => 'S',        // iがi+1よりSmaller
                std::cmp::Ordering::Greater => 'L',     // iがi+1よりLarger
                std::cmp::Ordering::Equal => ls_array[i + 1]   // iがi+1と等しい
            };
    
            // s[9..] = $       s[9..] = S (定義。こうすると都合が良い)
            // s[8..] = b$      s[8..] > s[7..] = L
            // s[7..] = bb$     s[7..] > s[8..] <=> L
            // s[6..] = bbb$    s[6..] > s[7..] <=> L
            // s[5..] = cbbb$   s[5..] > s[6..] <=> 
            // s[4..] = ccbbb$  s[4..] > s[5..]
        }
        // println!("       a = {:?}", a);
        // println!("       s = {:?}", s);
        // println!("ls_array = {:?}", ls_array);
    
        return ls_array
    
    }
}