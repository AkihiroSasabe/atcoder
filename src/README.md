このリポジトリでは、AtCoder Beginer Contest(ABC)の問題について、私がRustで解答したコードを管理しています。  

## (1)解答用のファイルを作って実行する方法 
```sh
cargo run --bin make_contest_folder <ABCの回数名>
```

例: abc44を解くなら
```sh
cargo run --bin make_contest_folder 44
```

例: abc4を解くなら
```sh
cargo run --bin make_contest_folder 4
```

## (2)テストケースを自動で作ってデバッグする方法  
### (2-1)`src/debug.sh`と`src/make_testcase.py`をデバッグしたい回のディレクトリにコピーして置く。
```sh
cp ../src/debug.sh debug.sh
cp ../src/make_testcase.py make_testcase.py
```

### (2-2)正解コードを、その回の`h.rs`に記入する。(愚直解か、AC済みの他人のコードのコピーなど)
### (2-3)デバッグしたい問題のコードと、`h.rs`のコードを実行して、解が異なるまで繰り返す。

```sh
bash debug.sh <デバッグしたい問題名>
```

### (2-4)生成された`temp.txt`に、両者の解が異なるケースが保存されるので、それを使って頑張ってデバッグする


## (3) スニペットの準備
VSCodeで、Ctrl + Shift + P で以下のように検索  
> `Snippets: Configure Snippets`

`./src/snippets`の内容をコピペする。 