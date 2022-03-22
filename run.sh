#!/bin/bash

mkdir -p log

cargo run --example ex01 -- --party_id 0 >log/0.log 2>&1 &
cargo run --example ex01 -- --party_id 1 >log/1.log 2>&1 &
cargo run --example ex01 -- --party_id 2 >log/2.log 2>&1
wait

ps -ef | grep -E "ex01" | grep party_id | grep -v grep | awk '{print $2}' | xargs kill -9 >/dev/null 2>&1

echo "DONE"
