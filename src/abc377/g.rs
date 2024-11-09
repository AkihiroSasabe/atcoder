#![allow(dead_code, unused_imports)]
use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::BTreeMap;
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::collections::{HashSet, BTreeSet};
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
use rand::Rng;

fn trie_example() {
    use trie::Trie;
    let mut trie = Trie::new();

    // 登場する文字列
    let firefox = "firefox".chars().collect::<Vec<char>>(); // &vec!['f', 'i', 'r', 'e', 'f', 'o', 'x'] を検索しているのと同じ。
    let firework = "firework".chars().collect::<Vec<char>>();
    let fire = "fire".chars().collect::<Vec<char>>();
    let firewalk = "firewalk".chars().collect::<Vec<char>>();
    let fix = "fix".chars().collect::<Vec<char>>();
    let rust = "rust".chars().collect::<Vec<char>>();
    let python = "python".chars().collect::<Vec<char>>();

    // 挿入: O(M Klog(K)) (Mは単語の長さ, Kは登場する文字の種類。アルファベットならK=26)
    trie.insert(&firefox, 0);
    trie.insert(&firework, 1);
    trie.insert(&python, 2);
    trie.insert(&fix, 3);
    trie.insert(&firewalk, 4);

    println!("Trie: {:?}", trie);

    // デバッグ
    trie.print_all_nodes();
    trie.print_tree();

    // 検索 (完全一致 / prefixの一致): O(MlogK) (Mは単語の長さ, Kは登場する文字の種類。アルファベットならK=26)
    println!("is_exact_match for 'firefox': {}", trie.is_exact_match(&firefox));
    println!("is_exact_match for 'firework': {}", trie.is_exact_match(&firework));
    println!("is_exact_match for 'fire': {}", trie.is_exact_match(&fire));
    println!("is_start_with for 'fire': {}", trie.is_start_with(&fire));
    println!("is_exact_match for 'rust': {}", trie.is_exact_match(&rust));
    return;

}

fn main() {
    // 2024-11-01 18:39-19:34 (55min, Trieなことは気付いたが、RustでTrieをどう実装していいか分からず。)
    // 2024-11-04 13:54-14:30 (36min)
    // 89min

    // trie_example();
    // return;

    input! {
        n: usize,
        s: [Chars; n]
    }
    use trie::Trie;
    let mut trie = Trie::new();
    for i in 0..n {
        // 途中まで一致
        let ans = trie.search_abc377g(&s[i]);

        println!("{}", ans);
        trie.insert_abc378g(&s[i], i);
    }
    // trie.print_tree();
}

// 3
// snuke
// snuki
// snuuk
// 5
// 2
// 4

// snuke
// snuki

// snuke
// snuuk
// 4

// 例題
// ABC377-G https://atcoder.jp/contests/abc377/tasks/abc377_g
// https://algo-logic.info/trie-tree/
// 挿入と検索
// 挿入(insert)：文字列をトライ木に保存する O(M Klog(K))
// 検索(search)：文字列がトライ木に保存されているか確認する O(MlogK)
// (Mは単語の長さ。Kは登場する文字の種類。アルファベットならK=26)
// 使い方
// use trie::Trie;
// let mut trie = Trie::new();
// let firework = "firework".chars().collect::<Vec<char>>();
// trie.insert(&firework, 1);
// println!("is_exact_match for 'firework': {}", trie.is_exact_match(&firework));
// trie.print_tree();

// 参考実装: https://algo-logic.info/trie-tree/
// ただし、 node の next を children というメンバー名に変えている。
// next はVec だったのに対し、 children は、BTreeMap にしている。
// 計算量：insert, search ともに O(M)（Mは単語の長さ）
// 正確には、insert:O(M Klog(K)), search: O(MlogK) (Kは登場する文字の種類。アルファベットならK=26)
mod trie {
    use std::collections::BTreeMap;
    use std::cmp::{max, min};
    use std::hash::Hash;


    #[derive(Debug)]
    pub struct Node<T> {
        // 子ノードを格納するBTreeMap。キーは文字、値は子ノードのインデックス
        children: BTreeMap<T, usize>,
        // このノードで終わる文字列のインデックスを格納
        ends: Vec<usize>, // 参考URLでは、accept という属性名が採用されていたけど、 ends の方が分かりやすい。 
        // このノードが共有されている文字列の数
        common: usize,
        // ノードに持たせたい値。new()関数で、初期値を入れるのを忘れないように。 abc377-g では:  残りの文字数の最小長さ (min_left_len)。
        value: usize
    }

    impl<T> Node<T> 
        where T: Eq + Clone + Copy + std::fmt::Debug + std::fmt::Display + std::cmp::Ord
    {
        fn new() -> Self {
            Node {
                children: BTreeMap::new(),
                ends: Vec::new(),
                common: 0,
                value: 1 << 60 // 適当な初期値をいれると良い
            }
        }
    }

    #[derive(Debug)]
    pub struct Trie<T> {
        pub nodes: Vec<Node<T>>,
    }

    impl<T> Trie<T> 
        where T: Eq + Clone + Copy + std::fmt::Debug + std::fmt::Display + std::cmp::Ord
    {
        pub fn new() -> Self {
            // 初期ノードとしてrootを追加
            Trie {
                nodes: vec![Node::new()],
            }
        }

        pub fn insert(&mut self, word: &Vec<T>, word_id: usize) {
            // 文字列をトライ木に保存する関数
            let mut node_index = 0;
            for i in 0..word.len() {
                let ch = word[i];
                let new_node_index = self.nodes.len(); // もし、新規の子ノードを作成する場合のインデックス
                self.nodes[node_index].common += 1;

                // 子ノードが存在しなければ新しいノードを作成
                if !self.nodes[node_index].children.contains_key(&ch) {
                    self.nodes[node_index].children.insert(ch, new_node_index);
                    self.nodes.push(Node::new()); // 子を持たないノードを追加
                    node_index = new_node_index;
                }
                else {
                    node_index = *self.nodes[node_index].children.get(&ch).unwrap();
                }
            }
            // 最後のノードに文字列のIDを追加
            self.nodes[node_index].ends.push(word_id);
            self.nodes[node_index].common += 1;
        }

        pub fn search(&self, word: &Vec<T>, is_prefix_check: bool) -> bool {
            // 文字列がトライ木に保存されているか確認する関数
            // 
            // [1]検索対象の単語を1文字ずつ確認していき、文字が無ければ、挿入されていないとみなす。
            // [2]検索対象の単語の最後の文字の頂点にたどり着いたら、その頂点で終了する文字列があるかを確認する。
            // 例えば、"firework" だけをトライ木に挿入済みで、"fire"という単語を検索した場合は、'e'で終わる単語が挿入されていないので、Falseを返す。
            let mut node_index = 0;
            for i in 0..word.len() {
                let ch = word[i];
                if let Some(&next_index) = self.nodes[node_index].children.get(&ch) {
                    node_index = next_index;
                }
                else {
                    return false
                }
            }
            if is_prefix_check {
                // prefix が、一致しているか確認したいだけなら、ここで完了
                return true
            }
            else {
                // 最後の頂点が受理状態か確認 (終了する単語があるか確認)
                return !self.nodes[node_index].ends.is_empty()    
            }
        }
        // 検索対象の単語と、完全に一致している単語が挿入されているかの確認
        pub fn is_exact_match(&self, word: &Vec<T>) -> bool {
            let is_prefix_check = false;
            return self.search(word, is_prefix_check)
        }
        // prefix を持つ単語が存在するかの確認
        pub fn is_start_with(&self, prefix: &Vec<T>) -> bool {
            let is_prefix_check = true;
            return self.search(prefix, is_prefix_check)
        }
        // 単語の番号を自動で採番して挿入する。
        pub fn insert_without_word_id(&mut self, word: &Vec<T>) {
            // id の指定無しで挿入
            self.insert(word, self.nodes[0].common);
        }
        pub fn get_num_inserted_words(&self) -> usize {
            // 挿入した単語の数 を返す
            return self.nodes[0].common
        }
        pub fn len(&self) -> usize {
            // Trie木のノード数 を返す関数
            return self.nodes.len()
        }

        pub fn print_all_nodes(&self) {
            for i in 0..self.len() {
                println!("trie.nodes[{i}] = {:?}", self.nodes[i]);
            }
        }
        
        pub fn print_tree(&self) {
            println!("---- Trie tree starts----");
            self.dfs_for_print(0, 0);
            println!("");
            println!("---- Trie tree ends ----")
        }

        fn dfs_for_print(&self, v: usize, depth: usize) {
            // v := ノード
            // depth := そのノードの深さ
            for (i, (&ch, &next_v)) in self.nodes[v].children.iter().enumerate() {
                if i != 0 {
                    println!("");
                    for _ in 0..depth*2 {
                        print!(" ");
                    }
                }
                print!("{} ", ch);
                self.dfs_for_print(next_v, depth + 1);
            }
        }

        // abc378-G 用にアレンジしたinsert
        pub fn insert_abc378g(&mut self, word: &Vec<T>, word_id: usize) {
            // 文字列をトライ木に保存する関数
            let mut node_index = 0;
            for i in 0..word.len() {
                let ch = word[i];
                let new_node_index = self.nodes.len();
                self.nodes[node_index].common += 1;

                // abc377-g用
                self.nodes[node_index].value = min(self.nodes[node_index].value, word.len()-i);
                
                // 子ノードが存在しなければ新しいノードを作成
                if !self.nodes[node_index].children.contains_key(&ch) {
                    self.nodes[node_index].children.insert(ch, new_node_index);
                    self.nodes.push(Node::new()); // 子を持たないノードを追加
                    node_index = new_node_index;
                }
                else {
                    node_index = *self.nodes[node_index].children.get(&ch).unwrap();
                }
            }
            // 最後のノードに文字列のIDを追加
            self.nodes[node_index].ends.push(word_id);
            self.nodes[node_index].common += 1;
            // abc377-g用
            self.nodes[node_index].value = min(self.nodes[node_index].value, 0);
        }
        // abc378-G 用にアレンジしたsearch
        pub fn search_abc377g(&self, word: &Vec<T>) -> usize {
            let mut node_index = 0;
            let mut ans = word.len(); // 空文字列と一致させるのに必要なコスト
            for i in 0..word.len() {
                let ch = word[i];
                if let Some(&next_index) = self.nodes[node_index].children.get(&ch) {
                    node_index = next_index;
                    ans = min(ans, self.nodes[node_index].value + word.len() - i - 1);
                }
                else {
                    return ans
                }
            }
            // 最後の頂点が受理状態か確認 (終了する単語があるか確認)
            return ans
        }
    }
}