#!/bin/bash

dir_path=$(dirname $0)

cargo build && \
cargo test && \

cmd=$dir_path/../target/debug/defiro
dirs=`find $dir_path -type f -name *.test`

for test_file in $dirs;
do
    echo "-------------------------------------------------------------"
    $cmd -- $test_file;
done

