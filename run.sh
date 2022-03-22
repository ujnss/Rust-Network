#!/bin/bash

prog=${1:-"ex01"}

if [ ! -f "examples/${prog}.rs" ]; then
  echo "examples/${prog}.rs not exist!"
  exit 1
fi

echo "run examples/${prog}.rs ..."

mkdir -p log

cargo run --example ${prog} -- --party_id 0 >log/${prog}-0.log 2>&1 &
cargo run --example ${prog} -- --party_id 1 >log/${prog}-1.log 2>&1 &
cargo run --example ${prog} -- --party_id 2 >log/${prog}-2.log 2>&1
wait

ps -ef | grep -E "${prog}" | grep party_id | grep -v grep | awk '{print $2}' | xargs kill -9 >/dev/null 2>&1
ps -ef | grep -E "ex01|ex02" | grep party_id | grep -v grep | awk '{print $2}' | xargs kill -9 >/dev/null 2>&1

echo "DONE"
