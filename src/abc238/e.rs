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
use std::collections::HashSet;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    // 2023-12-14 19:25-
    // 2024-03-05 19:44-20:50 (1h6m)
    input! {
        n: usize,
        q: usize,
        mut lr_raw: [(usize, usize); q]
    }

    // S[N]  が知りたい。(S[N] := Aの1からNまでの累積和)
    // A[L,R] = S[R] - S[L-1] がわかる。
    // S[N]= S[N] - S[0] が最終的に分かれば良さそう(S[0] := 0)

    // 例えば、S[6] が知りたいとする。Q=4で以下
    // A[1,3] = S[3] - S[0]
    // A[2,3] = S[3] - S[1]
    // A[2,5] = S[5] - S[1]
    // A[6,6] = S[6] - S[5]
    // が与えられているとする。

    // A[1,3] - A[2,3] 
    // = (S[3] - S[0]) - (S[3] - S[1])
    // S[1] - S[0] ----(1)
    // が得られる。

    // また、A[6,6] - A[2,5] で、
    // S[6] - S[1] ----(2)
    // が得られる。

    // (2) + (1)より、
    // S[6] - S[0]
    // が得られる。

    // よって、始点と終点のいずれかが揃っていれば、ツギハギができて、
    // 最終的に0-Nまで繋げられば良さそうなことがわかる。
    let mut uft =   UnionFindTree::new(n+1);
    for i in 0..q {
        let l = lr_raw[i].0 - 1;
        let r = lr_raw[i].1;
        uft.unite(l, r);
    }
    if uft.issame(0, n) {
        println!("Yes");
    }
    else {
        println!("No");
    }




    // 遅延評価セグメント木で、沼って解ける問題ではない。
    // let mut lr = vec![];
    // for i in 0..q {
    //     lr_raw[i].0 -= 1;
    //     lr_raw[i].1 -= 1;
    //     lr.push(vec![lr_raw[i].0, lr_raw[i].1]);
    // }
    // lr.sort();
    // println!("lr = {:?}", lr);

    // // たぶん、掃き出し法っぽい計算をしようとしている。
    // https://note.com/syamashi/n/ndcef5086f0df で掃き出し法で解いている人がいた。
    // let mut lazy_segment_tree = lazy_segment_tree::new_range_increment_update_and_range_sum_query(n);
    // for i in 0..q {
    //     lazy_segment_tree.range_update(lr[i][0], lr[i][1], 1);
    // }
    // // lazy_segment_tree.print_tree();

    // // まず、せ

    // // 特定の区間を消し去ればいい。
    // for i in 0..n {
    //     // 区間和を求めている?
    //     let data= lazy_segment_tree.range_query(i, i);
    //     println!("data = {:?}", data);
    //     let num = data.value;
    //     if num == 1 {
    //         continue
    //     }
    //     else if num == 0 {
    //         println!("No");
    //         return;
    //     }
    //     else {
    //         let ind = lr.lower_bound(&vec![num as usize, 0]);
    //         let left = lr[ind][0];
    //         let right = lr[ind][1];
    //         lazy_segment_tree.range_update(left, right, - num + 1);
    //     }
    // }

    // // lazy_segment_tree.print_list();
    // lazy_segment_tree.print_tree();


}


// Union-Find
// グループの管理（グループ同士の結合や、要素同士の所属か同じか判定）するのに便利なデータ構造
struct UnionFindTree {
    parents: Vec<usize>,    // 各頂点の属するグループ(根付き木)の親頂点の番号
    sizes: Vec<usize>       // 各頂点の属するグループ(根付き木)のサイズ(頂点数)
}

impl UnionFindTree {
    // 初期化
    fn new(n: usize) -> Self {
        // 各頂点が属するグループの根を格納
        let parents = (0..n).collect();
        // 各頂点が属するグループのサイズ(頂点数)を格納。※ただし、更新するのは根のインデックスだけで良い
        let sizes = vec![1; n];
        return UnionFindTree {parents, sizes}
    }

    // 根を求める。経路圧縮により計算量を削減
    fn root(&mut self, v: usize) -> usize {
        if self.parents[v] == v {return v}
        else {
            // 経路圧縮 (親を根に張り替える。)
            self.parents[v] = self.root(self.parents[v]);
            return self.parents[v] as usize
        }
    }

    // 同じグループに属するか
    fn issame(&mut self, v0: usize, v1: usize) -> bool {
        return self.root(v0) == self.root(v1)
    }

    // 頂点vが属する根付き木のサイズを取得
    fn size(&mut self, v: usize) -> usize {
        let root = self.root(v);
        return self.sizes[root]
    }

    // v0を含むグループと、v1を含むグループとを併合する。Union by sizeで計算量削減。
    fn unite(&mut self, mut v0: usize, mut v1: usize) -> bool {
        // 既に同じグループであれば何もしない
        v0 = self.root(v0);
        v1 = self.root(v1);
        if v0 == v1 {
            return false
        } 
        let child: usize;
        let parent: usize;
        // Union by sizeにより、サイズが小さいグループを、サイズが大きいグループに併合する
        if self.size(v0) <= self.size(v1) {
            child = v0;
            parent = v1;
        }
        else {
            child = v1;
            parent = v0;
        }
        self.sizes[parent] += self.size(child);
        self.parents[child] = parent;
        return true
    }
}



// 抽象化した遅延評価セグメント木を実装する
// 参考実装 (ACLのC++のコードと、kenkooさんのRust化されたコード)
// https://github.com/atcoder/ac-library/blob/master/atcoder/lazysegtree.hpp
// https://github.com/kenkoooo/competitive-programming-rs/blob/master/src/data_structure/lazy_segment_tree.rs
pub mod lazy_segment_tree {
    #[derive(Debug, Clone)]
    pub struct LazySegmentTree<S, Op, E, F, Mapping, Composition, Id> {
        list_size: usize,           // 探索対象の配列の大きさ
        tree_size: usize,           // セグメント木の頂点の総数
        leaf_size: usize,           // セグメント木の葉の総数
        tree: Vec<S>,               // セグメント木
        lazy_tree: Vec<F>,          // 遅延評価用の木
        op: Op,                     // 区間取得演算
        e: E,                       // 区間取得演算の単位元
        mapping: Mapping,           // 区間更新演算 (lazy -> data への伝播に対応)
        composition: Composition,   // 合成関数f(g(x)) (親のlazy -> 子のlazy　への伝播に対応)
        id: Id                      // 区間操作演算mappingにおける恒等写像 (遅延配列の初期値に対応)
    }
    //     // Range Increment Update
    //     // Range Assignment Update
    // Add と Replace の方がわかりやすいかも

    //     // Range Minimum Query
    //     // Range Maximum Query
    //     // Range Sum Query

    pub fn new_range_increment_update_and_range_maximum_query(list_size: usize) -> LazySegmentTree<isize, fn(&isize, &isize) -> isize, fn() -> isize, isize, fn(&isize, &isize) -> isize, fn(&isize, &isize) -> isize, fn() -> isize> {
        fn e() -> isize {
            let init_value: isize = 0;
            return init_value
        }
        fn op(&s0: &isize, &s1:  &isize) -> isize {
            s0.max(s1)
        }
        fn mapping(&f: &isize ,&x: &isize) -> isize {
            f + x
        }
        fn composition(&f: &isize ,&g: &isize) -> isize {
            f + g
        }
        fn id() -> isize {
            let init_value: isize = 0;
            return init_value
        }
        
        // 関数のかわりにクロージャーを使うこともできる。
        // let init_value: isize = 0;
        // let e = || init_value; // 返り値の型は、型推論によって省略可能
        // let op = |&s0: &isize, &s1: &isize| s0.max(s1);
        // let mapping = |&f: &isize ,&x: &isize| f + x;
        // let composition = |&f: &isize ,&g: &isize| f + g;
        // let id = || init_value;

        // ここで関数をpointerにcastしないと、"expected fn pointer, found fn item" というエラーメッセージが起きる。
        // fn item: 関数の実態
        // fn pointer: 関数の参照
        // Rustでは、関数の型アノテーションをする際に、fn item が使えず、 fn pointer として明示する必要がある。 cf. https://doc.rust-lang.org/beta/reference/types/function-item.html
        // (この関数の返り値の型の Op や E も、当然 fn pointer が使われている。)
        // また、キャストは不可逆なので注意 cf. https://stackoverflow.com/questions/71974428/dealing-with-expected-fn-pointer-found-fn-item
        // キャスト可能: fn item (fn ... {name})-> fn pointer 
        // キャスト不可：fn pointer -> fn item (fn ... {name})
        // キャストの例：
        // 可能な例：signer_func as fn(&[u8; 20], &[u8; 32]) -> &'a [u8; 32]
        // 不可な例：&signer_func as &fn(&[u8; 20], &[u8; 32]) -> &'a [u8; 32]
        let lazy_segment_tree = LazySegmentTree::new(
            list_size,
            e as fn() -> isize, 
            op as fn(&isize, &isize) -> isize, 
            mapping as fn(&isize, &isize) -> isize, 
            composition as fn(&isize, &isize) -> isize, 
            id as fn() -> isize
        );
        return lazy_segment_tree
    }

    #[derive(Clone, Debug, Copy)]
    pub struct SS {
        pub value: isize,
        size: isize
    }
    // 区間加算・区間和
    pub fn new_range_increment_update_and_range_sum_query(list_size: usize) -> LazySegmentTree<SS, fn(&SS, &SS) -> SS, fn() -> SS, isize, fn(&isize, &SS) -> SS, fn(&isize, &isize) -> isize, fn() -> isize> {
        
        fn e() -> SS {
            let init_value: isize = 0;
            let init_size: isize = 1;
            return SS {value: init_value, size: init_size}
        }
        fn op(&s0: &SS, &s1:  &SS) -> SS {
            return SS {value: s0.value + s1.value, size: s0.size + s1.size}
        }
        fn mapping(&f: &isize ,&x: &SS) -> SS {
            return SS {value: x.value + x.size * f, size: x.size}
        }
        fn composition(&f: &isize ,&g: &isize) -> isize {
            f + g
        }
        fn id() -> isize {
            let init_value: isize = 0;
            return init_value
        }
        
        let lazy_segment_tree = LazySegmentTree::new(
            list_size,
            e as fn() -> SS, 
            op as fn(&SS, &SS) -> SS, 
            mapping as fn(&isize, &SS) -> SS, 
            composition as fn(&isize, &isize) -> isize, 
            id as fn() -> isize
        );
        return lazy_segment_tree
    }

    impl<S, Op, E, F, Mapping, Composition, Id> LazySegmentTree<S, Op, E, F, Mapping, Composition, Id>
    where
        S: Clone + std::fmt::Debug,                         // セグメント木のノードに格納されたデータ型
        Op: Fn(&S, &S) -> S,                                // 
        E: Fn() -> S,                                       // 区間取得演算の単位元
        F: Clone + std::cmp::PartialEq + std::fmt::Debug,   // 遅延評価用の木のノードに格納されたデータ型
        Mapping: Fn(&F, &S) -> S,                           // 
        Composition: Fn(&F, &F) -> F,                       // 
        Id: Fn() -> F                                       // 
    {
        pub fn new(
            list_size: usize,                               // 入力配列の大きさ
            e: E,
            op: Op,
            mapping: Mapping,
            composition: Composition,
            id: Id
        ) -> Self {
            
            // セグメント木の頂点の総数tree_sizeを求める。
            // まずはセグメント木の葉の数leaf_sizeを、
            // (leaf_size / 2 < list_size <= leaf_size)
            // を満たす2のべき乗数となるように計算
            let mut leaf_size = 1;
            while (leaf_size < list_size) {
                leaf_size *= 2;
            }
            
            // セグメント木の頂点数 = セグメント木の葉の数 * 2 - 1
            let tree_size: usize = leaf_size * 2 - 1;
            
            // 木の各ノードの初期値
            let tree: Vec<S> = vec![e(); tree_size];
            let lazy_tree: Vec<F> = vec![id(); tree_size];
    
            return LazySegmentTree {list_size, tree_size, leaf_size, tree, lazy_tree, op, e, mapping, composition, id}
        }
    
        // 遅延木のv番目のノード(要素)について、遅延評価を行う(セグメント木には無かったメソッド)
        fn eval(&mut self, v: usize, v_l: usize, v_r: usize) {
            // v: 遅延評価したいノードのインデックス
            // v_l: ノードvの守備範囲の左端 (閉区間)
            // v_r: ノードvの守備範囲の右端 (閉区間)
            // 自ノードの値配列に値を伝播させる
            // 子ノードの遅延配列に値を伝播させる
            // 自分のノードの遅延配列を空にする
    
            // 遅延評価で空でない場合、自ノード及び子ノードへの値の伝播が起こる
            // let NEGATIVE_INF = (1 << 60) * (-1);
            // let init_value = 0;
            // if (self.lazy_tree[v] != NEGATIVE_INF) {
            
            if (self.lazy_tree[v] != (self.id)()) {
                // self.tree[v] = self.lazy_tree[v];
                // self.tree[v] += self.lazy_tree[v];
                self.tree[v] = (self.mapping)(&(self.lazy_tree[v].clone()), &self.tree[v]);
                
                // 最下段ではない場合、子ノードへ伝播
                if v_r - v_l >= 1 {
                    self.lazy_tree[2*v+1] = (self.composition)(&(self.lazy_tree[v].clone()), &(self.lazy_tree[2*v+1]));
                    self.lazy_tree[2*v+2] = (self.composition)(&(self.lazy_tree[v].clone()), &(self.lazy_tree[2*v+2]));
                    // self.lazy_tree[2*v+1] += self.lazy_tree[v];
                    // self.lazy_tree[2*v+2] += self.lazy_tree[v];
                    // self.lazy_tree[2*v+1] += self.lazy_tree[v] / 2;
                    // self.lazy_tree[2*v+2] += self.lazy_tree[v] / 2;
                }
                // 伝播が終わったので、自ノードの遅延配列を空にする
                self.lazy_tree[v] = (self.id)();
            }
        }
    
        pub fn range_update(&mut self, update_l: usize, update_r: usize, f: F) {
            // update_l:    更新区間の左端
            // update_r:    更新区間の右端(閉区間)
            // f:      更新後の値
            println!("start to update between {}-{} to {:?}", update_l, update_r, f);
            self._range_update(update_l, update_r, f, 0, 0, self.leaf_size - 1);
        }
    
        // 探索対象の配列の区間[l,r]の要素を、値xに変更する (1つの要素ではなく、複数要素を含む区間の更新)
        // 根から下に下がっていく。(セグメント木の1つの要素の更新のときは下から根に向かって更新していた)
        fn _range_update(&mut self, update_l: usize, update_r: usize, f: F, v: usize, v_l: usize, v_r: usize) {
            // update_l:    探索区間の左端
            // update_r:    探索区間の右端(閉区間)
            // x:      更新後の値
            // v:      現在の頂点のインデックス
            // v_l:    現在の頂点の守備範囲の左端
            // v_r:    現在の頂点の守備範囲の右端(閉区間)
            // 外からは、self._range_update(update_l, update_r, x, 0, 0, self.leaf_size - 1)として呼ぶ。特にv_rは、self.list_sizeではないので注意
            
            // v番目の頂点の遅延評価
            self.eval(v, v_l, v_r);
    
            // (1)更新範囲が、その頂点が持つ守備範囲と、交差しない
            if v_r < update_l || update_r < v_l {
                // 何もしない
                return;
            }
            // (2)更新範囲が、その頂点が持つ守備範囲を、完全に含む:Query ⊃ Vertex
            else if update_l <= v_l && v_r <= update_r {
                // 遅延木に値を入れた後に評価
                // self.lazy_tree[v] = x;
                // self.lazy_tree[v] += x;
                self.lazy_tree[v] = (self.composition)(&f, &self.lazy_tree[v]);
                // self.lazy_tree[v] += (update_r + 1 - update_l) as isize * x;
                // ここで評価を入れないと、(3)で子の配列が更新前の状態で、max()を呼び出すことになる
                self.eval(v, v_l, v_r);
                return;
            }
            // (3)更新範囲が、その頂点が持つ守備範囲と、部分一致:Query ∩ Vertex ≠ ∅
            else {
                // 2つの子頂点の内、大きい方に更新
                self._range_update(update_l, update_r, f.clone(), 2 * v + 1, v_l, (v_l + v_r) / 2);
                self._range_update(update_l, update_r, f.clone(), 2 * v + 2, (v_l + v_r) / 2 + 1, v_r);
                // self.tree[v] = max(self.tree[2 * v + 1], self.tree[2 * v + 2]);
                self.tree[v] = (self.op)(&self.tree[2 * v + 1], &self.tree[2 * v + 2]);
                return;
            }
        }
    
        // クラスの外からクエリを行うときのメソッド
        pub fn range_query(&mut self, q_l: usize, q_r: usize) -> S {
            // println!("start to query between {}-{} !!", q_l, q_r);
            return self._range_query(q_l, q_r, 0, 0, self.leaf_size - 1);
        }
    
        // 閉区間[q_l, q_r]の最大値を求める。右端が開区間')'ではなく、閉区間']'にしているので注意
        fn _range_query(&mut self, q_l: usize, q_r: usize, v: usize, v_l: usize, v_r: usize) -> S {
            // q_l:    探索区間の左端
            // q_r:    探索区間の右端(閉区間)
            // v:      現在の頂点のインデックス
            // v_l:    現在の頂点の守備範囲の左端
            // v_r:    現在の頂点の守備範囲の右端(閉区間)
            // 外からは、self._range_query(q_l, q_r, 0, 0, self.leaf_size - 1)として呼ぶ。特にv_rは、self.list_sizeではないので注意
    
            // 遅延評価!
            self.eval(v, v_l, v_r);
    
            // (1)探索範囲が、その頂点が持つ守備範囲と、交差しない
            if v_r < q_l || q_r < v_l {
                // let NEGATIVE_INF = (1 << 60) * (-1);
                // return NEGATIVE_INF
                // let init_value = 0;
                // return init_value
                return (self.e)()
            }
            // (2)探索範囲が、その頂点が持つ守備範囲を、完全に含む
            else if q_l <= v_l && v_r <= q_r {
                return self.tree[v].clone()
            }
            // (3)探索範囲が、その頂点が持つ守備範囲と、部分一致
            else {
                // 2つの子頂点の内、小さい方を返す
                let child_0 = self._range_query(q_l, q_r, 2 * v + 1, v_l, (v_l + v_r) / 2);
                let child_1 = self._range_query(q_l, q_r, 2 * v + 2, (v_l + v_r) / 2 + 1, v_r);
                return (self.op)(&child_0, &child_1);
            }
        }
    
        // 配列を確認(デバッグ用)
        fn print_list(&self) {
            println!("Print Array: ");
            for i in 0..self.list_size {
                let tree_index = i + self.tree_size / 2;
                print!("{:?}, ", self.tree[tree_index]);
            }
            println!("");
        }
    
        // セグメント木を確認(デバッグ用)
        fn _print_tree(&self, kind: &str) {
            let mut next_depth_index = 1;
            for i in 0..self.tree_size {
                if i == next_depth_index {
                    println!("");
                    next_depth_index = next_depth_index * 2 + 1;
                }
                match kind {
                    "segment_tree" => print!("{:?} ", self.tree[i]),
                    "lazy_tree" => print!("{:?} ", self.lazy_tree[i]),
                    _ => ()
                }
            }
            println!("");
        }
    
        pub fn print_tree(&self) {
            println!("==== Print Segment Tree ====");
            self._print_tree("segment_tree");
            println!("==== Print Lazy Tree ====");
            self._print_tree("lazy_tree");
            println!("==== ==== ==== ==== ==== =====");
        }
    }    
}
