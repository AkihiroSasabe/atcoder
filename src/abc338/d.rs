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
fn main() {
    // 2024-01-27 21:52-22:40 (48min)
    //            23:40-23:12 (32min)
    // total 80min 自力で一応解けた。コンテストには間に合わなかった
    input! {
        n: usize,
        m: usize,
        mut x: [usize; m],
    }
    for i in 0..m {
        x[i] -= 1;
    }

    let mut dists = vec![vec![0, 0]; m-1];
    let mut diffs = vec![];
    // let mut bit = BinaryIndexedTree::new(n);


    // 区間加算更新で、区間和取得する遅延評価セグメント木
    // lazy_segment_tree := 配列 i には、橋i-i+1を切ったときに、ツアー全体を最短で通ったときと比べて、増加してしまう移動量が搭載されている。
    let init_vector = vec![0; n];
    let mut lazy_segment_tree = lazy_segment_tree::new_range_increment_update_and_range_sum_query_from_vec(&init_vector);

    for i in 0..m-1 {
        // 時計回りの距離
        let long0 = max(x[i], x[i+1]) - min(x[i], x[i+1]);
        // 反時計回りの距離
        let long1 = n - long0;
        dists[i][0] = min(long0, long1);
        dists[i][1] = max(long0, long1);

        let diff = dists[i][1] - dists[i][0];
        diffs.push(diff);

        let vmin = min(x[i], x[i+1]);
        let vmax = max(x[i], x[i+1]);
        if vmax - vmin == dists[i][0] {
            // 素直 (時計回り) が最短距離のとき
            // 1本の橋を切る場所を、C(i) := (i,i+1) と表現すれば、
            // C(vmin) ~ C(vmax-1) のいずれかを切られてしまうと、逆回りに遠回りしないといけなくなるので、
            // その分、diff の分増える。
            lazy_segment_tree.range_update(vmin, vmax-1, diff as isize);
        }
        else {
            // 逆走 (反時計回り) が最短距離のとき
            // C(vmax) ~ C(vmin) = C(vmax) ~ C(n-1) + C(0) ~ C(vmin) のいずれかを切られてしまうと、時計回りに遠回りしないといけなくなるので、
            // その分、diff の分増える。
            lazy_segment_tree.range_update(vmax, n-1, diff as isize);
            if vmin != 0 {
                lazy_segment_tree.range_update(0, vmin-1, diff as isize);
            }
        }
    }

    // lazy_segment_tree.print_tree();
    // ideal_length := どこも橋を切らないときの、最短のツアーの長さ
    let mut ideal_length = 0;
    for i in 0..m-1 {
        ideal_length += dists[i][0];
    }
    // println!("ideal_length = {:?}", ideal_length);

    // どこを切るか i, i+1 を切った場合
    let mut ans = usize::MAX;
    for i in 0..n {
        let range_sum_node = lazy_segment_tree.range_query(i, i);
        let range_sum = range_sum_node.value;
        // 橋 (i,i+1)を切ったとき、range_sum の分だけ遠回りしないといけない
        let cand = ideal_length + range_sum as usize;
        ans = min(ans, cand);
    }
    println!("{}", ans);
    
}




// 抽象化した遅延評価セグメント木を実装する
// 参考実装 (ACLのC++のコードと、kenkooooさんのRust化されたコード)
// https://github.com/atcoder/ac-library/blob/master/atcoder/lazysegtree.hpp
// https://github.com/kenkoooo/competitive-programming-rs/blob/master/src/data_structure/lazy_segment_tree.rs
pub mod lazy_segment_tree {
    #[derive(Debug, Clone)]
    pub struct LazySegmentTree<S, Op, E, F, Mapping, Composition, Id> {
        list_size: usize,           // 探索対象の配列の大きさ
        tree_size: usize,           // セグメント木の頂点の総数
        leaf_size: usize,           // セグメント木の葉の総数
        pub tree: Vec<S>,               // セグメント木
        lazy_tree: Vec<F>,          // 遅延評価用の木
        op: Op,                     // 区間取得演算
        e: E,                       // 区間取得演算の単位元
        mapping: Mapping,           // 区間更新演算 (lazy -> data への伝播に対応)
        composition: Composition,   // 合成関数f(g(x)) (親のlazy -> 子のlazy　への伝播に対応)
        id: Id                      // 区間操作演算mappingにおける恒等写像 (遅延配列の初期値に対応)
    }
        // Update (更新)
        // - Range Increment Update
        // - Range Assignment Update
        //   (Add と Replace の方がわかりやすいかも)

        // Query (取得)
        // - Range Minimum Query
        // - Range Maximum Query
        // - Range Sum Query

    // [1] 区間加算更新・区間最小値取得　の生成
    pub fn new_range_increment_update_and_range_minimum_query(list_size: usize) -> LazySegmentTree<isize, fn(&isize, &isize) -> isize, fn() -> isize, isize, fn(&isize, &isize) -> isize, fn(&isize, &isize) -> isize, fn() -> isize> {
        /// 区間取得演算 (opだけど、更新操作ではなく取得なので注意)
        /// s0: 左の子data (たぶん)
        /// s1: 右の子data (たぶん)
        /// 返り値: 2子から得られたdata
        fn op(&s0: &isize, &s1:  &isize) -> isize {
            s0.min(s1)
        }
        /// Lazy -> Data の更新操作
        /// f: Lazyによる操作
        /// x: 更新前のdata
        /// 返り値: 更新後のdata
        fn mapping(&f: &isize ,&x: &isize) -> isize {
            return f + x
        }
        /// Lazy -> Lazy の更新操作
        /// f: 追加する最新の更新操作
        /// g: これまでの更新操作
        /// 返り値: g -> f の順にする操作。gとfの合成関数: f(g(x))
        fn composition(&f: &isize ,&g: &isize) -> isize {
            return f + g
        }
        /// 区間取得演算opの単位元 (dataの初期化に使われる)
        /// op(a, e()) = op(e(), a) = a となるような、e()
        fn e() -> isize {
            let init_value: isize = std::isize::MAX;
            return init_value
        }
        /// 区間操作(更新)演算mappingにおける恒等写像 (lazyの初期化と、mappingとcompositionに使われる。区間代入更新のときは、正直なんでもいいと思う。)
        /// mapping(id(), a) = a となるようなid()
        fn id() -> isize {
            let init_value: isize = 0;
            return init_value
        }
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

    // [1] 区間加算更新・区間最小値取得 を vector から生成
    pub fn new_range_increment_update_and_range_minimum_query_from_vec(init_vector: &Vec<isize>) -> LazySegmentTree<isize, fn(&isize, &isize) -> isize, fn() -> isize, isize, fn(&isize, &isize) -> isize, fn(&isize, &isize) -> isize, fn() -> isize> {
        let n = init_vector.len();
        let mut lazy_segment_tree = new_range_increment_update_and_range_minimum_query(n);
        // 遅延評価セグメント木の data 配列の初期化 (treeのdata配列は、isize::MAXで初期化しているが、元の配列は本問題ではa[i]=0(i=0,1,...)なので、-isize::MAXで打ち消して0にしている点に注意)
        lazy_segment_tree.range_update(0, n -1, -std::isize::MAX);
        for i in 0..n {
            lazy_segment_tree.range_update(i, i, init_vector[i]);
        }
        return lazy_segment_tree
    }
    
    // [2] 区間加算更新・区間最大値取得　の生成
    pub fn new_range_increment_update_and_range_maximum_query(list_size: usize) -> LazySegmentTree<isize, fn(&isize, &isize) -> isize, fn() -> isize, isize, fn(&isize, &isize) -> isize, fn(&isize, &isize) -> isize, fn() -> isize> {
        fn op(&s0: &isize, &s1:  &isize) -> isize {
            s0.max(s1)
        }
        fn mapping(&f: &isize ,&x: &isize) -> isize {
            f + x
        }
        fn composition(&f: &isize ,&g: &isize) -> isize {
            f + g
        }
        fn e() -> isize {
            // let init_value: isize = 0; // <- 本当はminにするべき
            let init_value: isize = - std::isize::MAX;
            return init_value
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

    // [2] 区間加算更新・区間最大値取得 を vector から生成
    pub fn new_range_increment_update_and_range_maximum_query_from_vec(init_vector: &Vec<isize>) -> LazySegmentTree<isize, fn(&isize, &isize) -> isize, fn() -> isize, isize, fn(&isize, &isize) -> isize, fn(&isize, &isize) -> isize, fn() -> isize> {
        let n = init_vector.len();
        let mut lazy_segment_tree = new_range_increment_update_and_range_maximum_query(n);
        // 遅延評価セグメント木の data 配列の初期化 (treeのdata配列は、-isize::MAXで初期化しているが、+isize::MAXで打ち消して0にしている点に注意)
        lazy_segment_tree.range_update(0, n -1, std::isize::MAX); // 本当は素直に-MINしたいけど、オーバーフローするのでMAXを足す。
        // isize::MIN := -9_223_372_036_854_775_808isize
        // isize::MAX :=  9_223_372_036_854_775_807isize
        for i in 0..n {
            lazy_segment_tree.range_update(i, i, init_vector[i]);
        }
        return lazy_segment_tree
    }

    // [3] 区間加算更新・区間和取得　の生成
    pub fn new_range_increment_update_and_range_sum_query(list_size: usize) -> LazySegmentTree<SSum, fn(&SSum, &SSum) -> SSum, fn() -> SSum, isize, fn(&isize, &SSum) -> SSum, fn(&isize, &isize) -> isize, fn() -> isize> {
        
        fn op(&s0: &SSum, &s1:  &SSum) -> SSum {
            return SSum {value: s0.value + s1.value, size: s0.size + s1.size}
        }
        fn mapping(&f: &isize ,&x: &SSum) -> SSum {
            return SSum {value: x.value + x.size * f, size: x.size}
        }
        fn composition(&f: &isize ,&g: &isize) -> isize {
            f + g
        }
        fn e() -> SSum {
            let init_value: isize = 0;
            let init_size: isize = 0;
            return SSum {value: init_value, size: init_size}
        }
        fn id() -> isize {
            let init_value: isize = 0;
            return init_value
        }
        
        let mut lazy_segment_tree = LazySegmentTree::new(
            list_size,
            e as fn() -> SSum, 
            op as fn(&SSum, &SSum) -> SSum, 
            mapping as fn(&isize, &SSum) -> SSum, 
            composition as fn(&isize, &isize) -> isize, 
            id as fn() -> isize
        );

        // 各ノードの、守備範囲の大きさ(tree.size)を適切に初期化
        init_node_size_for_range_sum_tree(&mut lazy_segment_tree);

        return lazy_segment_tree
    }

    // [3] 区間加算更新・区間和取得 を vector から生成
    pub fn new_range_increment_update_and_range_sum_query_from_vec(init_vector: &Vec<isize>) -> LazySegmentTree<SSum, fn(&SSum, &SSum) -> SSum, fn() -> SSum, isize, fn(&isize, &SSum) -> SSum, fn(&isize, &isize) -> isize, fn() -> isize> {
        let n = init_vector.len();
        let mut lazy_segment_tree = new_range_increment_update_and_range_sum_query(n);
        for i in 0..n {
            lazy_segment_tree.range_update(i, i, init_vector[i]);
        }
        return lazy_segment_tree
    }

    // [4] 区間代入更新・区間最小値取得　の生成
    pub fn new_range_assignment_update_and_range_minimum_query(list_size: usize) -> LazySegmentTree<isize, fn(&isize, &isize) -> isize, fn() -> isize, isize, fn(&isize, &isize) -> isize, fn(&isize, &isize) -> isize, fn() -> isize> {
        /// 区間取得演算 (opだけど、更新操作ではなく取得なので注意)
        /// s0: 左の子data (たぶん)
        /// s1: 右の子data (たぶん)
        /// 返り値: 2子から得られたdata
        fn op(&s0: &isize, &s1:  &isize) -> isize {
            s0.min(s1)
        }
        /// Lazy -> Data の更新操作
        /// f: Lazyによる操作
        /// x: 更新前のdata
        /// 返り値: 更新後のdata
        fn mapping(&f: &isize ,&x: &isize) -> isize {
            if f == id() {
                return x
            }
            else {
                return f
            }
        }
        /// Lazy -> Lazy の更新操作
        /// f: 追加する最新の更新操作
        /// g: これまでの更新操作
        /// 返り値: g -> f の順にする操作。gとfの合成関数: f(g(x))
        fn composition(&f: &isize ,&g: &isize) -> isize {
            if f == id() {
                return g
            }
            else {
                return f
            }
        }
        /// 区間取得演算opの単位元 (dataの初期化に使われる)
        fn e() -> isize {
            let init_value: isize = std::isize::MAX;
            return init_value
        }
        /// 区間操作(更新)演算mappingにおける恒等写像 (lazyの初期化と、mappingとcompositionに使われる。区間代入更新のときは、正直なんでもいいと思う。)
        fn id() -> isize {
            let init_value: isize = std::isize::MAX; // 代入する値の範囲外に使われなければ、どんな値を入れてもいい。
            return init_value
        }
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

    // [4] 区間代入更新・区間最小値取得 を vector から生成
    pub fn new_range_assignment_update_and_range_minimum_query_from_vec(init_vector: &Vec<isize>) -> LazySegmentTree<isize, fn(&isize, &isize) -> isize, fn() -> isize, isize, fn(&isize, &isize) -> isize, fn(&isize, &isize) -> isize, fn() -> isize> {
        let n = init_vector.len();
        let mut lazy_segment_tree = new_range_assignment_update_and_range_minimum_query(n);
        for i in 0..n {
            lazy_segment_tree.range_update(i, i, init_vector[i]);
        }
        return lazy_segment_tree
    }

    // [5] 区間代入更新・区間最大値取得　の生成
    pub fn new_range_assignment_update_and_range_maximum_query(list_size: usize) -> LazySegmentTree<isize, fn(&isize, &isize) -> isize, fn() -> isize, isize, fn(&isize, &isize) -> isize, fn(&isize, &isize) -> isize, fn() -> isize> {
        /// 区間取得演算 (opだけど、更新操作ではなく取得なので注意)
        /// s0: 左の子data (たぶん)
        /// s1: 右の子data (たぶん)
        /// 返り値: 2子から得られたdata
        fn op(&s0: &isize, &s1:  &isize) -> isize {
            s0.max(s1)
        }
        /// Lazy -> Data の更新操作
        /// f: Lazyによる操作
        /// x: 更新前のdata
        /// 返り値: 更新後のdata
        fn mapping(&f: &isize ,&x: &isize) -> isize {
            if f == id() {
                return x
            }
            else {
                return f
            }
        }
        /// Lazy -> Lazy の更新操作
        /// f: 追加する最新の更新操作
        /// g: これまでの更新操作
        /// 返り値: g -> f の順にする操作。gとfの合成関数: f(g(x))
        fn composition(&f: &isize ,&g: &isize) -> isize {
            if f == id() {
                return g
            }
            else {
                return f
            }
        }
        /// 区間取得演算opの単位元 (dataの初期化に使われる)
        fn e() -> isize {
            let init_value: isize = std::isize::MIN;
            return init_value
        }
        /// 区間操作(更新)演算mappingにおける恒等写像 (lazyの初期化と、mappingとcompositionに使われる。区間代入更新のときは、正直なんでもいいと思う。)
        fn id() -> isize {
            let init_value: isize = std::isize::MIN; // 代入する値の範囲外に使われなければ、どんな値を入れてもいい。
            return init_value
        }
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

    // [5] 区間代入更新・区間最大値取得 を vector から生成
    pub fn new_range_assignment_update_and_range_maximum_query_from_vec(init_vector: &Vec<isize>) -> LazySegmentTree<isize, fn(&isize, &isize) -> isize, fn() -> isize, isize, fn(&isize, &isize) -> isize, fn(&isize, &isize) -> isize, fn() -> isize> {
        let n = init_vector.len();
        let mut lazy_segment_tree = new_range_assignment_update_and_range_maximum_query(n);
        for i in 0..n {
            lazy_segment_tree.range_update(i, i, init_vector[i]);
        }
        return lazy_segment_tree
    }

    // [6] 区間代入更新・区間和取得　の生成
    pub fn new_range_assignment_update_and_range_sum_query(list_size: usize) -> LazySegmentTree<SSum, fn(&SSum, &SSum) -> SSum, fn() -> SSum, isize, fn(&isize, &SSum) -> SSum, fn(&isize, &isize) -> isize, fn() -> isize> {
        /// 区間取得演算 (opだけど、更新操作ではなく取得なので注意)
        /// s0: 左の子data (たぶん)
        /// s1: 右の子data (たぶん)
        /// 返り値: 2子から得られたdata
        fn op(&s0: &SSum, &s1:  &SSum) -> SSum {
            return SSum {value: s0.value + s1.value, size: s0.size + s1.size}
        }
        /// Lazy -> Data の更新操作
        /// f: Lazyによる操作
        /// x: 更新前のdata
        /// 返り値: 更新後のdata
        fn mapping(&f: &isize ,&x: &SSum) -> SSum {
            if f == id() {
                return x
            }
            else {
                // x.value = f * x.size;
                // return x
                return SSum {value: f * x.size, size: x.size}
            }
        }
        /// Lazy -> Lazy の更新操作
        /// f: 追加する最新の更新操作
        /// g: これまでの更新操作
        /// 返り値: g -> f の順にする操作。gとfの合成関数: f(g(x))
        fn composition(&f: &isize ,&g: &isize) -> isize {
            if f == id() {
                return g
            }
            else {
                return f
            }
        }
        /// 区間取得演算opの単位元 (dataの初期化に使われる)
        fn e() -> SSum {
            let init_value: isize = 0;
            let init_size: isize = 0;
            return SSum {value: init_value, size: init_size}
        }
        /// 区間操作(更新)演算mappingにおける恒等写像 (lazyの初期化と、mappingとcompositionに使われる。区間代入更新のときは、範囲外であれば正直なんでもいいと思う。)
        fn id() -> isize {
            let init_value: isize = std::isize::MAX; // 代入更新する値の範囲外であれば、どんな値を入れてもいい。
            return init_value
        }
        let mut lazy_segment_tree = LazySegmentTree::new(
            list_size,
            e as fn() -> SSum, 
            op as fn(&SSum, &SSum) -> SSum, 
            mapping as fn(&isize, &SSum) -> SSum, 
            composition as fn(&isize, &isize) -> isize, 
            id as fn() -> isize
        );

        // 各ノードの、守備範囲の大きさ(tree.size)を適切に初期化
        init_node_size_for_range_sum_tree(&mut lazy_segment_tree);

        return lazy_segment_tree
    }

    // [6] 区間代入更新・区間和取得 を vector から生成
    pub fn new_range_assignment_update_and_range_sum_query_from_vec(init_vector: &Vec<isize>) -> LazySegmentTree<SSum, fn(&SSum, &SSum) -> SSum, fn() -> SSum, isize, fn(&isize, &SSum) -> SSum, fn(&isize, &isize) -> isize, fn() -> isize> {
        let n = init_vector.len();
        let mut lazy_segment_tree = new_range_assignment_update_and_range_sum_query(n);
        for i in 0..n {
            lazy_segment_tree.range_update(i, i, init_vector[i]);
        }
        return lazy_segment_tree
    }

    // 区間和取得用の遅延評価セグメント木の各ノードの、守備範囲の大きさ(tree.size)を適切に初期化する関数
    fn init_node_size_for_range_sum_tree(lazy_segment_tree: &mut LazySegmentTree<SSum, fn(&SSum, &SSum) -> SSum, fn() -> SSum, isize, fn(&isize, &SSum) -> SSum, fn(&isize, &isize) -> isize, fn() -> isize>) {
        // 葉のノードについてはtree.sizeを、元の配列部分だけ1にして、範囲外は0のままにする
        let first_list_index = lazy_segment_tree.tree_size / 2; // 木における配列先頭のindex
        for i in 0..lazy_segment_tree.list_size {
            lazy_segment_tree.tree[i + first_list_index].size = 1;
        }

        // 木の最下段から、根に向かって、親ノードの守備範囲の大きさを計算
        let mut current_stage_size = lazy_segment_tree.leaf_size;
        let mut current_stage_first_index = first_list_index;
        while current_stage_size != 1 {
            for i in 0..current_stage_size {
                let v = current_stage_first_index + i;
                // p := parent index
                let p = (v - 1) / 2;
                lazy_segment_tree.tree[p].size += lazy_segment_tree.tree[v].size;
            }
            current_stage_size /= 2;
            current_stage_first_index /= 2;
        }
    }

    /// 区間和取得用のdataノードの構造体
    #[derive(Clone, Debug, Copy)]
    pub struct SSum {
        pub value: isize,
        size: isize
    }
    
    // メソッドの実装
    impl<S, Op, E, F, Mapping, Composition, Id> LazySegmentTree<S, Op, E, F, Mapping, Composition, Id>
    where
        S: Clone + std::fmt::Debug,                         // セグメント木のノードに格納されたデータ型
        Op: Fn(&S, &S) -> S,                                // 区間取得をどのような演算で行うか
        E: Fn() -> S,                                       // 区間取得演算の単位元
        F: Clone + std::cmp::PartialEq + std::fmt::Debug,   // 遅延評価用の木のノードに格納されたデータ型
        Mapping: Fn(&F, &S) -> S,                           // lazy -> data への操作(更新)
        Composition: Fn(&F, &F) -> F,                       // 親のlazy -> 子のlazy　への操作(更新)
        Id: Fn() -> F                                       // 区間操作(更新)演算 mapping における恒等写像
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
                // mapping: Lazy node -> Data node の伝播
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
            // println!("start to update between {}-{} to {:?}", update_l, update_r, f);
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
        pub fn print_list(&self) {
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



/// Binary Indexed Tree (BIT)
/// 参考: https://algo-logic.info/binary-indexed-tree/
/// (1)構築: O(N)
/// (2)加算: O(logN): 数列Anのi番目の項にxを足す (区間加算じゃないので注意)
/// (3)区間和: O(logN): 数列Anの先頭からi番目までの項の和を求める (閉区間だからiも含めるので注意)
/// セグメント木より機能が限定的だが、実装が簡単 & 定数倍で高速 & 省メモリ
#[derive(Debug, Clone, PartialEq, Eq)]
struct BinaryIndexedTree<T> {
    n: isize,       // 配列の要素数(数列の要素数+1)
    bit: Vec<T>    // データの格納先(1-indexed)。初期値は0
    // 0 始まり(0-indexed) ではなく 1 から番号を始めている(1-indexed)
    // また半開区間ではなく閉区間で考える。
    // これは後で計算をする際に楽になるため。
}

impl<T: Default + Copy + std::ops::AddAssign + std::ops::SubAssign + std::fmt::Debug + std::ops::Sub<Output = T>> BinaryIndexedTree<T> {
    fn new(n: usize) -> Self {
        BinaryIndexedTree {
            n: (n + 1) as isize,
            bit: vec![T::default(); n + 1] // 例えばTがusizeならdefault()は0を返す
        }
    }

    // add のインターフェースとしてindexは元の数列のindexを採用している(内部で+1している)
    fn add(&mut self, index: usize, x: T) {
        let mut i = (index + 1) as isize;
        // let mut i = index as isize; // こっちを採用すると、インターフェースも半開区間にできる
        while i < self.n {
            self.bit[i as usize] += x;
            // println!("i={}, i={:05b} -i={:05b}", i, i, -i);

            // i の最後の1のビット = i & -i (∵負の数は「ビット反転+1」で表現される)
            // 例: 6 & -6 = (00…0110)_2 & (11…1010)_2 = (00…0010)_2
            i += (i & - i); // iにi の最後の1のビットを足すと、親のインデックスに移れる

            // Rustでは、負の数は2の補数表現で保持される。
            // 補数の定義: N進法において自然数xを表現するのに必要な最小の桁数をnとしたとき
            // xのNの補数はN^n - x となる
            // 例： 5(10進数)=101(2進数)の2の補数は、2^3-5(10進法) = 1000 - 101 (2進法) = 011(2進法)となる
            // 参考1: http://www.cc.kyoto-su.ac.jp/~kbys/kiso/number/negative.html
            // 参考2: http://www.f.waseda.jp/takezawa/math/joho/hosuu.pdf
            // もっと端的に言うと、
            // 0の定義を、そのデータ型のビット数の限界に1桁左から1を追加したものとする
            // 例: 3bitだけ使える場合、下記のように考える
            // -3: 0101
            // -2: 0110
            // -1: 0111
            //  0: 1000 <- 3bitの0の定義
            //  1: 0001
            //  2: 0010
            //  3: 0011
            // また、isizeの場合、-1は
            // 1111111111
            // 1111111111
            // 1111111111
            // 1111111111
            // 1111111111
            // 1111111111
            // 1111 となる (1が64個)。
        }
    }

    // Tが非負整数型(usizeなど)のときに、除算を行う
    fn subtract(&mut self, index: usize, x: T) {
        let mut i = (index + 1) as isize;
        // let mut i = index as isize; // こっちを採用すると、インターフェースも半開区間にできる
        while i < self.n {
            self.bit[i as usize]  -= x;
            i += (i & - i); // iにi の最後の1のビットを足すと、親のインデックスに移れる
        }
    }

    // a_1 + a_2 + ... + a_i を計算する (sumのインターフェースは半開区間ではなく閉区間を採用。a[index]は足される)
    fn sum(&self, index: usize) -> T {
        let mut i = (index + 1) as isize;
        // let mut i = index as isize; // こっちを採用すると、インターフェースも開区間にできる
        let mut sum = T::default(); // 例えばTがusizeならdefault()は0を返す
        while i > 0 {
            // println!("i={}, sum={:?}", i, sum);
            sum += self.bit[i as usize];
            // i の最後の1のビット = i & -i (∵負の数は「ビット反転+1」で表現される)
            // 例: 6 & -6 = (00…0110)_2 & (11…1010)_2 = (00…0010)_2
            i -= (i & - i); // iにi の最後の1のビットを引くと、1個左上のノードのインデックスに移れる
            // println!("i={}, sum={:?}", i, sum);
            // println!("==== ==== ==== ====");
        }
        return sum
    }
    // 閉区間[left, right]を取得する
    fn sum_range(&self, left: usize, right: usize) -> T {
        let right_sum: T = self.sum(right);
        let left_sum: T = match left == 0 {
            true => Default::default(), // 0のこと
            false => self.sum(left-1)
        };
        let range_sum: T = right_sum - left_sum;
        return range_sum
    }
    // index番目の値を取得する (sum()は累積和を取得するメソッド)
    fn get_element(&self, index: usize) -> T {
        let element = match index == 0 {
            true => self.sum(index),
            false => self.sum(index) - self.sum(index - 1)
        };
        return element
    }
    fn print_all_cum(&self) {
        // デバッグ用に、各インデックスにおける、累積和を標準出力に print
        print!("bit = ");
        for i in 0..self.n-1 {
            let sum_i = self.sum(i as usize);
            print!("{:?} ", sum_i);
        }
        println!("");
    }
}

