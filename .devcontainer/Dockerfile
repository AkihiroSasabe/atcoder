# 参考
# https://github.com/yiwiy9/atcoder-rust-devcontainer/blob/main/.devcontainer/Dockerfile
# https://zenn.dev/codemountains/articles/5995bc3e6b3aa3
# https://code.visualstudio.com/docs/devcontainers/tutorial
# https://github.com/rust-web-app-book/rusty-book-manager/blob/main/Dockerfile
# https://hub.docker.com/_/rust

# bullseye, slim-bookworm, 
# bookworm: もっとも標準的。aptが使える・互換性が広い。サイズは“slim”より大きめ。Debianの現行安定版系列。
# bookworm-slim: 余計なパッケージを削った軽量版。ビルド時に必要なツールは適宜 apt-get で追加。実運用では「まずこれ」で良いことが多い。
# bullseye: 旧安定（oldstable）。長期メンテ環境や古いGLIBC互換が必要なケースで残っている。新規は基本 bookworm 推奨。
# alpine: さらに小さいが、musl 由来の互換性差に注意（glibc想定バイナリやネイティブ依存が多いとハマりやすい）。apk使用。サイズ最小を狙う・完全静的リンクなどの玄人向け。

# FROM rust:1.70-bullseye
# FROM mcr.microsoft.com/vscode/devcontainers/base:0-bullseye
# rust:1.78-slim-bookworm
FROM mcr.microsoft.com/devcontainers/rust:latest
ENV TZ=Asia/Tokyo
# Z=Asia/Tokyo を入れただけでは一部コマンドがまだUTC表示のことがある。厳密にローカルタイムを揃えたいなら以下に置き換える。
# RUN ln -snf /usr/share/zoneinfo/Asia/Tokyo /etc/localtime && echo "Asia/Tokyo" > /etc/timezone


# 考察で使う matplotlib 用の python 仮想環境を作成
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends \
        python3.13-venv python3-pip \
    && rm -rf /var/lib/apt/lists/*

RUN python3 -m venv /opt/pyenv \
    && /opt/pyenv/bin/pip install --upgrade pip wheel setuptools \
    && /opt/pyenv/bin/pip install matplotlib

ENV PATH="/opt/pyenv/bin:${PATH}"


# RUN pip3 install matplotlib
# RUN apt-get install -y python3.13-venv python3-pip
# RUN python3 -m venv .venv
# RUN . .venv/bin/activate && pip install --upgrade pip wheel setuptools && pip install matplotlib
# # 仮想環境の有効化
# RUN source .venv/bin/activate



# some more ls aliases を追加 (全ユーザーに適用)
# -A: Almost all. 隠しファイルも表示。-a だと . と .. も出てきて一覧が煩雑になりがちなので、普段使いは -A が実用的
# -l: long format. 縦並びの詳細形式で表示。
# -F: classify. 種別を表す記号を末尾に付ける：ディレクトリ/、実行可能*、シンボリックリンク@、ソケット=, FIFO| など。
# --group-directories-first: ディレクトリを先に、その後に通常ファイルを並べる
# --color=auto: 出力先が端末のときだけ色付け（パイプやリダイレクト時は無色）。ログや grep へのパイプで制御文字が混ざらないので安全。
# -h: human-readable. サイズを 1.2K / 3.4M / 5.6G のように読みやすく

RUN echo "alias ll='ls -AlFh --group-directories-first --color=auto'" >> /etc/bash.bashrc \
    && echo "alias la='ls -AlFh --group-directories-first --color=auto'" >> /etc/bash.bashrc \
    && echo "alias l='ls -AlFh --group-directories-first --color=auto'" >> /etc/bash.bashrc

