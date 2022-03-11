#!/bin/bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh << EOF
1
EOF
rustc --version
cargo --version