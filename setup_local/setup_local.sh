#!/bin/bash
# コンパイル時にはgccリンカをインストールしておく必要がある
sudo apt-get update
sudo apt-get install -y build-essential
# Rustインストール
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh << EOF
1
EOF
# Rustインストール確認
rustc --version
cargo --version
# vscodeで補間等するために必要な静的検査ツールをインストール
rustup component add rust-src
rustup component add rust-analysis
rustup component add rls