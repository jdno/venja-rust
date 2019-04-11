#!/bin/bash

version=0.17.1
platform=x86_64-unknown-linux-musl

curl https://github.com/sagiegurari/cargo-make/releases/download/${version}/cargo-make-v${version}-${platform}.zip -sSfL -o /tmp/cargo-make.zip
unzip /tmp/cargo-make.zip
mv cargo-make-*/* $HOME/.cargo/bin;

echo "##vso[task.setvariable variable=PATH;]$PATH:$HOME/.cargo/bin"
