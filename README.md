# Atcoder Begginer Contest を Rust で解答したものを保存
Atcoder Begginer Contest を Rust で解答したものを保存している。  

## 新規コンテストフォルダの作成方法
```sh
$ cargo run --bin make_contest_folder <コンテスト番号>

# 例: abc777 を解くなら以下を実行して、 src/abc777/ を作成する
$ cargo run --bin make_contest_folder 777
```

## Debug方法
[src/debug.sh](src/debug.sh) を参照  

## スニペット
VSCode用のスニペットを用意している。 [.vscode/competitive_programming.code-snippets](.vscode/competitive_programming.code-snippets) を参照。  

もしこのリポジトリ内ではなく、グローバルにスニペットを設定する場合は、VSCodeで、`Ctrl` + `Shift` + `P` でコマンドパレットを開き、以下のように検索  
> `Snippets: Configure Snippets`

し、[.vscode/competitive_programming.code-snippets](.vscode/competitive_programming.code-snippets) の内容をコピペする。  
例えばWindowsでVSCodeなら、以下にスニペットが保存されている。  
`/C:/Users/<ユーザー名>/AppData/Roaming/Code/User/snippets/competitive_programming.code-snippets`

参考記事:
- [VS Codeのユーザースニペット機能の使い方メモ](https://qiita.com/12345/items/97ba616d530b4f692c97)  
- [snippet-generator](https://snippet-generator.app/?description=&tabtrigger=&snippet=&mode=vscode)  


## Rust-Analyzer
WSLなら、グローバルな設定は重くなるので、やらないほうがいい。  
(.vscode/settings.json)[.vscode/settings.json] で設定するのが良い。
また devcontainer の場合は、`bind`でmountするのも遅くなるので、  
`rust-analyzer.cargo.targetDir`をプロジェクトの外に指定した方が良い。   

// 以下、悪い例
/C:/Users/<ユーザー名>/AppData/Roaming/Code/User/settings.json
```json
{
    "terminal.integrated.enableMultiLinePasteWarning": false,
    "rust-analyzer.linkedProjects": [
        "/home/akki/projects/atcoder/Cargo.toml",
        "/home/akki/projects/kyopro_circle/Cargo.toml",
        "/home/akki/projects/typical90/Cargo.toml",
        "/home/akki/projects/wasm_rust/walk-the-dog/Cargo.toml"
    ],
    "workbench.colorTheme": "Default Light+",
    "git.openRepositoryInParentFolders": "never",
    "workbench.colorCustomizations": {},
    "editor.tabCompletion": "on",
    "github.copilot.enable": {
        "*": false,
        "plaintext": false,
        "markdown": false,
        "scminput": false
    },
    "github.copilot.nextEditSuggestions.enabled": true
}
```