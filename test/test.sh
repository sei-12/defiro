#!/bin/bash

cargo build && \
cargo test && \
cat ./test/test_color.txt | target/debug/defiro
