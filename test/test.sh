#!/bin/bash

cargo build && cat ./test/test_color.txt | target/debug/defiro
